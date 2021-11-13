use std::{io, ptr};
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};
use num_traits::FromPrimitive;
use std::convert::TryInto;
use colored::*;

mod op_codes;
use op_codes::*;
mod il_type;
use il_type::*;
mod image_reader;
use image_reader::*;
mod metadata;
use metadata::*;
mod signature;
pub use signature::*;
mod type_def;
use type_def::*;
mod type_ref;
use type_ref::*;
mod member_ref;
use member_ref::*;
mod assembly_name;
use assembly_name::*;
mod assembly_ref;
use assembly_ref::*;
mod method;
use method::*;
mod param;
use param::*;
mod object;
use object::*;

pub struct Assembly {
    pub assembly_path: String,
    pub assembly_name: AssemblyName,

    pub image: Vec<u8>,    // 映像，一次性读取
    pub pe: PE,
    pub metadata: Metadata,

    pub type_refs: Vec<TypeRef>,                //  <token(0x01000001...), TypeRef>
    pub type_defs: HashMap<String, TypeDef>,    //  <namespace.classname, TypeDef>
    pub methods: Vec<Method>,                   //  <token(0x06000001...), Method>
    pub params: Vec<Param>,                     //  <token(0x08000001...), Param>
    pub member_refs: Vec<MemberRef>,            //  <token(0x0A000001...), MemberRef>
    pub assembly_refs: Vec<AssemblyRef>,        //  <token(0x23000001...), AssemblyRef>
}

impl Assembly {
    pub fn new(assembly_path: &String) -> io::Result<Assembly> {
        let mut file = File::open(assembly_path)?;
        let metadata = file.metadata().expect("unable to read metadata");
        let mut image = vec![0; metadata.len() as usize];
        file.read(&mut image).expect("Error reading assembly, overflow.");
        let mut reader = ImageReader::new(&image);
        let pe = PE::new(&mut reader)?;
        let metadata = Metadata::new(&pe, &mut reader)?;

        let type_refs = TypeRef::read_type_refs(&metadata)?;
        let type_defs = TypeDef::read_type_defs(&metadata)?;

        // 这是为了寻找方法的类型定义
        // 例如类型定义中的Methods的RidList按顺序是这样，1->1, 1->4, 4->4, 4->5
        // 那么这个数组就存放的是[1, 1, 4, 4]
        // read_methods的时候，假设一个method的Rid是3，那么就能知道4是第一个比3大的，是第3个方法的Method，即0x06000003
        let mut method_to_type_map = Vec::new();
        for type_def in type_defs.values() {
            method_to_type_map.push(type_def.method_list.start_rid);
        }
        method_to_type_map.sort();

        let methods = Method::read_methods(&pe, &metadata, method_to_type_map, &mut reader)?;
        let params = Param::read_params(&metadata)?;

        let member_refs = MemberRef::read_member_refs(&metadata)?;
        let assembly_refs = AssemblyRef::read_assembly_refs(&metadata)?;
        
        let assembly_table = &metadata.table_stream.md_tables[0x20];
        let major_version = assembly_table.columns[1].get_cell_u16(0);
        let minor_version = assembly_table.columns[2].get_cell_u16(0);
        let build_number = assembly_table.columns[3].get_cell_u16(0);
        let revision_number = assembly_table.columns[4].get_cell_u16(0);
        let flags = assembly_table.columns[5].get_cell_u32(0);
        let public_key_token = metadata.blob_stream.read(assembly_table.columns[6].get_cell_u16(0) as u32)?;
        let name = metadata.strings_stream.get_string_clone(assembly_table.columns[7].get_cell_u16_or_u32(0))?;
        
        println!("Assembly loaded: {:?}", assembly_path);

        Ok(Assembly {
            assembly_path: assembly_path.to_string(),
            assembly_name: AssemblyName {
                major_version,
                minor_version,
                build_number,
                revision_number,
                flags,
                public_key_token,
                name,
            },

            image,
            pe,
            metadata,

            type_refs,
            type_defs,
            methods,
            params,
            member_refs,
            assembly_refs,
        })
    }

    pub fn load(assembly_name: &AssemblyName) -> io::Result<Assembly> {
        const NET5_PATH: &'static str = r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\5.0.11\";
        let assembly_path = format!("{}{}.dll", NET5_PATH, assembly_name.name);
        let assembly = Assembly::new(&assembly_path)?;
        if assembly.assembly_name != *assembly_name {
            return Err(io::Error::new(io::ErrorKind::Other, "Assembly name not match."));
        }
        Ok(assembly)
    }
}

pub struct Interpreter {
    assemblies: Vec<Assembly>,              // index0放入口Assembly，加载的外部Assembly依次往后放
    assembly_map: HashMap<String, usize>,   // 根据AssemblyName定位Assemblies的index
    call_stack: Vec<(usize, u32)>,          // 调用堆栈，记录当前assembly的index和method_token

    assembly_index: usize,
    stack: VecDeque<ILType>,
    objects: Vec<Object>,
    strings: Vec<String>,
}

impl Interpreter {
    pub fn new(assembly_path: String) -> io::Result<Interpreter> {
        let mut assemblies = Vec::new();

        let assembly = Assembly::new(&assembly_path)?;
        println!("{}", "Assembly Methods: ".green());
        for method in assembly.methods.iter() {
            println!("{:?}", method);
        }
        assemblies.push(assembly);

        Ok(Interpreter {
            assemblies,
            assembly_map: HashMap::new(),
            call_stack: Vec::new(),

            assembly_index: 0,
            stack: VecDeque::new(),
            objects: Vec::new(),
            strings: Vec::new(),
        })
    }

    pub fn run(&mut self) {
        println!("\nstart run:\n");
        self.il_call(0x06000001);
    }

    fn load_assembly(&mut self, assembly_name: &AssemblyName) -> usize {
        self.assemblies.push(Assembly::load(assembly_name).unwrap());
        let index = self.assemblies.len() - 1;
        self.assembly_map.insert(assembly_name.name.clone(), index);
        index
    }

    fn il_box_obj(&mut self, type_token: u32, value: ILType) {
        self.objects.push(Object::new_box(type_token, value));
        self.stack.push_back(ILType::Ref(ILRefType::Object(self.objects.len() - 1)));
    }

    fn il_new_obj(&mut self, type_token: u32) {
        self.objects.push(Object::new(type_token));
        self.stack.push_back(ILType::Ref(ILRefType::Object(self.objects.len() - 1)));
    }

    fn il_new_string(&mut self, string: String) {
        self.strings.push(string);
        self.stack.push_back(ILType::Ref(ILRefType::String(self.strings.len() - 1)));
    }

    // 加载其他Assembly中的方法
    fn load_dest_method(&mut self, method_token: u32) -> &Method {
        let assembly = &self.assemblies[self.assembly_index];
        let member_ref = &assembly.member_refs[(method_token & 0x00FFFFFF) as usize - 1];
        let class = member_ref.class;
        if (class >> 24) == 0x01 {  // TypeRef
            let type_ref = self.assemblies[self.assembly_index].type_refs[(class & 0x00FFFFFF) as usize - 1].clone();
            let resolution_scope = assembly.metadata.resolve_resolution_scope(type_ref.resolution_scope as u32).unwrap();
            if (resolution_scope >> 24) == 0x23 {  // AssemblyRef
                let assembly_name = &assembly.assembly_refs[(resolution_scope & 0x00FFFFFF) as usize - 1].assembly_name.clone();
                if self.assembly_map.get(&assembly_name.name).is_none() {  // 如果引用的Assembly没有加载，那就加载
                    self.assembly_index = self.load_assembly(&assembly_name);
                }

                let assembly = &self.assemblies[self.assembly_index];
                let dest_type = assembly.type_defs.get(&type_ref.full_name).unwrap();
                
            }
        }
        panic!("Invalid method_token")
    }

    fn il_call(&mut self, method_token: u32) {
        let assembly_index = self.assembly_index;
        let param_count;
        let mut rip;
        {
            let assembly = &self.assemblies[assembly_index];
            let method = match method_token >> 24 {
                0x06 => {  // 表示是当前Assembly内的方法
                    self.call_stack.push((self.assembly_index, method_token));
                    &assembly.methods[(method_token & 0x00FFFFFF) as usize - 1]
                },
                0x0A => {  // 需要先找到MemberRef，再找到TypeRef，最后定位到AssemblyRef
                    self.load_dest_method(method_token)
                },
                _ => panic!("Invalid method_token")
            };
            println!("call: {:?}", method);
            if method.is_static() {
                param_count = method.param_list.count as usize;
            } else {
                param_count = method.param_list.count as usize + 1;  // 实例方法第一个参数是this
            }
            rip = method.code_position;  // 当前函数指针
        }

        let mut params = VecDeque::new();
        for _ in 0..param_count {
            params.push_front(self.stack.pop_back().unwrap());  // 逆向出栈，获取参数
        }

        let mut locals = [ILType::Ref(ILRefType::Null); 8];  // TODO
        loop {
            let op = FromPrimitive::from_u8(self.assemblies[assembly_index].image[rip]);
            rip += 1;
            match op {
                Some(OpCode::Nop) => {},
                Some(OpCode::Break) => {
                    println!("break");
                },
                Some(OpCode::Ldarg0) => {
                    self.stack.push_back(params[0]);
                },
                Some(OpCode::Ldarg1) => {
                    self.stack.push_back(params[1]);
                },
                Some(OpCode::Ldarg2) => {
                    self.stack.push_back(params[2]);
                },
                Some(OpCode::Ldarg3) => {
                    self.stack.push_back(params[3]);
                },
                Some(OpCode::Ldloc0) => {
                    self.stack.push_back(locals[0]);
                },
                Some(OpCode::Ldloc1) => {
                    self.stack.push_back(locals[1]);
                },
                Some(OpCode::Ldloc2) => {
                    self.stack.push_back(locals[2]);
                },
                Some(OpCode::Ldloc3) => {
                    self.stack.push_back(locals[3]);
                },
                Some(OpCode::Stloc0) => {
                    locals[0] = self.stack.pop_back().unwrap();
                },
                Some(OpCode::Stloc1) => {
                    locals[1] = self.stack.pop_back().unwrap();
                },
                Some(OpCode::Stloc2) => {
                    locals[2] = self.stack.pop_back().unwrap();
                },
                Some(OpCode::Stloc3) => {
                    locals[3] = self.stack.pop_back().unwrap();
                },
                Some(OpCode::Ldargs) => {
                    let index = self.assemblies[assembly_index].image[rip];
                    rip += 1;
                    self.stack.push_back(params[index as usize]);
                },
                Some(OpCode::Ldargas) => {
                    let index = self.assemblies[assembly_index].image[rip];
                    rip += 1;
                    self.stack.push_back(ILType::Ptr(ptr::addr_of_mut!(params[index as usize])));
                },
                Some(OpCode::Stargs) => {
                    let index = self.assemblies[assembly_index].image[rip];
                    rip += 1;
                    params[index as usize] = self.stack.pop_back().unwrap();
                },
                Some(OpCode::Ldlocs) => {
                    let index = self.assemblies[assembly_index].image[rip];
                    rip += 1;
                    self.stack.push_back(locals[index as usize]);
                },
                Some(OpCode::Ldlocas) => {
                    let index = self.assemblies[assembly_index].image[rip];
                    rip += 1;
                    self.stack.push_back(ILType::Ptr(ptr::addr_of_mut!(locals[index as usize])));
                },
                Some(OpCode::Stlocs) => {
                    let index = self.assemblies[assembly_index].image[rip];
                    rip += 1;
                    locals[index as usize] = self.stack.pop_back().unwrap();
                },
                Some(OpCode::Ldnull) => {
                    self.stack.push_back(ILType::Ref(ILRefType::Null));
                },
                Some(OpCode::Ldci4m1) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(-1)));
                },
                Some(OpCode::Ldci40) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(0)));
                },
                Some(OpCode::Ldci41) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(1)));
                },
                Some(OpCode::Ldci42) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(2)));
                },
                Some(OpCode::Ldci43) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(3)));
                },
                Some(OpCode::Ldci44) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(4)));
                },
                Some(OpCode::Ldci45) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(5)));
                },
                Some(OpCode::Ldci46) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(6)));
                },
                Some(OpCode::Ldci47) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(7)));
                },
                Some(OpCode::Ldci48) => {
                    self.stack.push_back(ILType::Val(ILValType::Int32(8)));
                },
                Some(OpCode::Ldci4s) => {
                    let val = self.assemblies[assembly_index].image[rip];
                    rip += 1;
                    self.stack.push_back(ILType::Val(ILValType::Byte(val)));
                },
                Some(OpCode::Ldci4) => {
                    let val = u32::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    self.stack.push_back(ILType::Val(ILValType::UInt32(val)));
                },
                Some(OpCode::Ldci8) => {
                    let val = u64::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 8].try_into().unwrap());
                    rip += 8;
                    self.stack.push_back(ILType::Val(ILValType::UInt64(val)));
                },
                Some(OpCode::Ldcr4) => {
                    let val = f32::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    self.stack.push_back(ILType::Val(ILValType::Single(val)));
                },
                Some(OpCode::Ldcr8) => {
                    let val = f64::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 8].try_into().unwrap());
                    rip += 8;
                    self.stack.push_back(ILType::Val(ILValType::Double(val)));
                },
                Some(OpCode::Dup) => {
                    self.stack.push_back(self.stack.back().unwrap().clone());
                },
                Some(OpCode::Pop) => {
                    self.stack.pop_back();
                },
                Some(OpCode::Jmp) => {
                    assert_eq!(self.assemblies[assembly_index].params.len(), 0);
                    let token = u32::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 4].try_into().unwrap());
                    self.il_call(token);
                    return;
                },
                Some(OpCode::Call) => {
                    let token = u32::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    self.il_call(token);
                },
                Some(OpCode::Calli) => {
                    todo!();
                },
                Some(OpCode::Ret) => {
                    return;
                },
                Some(OpCode::Brs) => {
                    let target = self.assemblies[assembly_index].image[rip];
                    rip += 1 + target as usize;
                },
                // 忽略一些
                Some(OpCode::Add) => {
                    let a = self.stack.pop_back().unwrap();
                    let b = self.stack.pop_back().unwrap();
                    self.stack.push_back(a + b);
                },
                Some(OpCode::Sub) => {
                    let a = self.stack.pop_back().unwrap();
                    let b = self.stack.pop_back().unwrap();
                    self.stack.push_back(a - b);
                },
                Some(OpCode::Mul) => {
                    // let a = self.stack.pop_back().unwrap();
                    // let b = self.stack.pop_back().unwrap();
                    // self.stack.push_back(a * b);
                    todo!();
                },
                Some(OpCode::Div) => {
                    // let a = self.stack.pop_back().unwrap();
                    // let b = self.stack.pop_back().unwrap();
                    // self.stack.push_back(a / b);
                    todo!();
                },
                Some(OpCode::Divun) => {
                    todo!();
                },
                Some(OpCode::Rem) => {
                    todo!();
                },
                Some(OpCode::Remun) => {
                    todo!();
                },
                Some(OpCode::And) => {
                    todo!();
                },
                Some(OpCode::Or) => {
                    todo!();
                },
                Some(OpCode::Xor) => {
                    todo!();
                },
                Some(OpCode::Shl) => {
                    todo!();
                },
                Some(OpCode::Shr) => {
                    todo!();
                },
                Some(OpCode::Shrun) => {
                    todo!();
                },
                Some(OpCode::Neg) => {
                    todo!();
                },
                Some(OpCode::Not) => {
                    todo!();
                },
                // 忽略一些
                Some(OpCode::Ldstr) => {
                    let token = u32::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    let str = self.assemblies[assembly_index].metadata.get_us_string(token).unwrap();
                    self.il_new_string(str);
                },
                Some(OpCode::Newobj) => {
                    let token = u32::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 4].try_into().unwrap());  // .ctor方法的token
                    rip += 4;
                    let type_token = self.assemblies[assembly_index].methods[token as usize - 0x06000001].owner_type + 0x02000001;
                    self.il_new_obj(type_token);  // 根据.ctor找到类，new出来推送到栈上，作为.ctor的this
                    self.il_call(token);
                },
                // 忽略一些
                Some(OpCode::Unbox) => {
                    todo!();
                },
                Some(OpCode::Throw) => {
                    todo!();
                },
                Some(OpCode::Ldfld) => {
                    todo!();
                },
                Some(OpCode::Ldflda) => {
                    todo!();
                },
                Some(OpCode::Stfld) => {
                    todo!();
                },
                Some(OpCode::Ldsfld) => {
                    todo!();
                },
                Some(OpCode::Ldsflda) => {
                    todo!();
                },
                Some(OpCode::Stsfld) => {
                    todo!();
                },
                Some(OpCode::Stobj) => {
                    todo!();
                },
                Some(OpCode::Convovfi1un) => {
                    todo!();
                },
                Some(OpCode::Convovfi2un) => {
                    todo!();
                },
                Some(OpCode::Convovfi4un) => {
                    todo!();
                },
                Some(OpCode::Convovfi8un) => {
                    todo!();
                },
                Some(OpCode::Convovfu1un) => {
                    todo!();
                },
                Some(OpCode::Convovfu2un) => {
                    todo!();
                },
                Some(OpCode::Convovfu4un) => {
                    todo!();
                },
                Some(OpCode::Convovfu8un) => {
                    todo!();
                },
                Some(OpCode::Convovfiun) => {
                    todo!();
                },
                Some(OpCode::Convovfuun) => {
                    todo!();
                },
                Some(OpCode::Box) => {
                    let token = u32::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    let value = self.stack.pop_back().unwrap();
                    self.il_box_obj(token, value);
                },
                Some(OpCode::Newarr) => {
                    todo!();
                },
                Some(OpCode::Ldlen) => {
                    todo!();
                },
                // 忽略一些
                Some(OpCode::Unboxany) => {
                    let token = u32::from_le_bytes(self.assemblies[assembly_index].image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    let boxed = self.stack.pop_back().unwrap();
                    let ref_obj = &self.objects[boxed.get_ref()];
                    if ref_obj.get_type() != token {
                        panic!("unboxany: type mismatch");
                    }
                    self.stack.push_back(ref_obj.box_value.unwrap());
                },
                // 忽略一些
                _ => {
                    println!("Unknown OpCode: 0x{:02X}", self.assemblies[assembly_index].image[rip - 1]);
                    return
                }
            }
        }
    }

    fn convert_to_string(&self, val: &ILType) -> String {
        match val {
            ILType::Val(v) => {
                v.to_string()
            },
            ILType::Ref(ILRefType::String(s)) => {
                self.strings[*s].to_string()
            },
            ILType::Ref(ILRefType::Object(o)) => {
                self.objects[*o].to_string()
            },
            ILType::Ref(ILRefType::Null) => {
                "null".to_string()
            },
            ILType::Ptr(p) => {
                self.convert_to_string(unsafe { &**p })
            },
        }
    }

}