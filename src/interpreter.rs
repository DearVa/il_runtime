use std::rc::Rc;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};
use num_traits::FromPrimitive;
use colored::*;

mod op_codes;
use op_codes::*;
mod il_type;
use il_type::*;
mod data_reader;
use data_reader::*;
mod metadata;
use metadata::*;
mod assembly_name;
use assembly_name::*;

mod type_ref;
use type_ref::*;
mod type_def;
use type_def::*;
mod field;
use field::*;
mod method;
use method::*;
mod param;
use param::*;
mod member_ref;
use member_ref::*;
mod standalone_sig;
use standalone_sig::*;
mod type_spec;
use type_spec::*;
mod assembly_ref;
use assembly_ref::*;
mod exported_type;
use exported_type::*;
mod method_spec;
use method_spec::*;

mod type_sig;
use type_sig::*;
mod calling_convention_sig;
use calling_convention_sig::*;
mod object;
use object::*;

use crate::hash_vec::HashVec;

pub struct Assembly {
    pub assembly_path: String,
    pub assembly_name: AssemblyName,
    pub is_cor_lib: bool,

    pub reader: DataReader,
    pub pe: PE,
    pub metadata: Metadata,

    pub type_refs: HashVec<String, TypeRef>,    //  (0x01000001...),
    pub type_defs: HashVec<String, TypeDef>,    //  (0x02000001...), <namespace.classname, TypeDef>
    pub fields: Vec<Field>,                     //  (0x04000001...), Field
    pub methods: Vec<Method>,                   //  (0x06000001...), Method
    pub params: Vec<Param>,                     //  (0x08000001...), Param
    pub member_refs: Vec<MemberRef>,            //  (0x0A000001...), MemberRef
    pub standalone_sigs: Vec<StandaloneSig>,    //  (0x11000001...), StandaloneSig
    pub type_specs: Vec<TypeSpec>,              //  (0x1B000001...), TypeSpec 
    pub assembly_refs: Vec<AssemblyRef>,        //  (0x23000001...), AssemblyRef
    pub exported_types: HashVec<String, ExportedType>, // (0x27000001...), ExportedType
    pub method_specs: Vec<MethodSpec>,          // (0x2B000001...), MethodSpec
}

impl Assembly {
    const NET5_PATH: &'static str = r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\5.0.11\";

    pub fn new(assembly_path: &String, is_cor_lib: bool) -> io::Result<Assembly> {
        let mut file = File::open(assembly_path)?;
        let metadata = file.metadata().expect("unable to read metadata");
        let mut image = vec![0; metadata.len() as usize];
        file.read(&mut image).expect("Error reading assembly, overflow.");
        let mut reader = DataReader::new(image);
        let pe = PE::new(&mut reader)?;
        let metadata = Metadata::new(&pe, &mut reader)?;

        let type_refs = TypeRef::read_type_refs(&metadata)?;
        let type_defs = TypeDef::read_type_defs(&metadata)?;
        // 读取完之后ref和def之后，更新def中的field_list

        let field_to_type_map = type_defs.vec().map(|t| t.field_list.start_rid).collect::<Vec<u32>>();
        let fields = Field::read_fields(&metadata, field_to_type_map)?;
        // 这是为了寻找方法的类型定义
        // 例如类型定义中的Methods的RidList按顺序是这样，1->1, 1->4, 4->4, 4->5
        // 那么这个数组就存放的是[1, 1, 4, 4]
        // read_methods的时候，假设一个method的Rid是3，那么就能知道4是第一个比3大的，是第3个方法的Method，即0x06000003
        let method_to_type_map = type_defs.vec().map(|t| t.method_list.start_rid).collect::<Vec<u32>>();
        let methods = Method::read_methods(&pe, &metadata, method_to_type_map, &mut reader)?;
        let params = Param::read_params(&metadata)?;

        let member_refs = MemberRef::read_member_refs(&metadata)?;
        let standalone_sigs = StandaloneSig::read_standalone_sigs(&metadata)?;
        let type_specs = TypeSpec::read_type_specs(&metadata)?;
        let assembly_refs = AssemblyRef::read_assembly_refs(&metadata)?;
        let exported_types = ExportedType::read_exported_types(&metadata)?;
        let method_specs = MethodSpec::read_method_specs(&metadata)?;

        let assembly_table = &metadata.table_stream.md_tables[0x20];
        let major_version = assembly_table.columns[1].get_cell_u16(0);
        let minor_version = assembly_table.columns[2].get_cell_u16(0);
        let build_number = assembly_table.columns[3].get_cell_u16(0);
        let revision_number = assembly_table.columns[4].get_cell_u16(0);
        let flags = assembly_table.columns[5].get_cell_u32(0);
        let public_key_token = metadata.blob_stream.read(assembly_table.columns[6].get_cell_u16_or_u32(0))?;
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
            is_cor_lib,

            reader,
            pe,
            metadata,

            type_refs,
            type_defs,
            fields,
            methods,
            params,
            member_refs,
            standalone_sigs,
            type_specs,
            assembly_refs,
            exported_types,
            method_specs,
        })
    }

    pub fn load(assembly_name: &AssemblyName) -> io::Result<Assembly> {
        let assembly_path = format!("{}{}.dll", Self::NET5_PATH, assembly_name.name);
        let assembly = Assembly::new(&assembly_path, false)?;
        if assembly.assembly_name != *assembly_name {
            return Err(io::Error::new(io::ErrorKind::Other, "Assembly name not match."));
        }
        Ok(assembly)
    }

    // pub fn load_cor_lib() -> io::Result<Assembly> {
    //     let assembly_path = format!("{}{}.dll", Self::NET5_PATH, Self::COR_LIB_NAME);
    //     Assembly::new(&assembly_path, true)
    // }

    /// 将CorLibType解析成u32，即指向TypeDef（如果当前就是mscorlib）或者TypeRef的token
    pub fn resolve_cor_lib_type(&self, cor_lib_type: &CorLibType) -> io::Result<u32> {
        let type_full_name = &match cor_lib_type {
            CorLibType::Void => "System.Void",
            CorLibType::Boolean => "System.Boolean",
            CorLibType::Char => "System.Char",
            CorLibType::SByte => "System.SByte",
            CorLibType::Byte => "System.Byte",
            CorLibType::Int16 => "System.Int16",
            CorLibType::UInt16 => "System.UInt16",
            CorLibType::Int32 => "System.Int32",
            CorLibType::UInt32 => "System.UInt32",
            CorLibType::Int64 => "System.Int64",
            CorLibType::UInt64 => "System.UInt64",
            CorLibType::Single => "System.Single",
            CorLibType::Double => "System.Double",
            CorLibType::String => "System.String",
            CorLibType::TypedReference => "System.TypedReference",
            CorLibType::IntPtr => "System.IntPtr",
            CorLibType::UIntPtr => "System.UIntPtr",
            CorLibType::Object => "System.Object",
        }.to_string();
        if self.is_cor_lib {
            self.type_defs.key_get(type_full_name).map(|t| t.token).ok_or(io::Error::new(io::ErrorKind::Other, "CorLibType not found."))
        } else {
            self.type_refs.key_get(type_full_name).map(|t| t.token).ok_or(io::Error::new(io::ErrorKind::Other, "CorLibType not found."))
        }
    }
}

/// 解释器执行的上下文
pub struct Context {
    assembly: Rc<Assembly>,
    assembly_index: usize,
    /// 调用堆栈，记录当前assembly的index和method_token
    call_stack: Vec<(usize, u32)>,
    /// 栈ID，每次调用加一，因为Local和Param每次调用就要清除，所以如果根据托管指针来改变，需要检查这个值
    stack_id: usize,
}

impl Context {
    pub fn new(assembly: &Rc<Assembly>, assembly_index: usize) -> Context {
        Context {
            assembly: Rc::clone(assembly),
            assembly_index,
            call_stack: Vec::new(),
            stack_id: 0,
        }
    }

    pub fn make_temp(&self) -> Context {
        Context {
            assembly: Rc::clone(&self.assembly),
            assembly_index: self.assembly_index,
            call_stack: Vec::new(),
            stack_id: self.stack_id,
        }
    }
}

pub struct Interpreter {
    /// index0放入口Assembly，加载的外部Assembly依次往后放
    assemblies: HashVec<String, Rc<Assembly>>,

    stack: VecDeque<ILType>,
    objects: Vec<Object>,
    strings: Vec<String>,
    
    /// 存放Assembly里的所有静态字段 <field_token, ILType>
    pub static_fields: Vec<HashMap<u32, ILType>>,
}

impl Interpreter {
    pub fn new(assembly_path: String) -> io::Result<Interpreter> {
        let mut assemblies = HashVec::new();
        // assemblies.insert(String::from("mscorlib"), Rc::new(Assembly::load_cor_lib().unwrap()));  // index0放入mscorlib

        let assembly = Assembly::new(&assembly_path, false)?;
        assemblies.insert(String::default(), Rc::new(assembly));

        Ok(Interpreter {
            assemblies,
            stack: VecDeque::new(),
            objects: Vec::new(),
            strings: Vec::new(),

            static_fields: Vec::new(),
        })
    }

    pub fn run(&mut self) {
        println!("\nstart run:\n");
        self.il_call(&mut Context::new(&self.assemblies.index_get(0).unwrap(), 0), 0x06000001);
    }

    pub fn format_il_type(&self, il_type: &ILType) -> String {
        match il_type {
            ILType::Ref(ILRefType::Null) => String::from("Null"),
            ILType::Ref(ILRefType::Object(o)) => format!("{}", self.objects[*o as usize].to_string(self)),
            ILType::Ref(ILRefType::String(s)) => format!("{}", self.strings[*s as usize]),
            ILType::Val(v) => format!("{}", v.to_string()),
            ILType::Ptr(p) => format!("Ptr: {:?}", p),
            ILType::NPtr(n) => format!("NPtr: {:?}", n),
        }
    }
    
    fn load_assembly(&mut self, assembly_name: &AssemblyName) -> Rc<Assembly> {
        let assembly = Rc::new(Assembly::load(assembly_name).unwrap());
        self.assemblies.insert(assembly_name.name.clone(), assembly.clone());
        self.static_fields.push(HashMap::new());
        assembly
    }

    /// 解析给定的type_ref，如果其引用的Assembly未加载，那就加载它，返回(type_def_index, 加载后assembly的index)
    fn resolve_type_ref(&mut self, ctx: &Context, type_ref_token: u32) -> (usize, usize) {
        let type_ref = ctx.assembly.type_refs.index_get((type_ref_token & 0x00FFFFFF) as usize - 1).unwrap();
        let mut assembly_index;
        let mut assembly = &Rc::clone(&ctx.assembly);
        let resolution_scope = assembly.metadata.resolve_resolution_scope(type_ref.resolution_scope as u32).unwrap();
        if (resolution_scope >> 24) == 0x23 {  // AssemblyRef
            let assembly_name = &assembly.assembly_refs[(resolution_scope & 0x00FFFFFF) as usize - 1].assembly_name.clone();
            match self.assemblies.key_get_index(&assembly_name.name) {
                Some(index) => {
                    assembly_index = index;
                },
                None => {
                    self.load_assembly(&assembly_name);
                    assembly_index = self.assemblies.len() - 1;
                },
            };
            assembly = self.assemblies.index_get(assembly_index).unwrap();
            let dest_type_index = assembly.type_defs.key_get_index(&type_ref.full_name);
            if dest_type_index.is_none() {  // 说明是ExportedType
                let exported_type = assembly.exported_types.key_get(&type_ref.full_name).unwrap();
                match exported_type.implementation_type {
                    ExportedTypeImpl::AssemblyRef => {
                        let assembly_name = &assembly.assembly_refs[exported_type.implementation_rid as usize - 1].assembly_name.clone();
                        match self.assemblies.key_get_index(&assembly_name.name) {
                            Some(index) => {
                                assembly_index = index;
                            },
                            None => {
                                self.load_assembly(&assembly_name);
                                assembly_index = self.assemblies.len() - 1;
                            }
                        }
                        assembly = self.assemblies.index_get(assembly_index).unwrap();
                        return (assembly.type_defs.key_get_index(&type_ref.full_name).unwrap(), assembly_index);
                    },
                    _ => todo!()
                }
            }
            return (dest_type_index.unwrap(), assembly_index);
        }
        panic!("Cannot resolve type reference")
    }

    /// 解析给定的type_token，可能是type_def或者type_ref，如果为type_ref，就可以自动加载Assembly，返回为type_defs的index
    fn resolve_type_def_or_ref(&mut self, ctx: &mut Context, type_def_or_ref_token: u32) -> usize {
        if type_def_or_ref_token << 8 == 0 {
            return 0;
        }
        match type_def_or_ref_token >> 24 {
            0x01 => {  // TypeRef
                let resolve_result = self.resolve_type_ref(ctx, type_def_or_ref_token);
                ctx.assembly = Rc::clone(self.assemblies.index_get(resolve_result.1).unwrap());
                ctx.assembly_index = resolve_result.1;
                resolve_result.0
            },
            0x02 => {  // TypeDef
                (type_def_or_ref_token as usize & 0x00FFFFFF) - 1
            },
            _ => panic!("Not a type_def or type_ref"),
        }
    }

    /// 获取一个method的local列表
    fn get_method_locals(&mut self, ctx: &Context, method: &Method) -> Vec<ILType> {
        if method.local_var_rid == 0 {
            return Vec::default();
        }
        let local_sig = &ctx.assembly.standalone_sigs[method.local_var_rid as usize - 1].signature;
        if let Some(CallingConventionSig::LocalSig(sig)) = local_sig {
            return ILType::from_type_sigs(sig.locals.iter().collect());
        }
        panic!("Method has no locals")
    }

    /// 尝试获取一个字段的cctor，前提是它是static并且他的cctor尚未被调用过
    fn try_get_cctor(&mut self, ctx: &Context, field: &Field) -> Option<u32> {
        if field.is_static() {
            if self.static_fields[ctx.assembly_index].get(&field.token).is_none() {
                let owner_type = ctx.assembly.type_defs.index_get(field.owner_type as usize).unwrap();
                for rid in owner_type.method_list.iter() {
                    let method = &ctx.assembly.methods[rid as usize - 1];
                    if method.name == ".cctor" {
                        return Some(method.token);
                    }
                }
            }
        }
        None
    }

    /// 将owner_type（从Field获取）中的所有static字段初始化
    fn init_static_fields(&mut self, ctx: &Context, owner_type: u32) {
        let owner_type = ctx.assembly.type_defs.index_get(owner_type as usize).unwrap();
        for rid in owner_type.field_list.iter() {
            let field = &ctx.assembly.fields[rid as usize - 1];
            if field.is_static() {
                self.static_fields[ctx.assembly_index].insert(field.token, ILType::from_signature(field.signature.as_ref().unwrap()));
            }
        }
    }

    fn get_field_list(&mut self, ctx: &Context, type_def_or_ref: u32, field_map: &mut HashVec<u32, ILType>) {
        let mut type_def_or_ref = type_def_or_ref;
        let mut ctx = ctx.make_temp();
        loop {
            let type_def_index = self.resolve_type_def_or_ref(&mut ctx, type_def_or_ref);
            if type_def_index == 0 {
                return;
            }
            let type_def = ctx.assembly.type_defs.index_get(type_def_index).unwrap();
            for rid in type_def.field_list.iter() {
                if ctx.assembly.fields[rid as usize - 1].is_static() {
                    continue;
                }
                field_map.insert(rid, ILType::from_signature(ctx.assembly.fields[rid as usize - 1].signature.as_ref().unwrap()));
            }
            type_def_or_ref = type_def.extends;
        }
    }

    /// 解析member_ref，如果需要则自动加载Assembly并更改ctx，返回为type_defs的index
    fn resolve_member_ref(&mut self, ctx: &mut Context, member_ref_token: u32) -> usize {
        let assembly = Rc::clone(&ctx.assembly);
        let member_ref = &assembly.member_refs[(member_ref_token & 0x00FFFFFF) as usize - 1];
        let name = member_ref.name.clone();
        let class = member_ref.class;

        let type_def = match class >> 24 { 
            0x01 => {  // TypeRef
                let resolve_result = self.resolve_type_ref(ctx, class);
                ctx.assembly = Rc::clone(self.assemblies.index_get(resolve_result.1).unwrap());
                ctx.assembly_index = resolve_result.1;
                resolve_result.0
            },
            0x02 => { // TypeDef
                (class as usize & 0x00FFFFFF) - 1
            },
            0x1B => {  // TypeSpec，表示泛型
                let type_spec = &ctx.assembly.type_specs[(class & 0x00FFFFFF) as usize - 1];
                if let Some(TypeSig::GenericInstSig(sig)) = &type_spec.signature {
                    let type_def_or_ref_token = sig.unwarp_token();
                    self.resolve_type_def_or_ref(ctx, type_def_or_ref_token)
                } else {
                    panic!("Invalid TypeSpec")
                }
            },
            _ => panic!("Invalid member_ref class"),
        };
        let dest_type = ctx.assembly.type_defs.index_get(type_def as usize).unwrap();
        for dest_method_rid in dest_type.method_list.iter() {
            let dest_method = &ctx.assembly.methods[dest_method_rid as usize - 1];
            if dest_method.name == name && dest_method.signature == member_ref.signature {
                return dest_method_rid as usize - 1;
            }
        }
        panic!("Invalid member_ref_token");
    }

    /// 通过method_token或者member_ref_token获取method的index
    fn get_method_index(&mut self, ctx: &mut Context, token: u32) -> usize {
        match token >> 24 {
            0x06 => {  // 表示是当前Assembly内的方法
                (token & 0x00FFFFFF) as usize - 1
            },
            0x0A => {  // 需要先找到MemberRef，再找到TypeRef，最后定位到AssemblyRef
                self.resolve_member_ref(ctx, token)
            },
            0x2B => {  // 泛型方法
                self.get_method_index(ctx, ctx.assembly.method_specs[(token & 0x00FFFFFF) as usize - 1].method)
            },
            _ => panic!("Invalid method_token")
        }
    }

    fn il_internal_call(&mut self, method: &Method) {
        if method.name == "WriteLine" {
            let value = self.stack.pop_back().unwrap();
            println!("{}", self.format_il_type(&value).green());
        }
    }

    fn il_box_obj(&mut self, type_token: u32, value: ILType) {
        self.objects.push(Object::new_box(type_token, value));
        self.stack.push_back(ILType::Ref(ILRefType::Object(self.objects.len() - 1)));
    }

    fn il_new_obj(&mut self, ctx: &Context, type_token: u32) {
        // 由于类存在继承，所以FieldList可能是不连续的
        let mut field_map = HashVec::new();
        self.get_field_list(ctx, type_token, &mut field_map);
        self.objects.push(Object::new(type_token, field_map));
        self.stack.push_back(ILType::Ref(ILRefType::Object(self.objects.len() - 1)));
    }

    fn il_new_string(&mut self, string: String) {
        self.strings.push(string);
        self.stack.push_back(ILType::Ref(ILRefType::String(self.strings.len() - 1)));
    }

    fn il_call(&mut self, ctx: &mut Context, method_or_member_ref: u32) {
        ctx.stack_id += 1;
        let param_count;
        let method_index = self.get_method_index(ctx, method_or_member_ref);
        let method = &ctx.assembly.methods[method_index];
        ctx.call_stack.push((ctx.assembly_index, method_or_member_ref));
        let call_depth = ctx.call_stack.len();
        let method_name = method.to_string(&ctx.assembly);
        for _ in 0..call_depth {
            print!("-");
        }
        println!("call: {}", method_name);
        if method.is_static() {
            param_count = method.param_list.count as usize;
        } else {
            param_count = method.param_list.count as usize + 1;  // 实例方法第一个参数是this
        }

        if method.check_impl_flag_info(ImplAttrFlagInfo::CommonImplAttrFlagInfo(CommonImplAttrFlagInfo::InternalCall)) {
            self.il_internal_call(method);
            ctx.call_stack.pop();
            ctx.assembly_index = ctx.call_stack.last().unwrap().0;
            ctx.assembly = Rc::clone(&self.assemblies.index_get(ctx.assembly_index).unwrap());
            for _ in 0..call_depth {
                print!("-");
            }
            println!("exit: {}", method_name);
            return;
        }

        let assembly = Rc::clone(&ctx.assembly);
        let reader = &assembly.reader;
        let mut rip = method.code_position;  // 当前函数指针

        let mut params = VecDeque::new();
        for _ in 0..param_count {
            params.push_front(self.stack.pop_back().unwrap());  // 逆向出栈，获取参数
        }

        let mut locals = self.get_method_locals(ctx, method);
        'root: loop {  // 禁止使用return脱离循环
            let op = reader.read_u8_immut(&mut rip).unwrap();
            match FromPrimitive::from_u8(op) {
                Some(OpCode::Nop) => {},
                Some(OpCode::Break) => {
                    println!("break");
                },
                Some(OpCode::Ldarg0) => {
                    self.stack.push_back(params[0].clone());
                },
                Some(OpCode::Ldarg1) => {
                    self.stack.push_back(params[1].clone());
                },
                Some(OpCode::Ldarg2) => {
                    self.stack.push_back(params[2].clone());
                },
                Some(OpCode::Ldarg3) => {
                    self.stack.push_back(params[3].clone());
                },
                Some(OpCode::Ldloc0) => {
                    self.stack.push_back(locals[0].clone());
                },
                Some(OpCode::Ldloc1) => {
                    self.stack.push_back(locals[1].clone());
                },
                Some(OpCode::Ldloc2) => {
                    self.stack.push_back(locals[2].clone());
                },
                Some(OpCode::Ldloc3) => {
                    self.stack.push_back(locals[3].clone());
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
                    let index = reader.read_u8_immut(&mut rip).unwrap();
                    self.stack.push_back(params[index as usize].clone());
                },
                Some(OpCode::Ldargas) => {
                    let index = reader.read_u8_immut(&mut rip).unwrap();
                    self.stack.push_back(ILType::Ptr(ILPtr::Param((ctx.stack_id, index as usize))));
                },
                Some(OpCode::Stargs) => {
                    let index = reader.read_u8_immut(&mut rip).unwrap();
                    params[index as usize] = self.stack.pop_back().unwrap();
                },
                Some(OpCode::Ldlocs) => {
                    let index = reader.read_u8_immut(&mut rip).unwrap();
                    self.stack.push_back(locals[index as usize].clone());
                },
                Some(OpCode::Ldlocas) => {
                    let index = reader.read_u8_immut(&mut rip).unwrap();
                    self.stack.push_back(ILType::Ptr(ILPtr::Local((ctx.stack_id, index as usize))));
                },
                Some(OpCode::Stlocs) => {
                    let index = reader.read_u8_immut(&mut rip).unwrap();
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
                    let val = reader.read_u8_immut(&mut rip).unwrap();
                    self.stack.push_back(ILType::Val(ILValType::Int32(val as i32)));
                },
                Some(OpCode::Ldci4) => {
                    let val = reader.read_u32_immut(&mut rip).unwrap();
                    self.stack.push_back(ILType::Val(ILValType::Int32(val as i32)));
                },
                Some(OpCode::Ldci8) => {
                    let val = reader.read_u64_immut(&mut rip).unwrap();
                    self.stack.push_back(ILType::Val(ILValType::Int64(val as i64)));
                },
                Some(OpCode::Ldcr4) => {
                    let val = reader.read_f32_immut(&mut rip).unwrap();
                    self.stack.push_back(ILType::Val(ILValType::Single(val)));
                },
                Some(OpCode::Ldcr8) => {
                    let val = reader.read_f64_immut(&mut rip).unwrap();
                    self.stack.push_back(ILType::Val(ILValType::Double(val)));
                },
                Some(OpCode::Dup) => {
                    self.stack.push_back(self.stack.back().unwrap().clone());
                },
                Some(OpCode::Pop) => {
                    self.stack.pop_back();
                },
                Some(OpCode::Jmp) => {
                    assert_eq!(assembly.params.len(), 0);
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    self.il_call(ctx, token);
                    break;
                },
                Some(OpCode::Call) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    self.il_call(ctx, token);
                },
                Some(OpCode::Calli) => {
                    todo!();
                },
                Some(OpCode::Ret) => {
                    break;
                },
                Some(OpCode::Brs) => {
                    let target = reader.read_u8_immut(&mut rip).unwrap() as i8;
                    rip = (rip as isize + target as isize) as usize;
                },
                Some(OpCode::Brfalses) => {
                    let target = reader.read_u8_immut(&mut rip).unwrap() as i8;
                    let val = self.stack.pop_back().unwrap();
                    if val.is_false_type() {
                        rip = (rip as isize + target as isize) as usize;
                    }
                },
                Some(OpCode::Brtrues) => {
                    let target = reader.read_u8_immut(&mut rip).unwrap() as i8;
                    let val = self.stack.pop_back().unwrap();
                    if !val.is_false_type() {
                        rip = (rip as isize + target as isize) as usize;
                    }
                },
                Some(OpCode::Beqs) => {
                    todo!();
                },
                Some(OpCode::Bges) => {
                    todo!();
                },
                Some(OpCode::Bgts) => {
                    todo!();
                },
                Some(OpCode::Bles) => {
                    todo!();
                },
                Some(OpCode::Blts) => {
                    todo!();
                },
                Some(OpCode::Bneuns) => {
                    todo!();
                },
                Some(OpCode::Bgeuns) => {
                    todo!();
                },
                Some(OpCode::Bgtuns) => {
                    todo!();
                },
                Some(OpCode::Bleuns) => {
                    todo!();
                },
                Some(OpCode::Bltuns) => {
                    todo!();
                },
                Some(OpCode::Br) => {
                    todo!();
                },
                Some(OpCode::Brfalse) => {
                    todo!();
                },
                Some(OpCode::Brtrue) => {
                    todo!();
                },
                Some(OpCode::Beq) => {
                    todo!();
                },
                Some(OpCode::Bge) => {
                    todo!();
                },
                Some(OpCode::Bgt) => {
                    todo!();
                },
                Some(OpCode::Ble) => {
                    todo!();
                },
                Some(OpCode::Blt) => {
                    todo!();
                },
                Some(OpCode::Bneun) => {
                    todo!();
                },
                Some(OpCode::Bgeun) => {
                    todo!();
                },
                Some(OpCode::Bgtun) => {
                    todo!();
                },
                Some(OpCode::Bleun) => {
                    todo!();
                },
                Some(OpCode::Bltun) => {
                    todo!();
                },
                Some(OpCode::Switch) => {
                    let n = reader.read_u32_immut(&mut rip).unwrap();
                    let val = self.stack.pop_back().unwrap();
                    if let ILType::Val(val) = val {
                        let val = val.to_u32();
                        if val < n {
                            rip += val as usize * 4;
                            let target = reader.read_u32_immut(&mut rip).unwrap() as i32;
                            rip = (rip as isize + target as isize) as usize;
                        } else {
                            rip += n as usize * 4;
                        }
                    } else {
                        panic!("switch value must be a val");
                    }
                },
                Some(OpCode::Ldindi1) => {
                    todo!();
                },
                Some(OpCode::Ldindu1) => {
                    todo!();
                },
                Some(OpCode::Ldindi2) => {
                    todo!();
                },
                Some(OpCode::Ldindu2) => {
                    todo!();
                },
                Some(OpCode::Ldindi4) => {
                    todo!();
                },
                Some(OpCode::Ldindu4) => {
                    todo!();
                },
                Some(OpCode::Ldindi8) => {
                    todo!();
                },
                Some(OpCode::Ldindi) => {
                    todo!();
                },
                Some(OpCode::Ldindr4) => {
                    todo!();
                },
                Some(OpCode::Ldindr8) => {
                    todo!();
                },
                Some(OpCode::Ldindref) => {
                    todo!();
                },
                Some(OpCode::Stindref) => {
                    todo!();
                },
                Some(OpCode::Stindi1) => {
                    todo!();
                },
                Some(OpCode::Stindi2) => {
                    todo!();
                },
                Some(OpCode::Stindi4) => {
                    todo!();
                },
                Some(OpCode::Stindi8) => {
                    todo!();
                },
                Some(OpCode::Stindr4) => {
                    todo!();
                },
                Some(OpCode::Stindr8) => {
                    todo!();
                },
                Some(OpCode::Add) => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    self.stack.push_back(a + b);
                },
                Some(OpCode::Sub) => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
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
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    self.stack.push_back(a & b);
                },
                Some(OpCode::Or) => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    self.stack.push_back(a | b);
                },
                Some(OpCode::Xor) => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    self.stack.push_back(a ^ b);
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
                Some(OpCode::Convi1) => {
                    todo!();
                },
                Some(OpCode::Convi2) => {
                    todo!();
                },
                Some(OpCode::Convi4) => {
                    todo!();
                },
                Some(OpCode::Convi8) => {
                    todo!();
                },
                Some(OpCode::Convr4) => {
                    todo!();
                },
                Some(OpCode::Convr8) => {
                    todo!();
                },
                Some(OpCode::Convu4) => {
                    todo!();
                },
                Some(OpCode::Convu8) => {
                    todo!();
                },
                Some(OpCode::Callvirt) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    self.il_call(ctx, token);
                },
                Some(OpCode::Cpobj) => {
                    todo!();
                },
                Some(OpCode::Ldobj) => {
                    todo!();
                },
                Some(OpCode::Ldstr) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let str = assembly.metadata.get_us_string(token).unwrap();
                    self.il_new_string(str);
                },
                Some(OpCode::Newobj) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let method_index = self.get_method_index(ctx, token);
                    let type_token = ctx.assembly.methods[method_index].owner_type + 0x02000001;
                    self.il_new_obj(ctx, type_token);  // 根据.ctor找到类，new出来推送到栈上，作为.ctor的this
                    self.stack.push_back(self.stack.back().unwrap().clone());
                    self.il_call(ctx, token);
                },
                Some(OpCode::Castclass) => {
                    let type_token = reader.read_u32_immut(&mut rip).unwrap();
                    let obj = self.stack.pop_back().unwrap();
                    match obj {
                        ILType::Ref(ILRefType::Null) => {
                            self.stack.push_back(ILType::Ref(ILRefType::Null));
                        },
                        ILType::Ref(ILRefType::Object(index)) => {
                            let obj_type_token = self.objects[index].get_type();
                            let mut temp_ctx = ctx.make_temp();
                            let obj_type_index = self.resolve_type_def_or_ref(&mut temp_ctx, obj_type_token);
                            let obj_type = ctx.assembly.type_defs.index_get(obj_type_index).expect("TypeLoadException");
                            let mut extends = obj_type.extends;
                            while extends != 0 {
                                let extends_index = self.resolve_type_def_or_ref(&mut temp_ctx, obj_type_token);
                                let extends_type = ctx.assembly.type_defs.index_get(extends_index).expect("TypeLoadException");
                                if extends_type.token == type_token {
                                    self.objects[index].type_token = type_token;
                                    self.stack.push_back(ILType::Ref(ILRefType::Object(index)));
                                    break 'root;
                                }
                                extends = extends_type.extends;
                            }
                            panic!("InvalidCastException");
                        },
                        ILType::Ref(ILRefType::String(index)) => {
                            self.stack.push_back(ILType::Ref(ILRefType::String(index)));
                        },
                        _ => panic!("InvalidCastException"),
                    }
                },
                Some(OpCode::Isinst) => {
                    todo!();
                },
                Some(OpCode::Convrun) => {
                    todo!();
                },
                Some(OpCode::Unbox) => {
                    todo!();
                },
                Some(OpCode::Throw) => {
                    todo!();
                },
                Some(OpCode::Ldfld) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let rid = token - 0x04000000;
                    let this = self.stack.pop_back().unwrap();
                    match this {
                        ILType::Ref(ref_type) => {
                            match ref_type {
                                ILRefType::Null => {
                                    panic!("Null reference exception.");
                                },
                                ILRefType::String(_) => {
                                    todo!();
                                },
                                ILRefType::Object(index) => {
                                    let obj = &self.objects[index];
                                    self.stack.push_back(obj.get_field(rid).unwrap().clone());
                                },
                            }
                        },
                        _ => {
                            panic!("Invalid this reference.");
                        }
                    }
                },
                Some(OpCode::Ldflda) => {
                    todo!();
                },
                Some(OpCode::Stfld) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let value = self.stack.pop_back().unwrap();
                    let this = self.stack.pop_back().unwrap();
                    match this {
                        ILType::Ref(ref_type) => {
                            match ref_type {
                                ILRefType::Null => {
                                    panic!("Null reference exception.");
                                },
                                ILRefType::String(_) => {
                                    todo!();
                                },
                                ILRefType::Object(index) => {
                                    let obj = &mut self.objects[index];
                                    obj.set_field(token, value);
                                },
                            }
                        },
                        _ => {
                            panic!("Invalid this reference.");
                        }
                    }
                },
                Some(OpCode::Ldsfld) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let field = &assembly.fields[token as usize - 0x04000001];
                    let cctor = self.try_get_cctor(ctx, field);
                    if cctor.is_some() {  // 如果没有初始化，则调用cctor
                        self.init_static_fields(ctx, field.owner_type);
                        self.il_call(ctx, cctor.unwrap());
                    }
                    let field_value = self.static_fields[ctx.assembly_index].get(&token).unwrap().clone();
                    self.stack.push_back(field_value);
                },
                Some(OpCode::Ldsflda) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let field = &assembly.fields[token as usize - 0x04000001];
                    let cctor = self.try_get_cctor(ctx, field);
                    if cctor.is_some() {  // 如果没有初始化，则调用cctor
                        self.init_static_fields(ctx, field.owner_type);
                        self.il_call(ctx, cctor.unwrap());
                    }
                    self.stack.push_back(ILType::Ptr(ILPtr::Static((ctx.assembly_index, token))));
                },
                Some(OpCode::Stsfld) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let field = &assembly.fields[token as usize - 0x04000001];
                    let cctor = self.try_get_cctor(ctx, field);
                    if cctor.is_some() {  // 如果没有初始化，则调用cctor初始化
                        self.init_static_fields(ctx, field.owner_type);
                        self.il_call(ctx, cctor.unwrap());
                    }
                    self.static_fields[ctx.assembly_index].insert(token, self.stack.pop_back().unwrap());
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
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let value = self.stack.pop_back().unwrap();
                    self.il_box_obj(token, value);
                },
                Some(OpCode::Newarr) => {
                    todo!();
                },
                Some(OpCode::Ldlen) => {
                    todo!();
                },
                Some(OpCode::Ldelema) => {
                    todo!();
                },
                Some(OpCode::Ldelemi1) => {
                    todo!();
                },
                Some(OpCode::Ldelemu1) => {
                    todo!();
                },
                Some(OpCode::Ldelemi2) => {
                    todo!();
                },
                Some(OpCode::Ldelemu2) => {
                    todo!();
                },
                Some(OpCode::Ldelemi4) => {
                    todo!();
                },
                Some(OpCode::Ldelemu4) => {
                    todo!();
                },
                Some(OpCode::Ldelemi8) => {
                    todo!();
                },
                Some(OpCode::Ldelemi) => {
                    todo!();
                },
                Some(OpCode::Ldelemr4) => {
                    todo!();
                },
                Some(OpCode::Ldelemr8) => {
                    todo!();
                },
                Some(OpCode::Ldelemref) => {
                    todo!();
                },
                Some(OpCode::Stelemi) => {
                    todo!();
                },
                Some(OpCode::Stelemi1) => {
                    todo!();
                },
                Some(OpCode::Stelemi2) => {
                    todo!();
                },
                Some(OpCode::Stelemi4) => {
                    todo!();
                },
                Some(OpCode::Stelemi8) => {
                    todo!();
                },
                Some(OpCode::Stelemr4) => {
                    todo!();
                },
                Some(OpCode::Stelemr8) => {
                    todo!();
                },
                Some(OpCode::Stelemref) => {
                    todo!();
                },
                Some(OpCode::Ldelem) => {
                    todo!();
                },
                Some(OpCode::Stelem) => {
                    todo!();
                },
                Some(OpCode::Unboxany) => {
                    let token = reader.read_u32_immut(&mut rip).unwrap();
                    let boxed = self.stack.pop_back().unwrap();
                    let ref_obj = &self.objects[boxed.get_ref()];
                    if ref_obj.get_type() != token {
                        panic!("unboxany: type mismatch");
                    }
                    self.stack.push_back(ref_obj.box_value.clone().unwrap());
                },
                Some(OpCode::Convovfi1) => {
                    todo!();
                },
                Some(OpCode::Convovfu1) => {
                    todo!();
                },
                Some(OpCode::Convovfi2) => {
                    todo!();
                },
                Some(OpCode::Convovfu2) => {
                    todo!();
                },
                Some(OpCode::Convovfi4) => {
                    todo!();
                },
                Some(OpCode::Convovfu4) => {
                    todo!();
                },
                Some(OpCode::Convovfi8) => {
                    todo!();
                },
                Some(OpCode::Convovfu8) => {
                    todo!();
                },
                Some(OpCode::Refanyval) => {
                    todo!();
                },
                Some(OpCode::Ckfinite) => {
                    todo!();
                },
                Some(OpCode::Mkrefany) => {
                    todo!();
                },
                Some(OpCode::Ldtoken) => {
                    todo!();
                },
                Some(OpCode::Convu2) => {
                    todo!();
                },
                Some(OpCode::Convu1) => {
                    todo!();
                },
                Some(OpCode::Convi) => {
                    todo!();
                },
                Some(OpCode::Convovfi) => {
                    todo!();
                },
                Some(OpCode::Convovfu) => {
                    todo!();
                },
                Some(OpCode::Addovf) => {
                    todo!();
                },
                Some(OpCode::Addovfun) => {
                    todo!();
                },
                Some(OpCode::Mulovf) => {
                    todo!();
                },
                Some(OpCode::Mulovfun) => {
                    todo!();
                },
                Some(OpCode::Subovf) => {
                    todo!();
                },
                Some(OpCode::Subovfun) => {
                    todo!();
                },
                Some(OpCode::Endfault) => {
                    todo!();
                },
                Some(OpCode::Leave) => {
                    todo!();
                },
                Some(OpCode::Leaves) => {
                    todo!();
                },
                Some(OpCode::Stindi) => {
                    todo!();
                },
                Some(OpCode::Convu) => {
                    let val = self.stack.pop_back().unwrap();
                    if let ILType::Val(val) = val {
                        self.stack.push_back(ILType::Val(ILValType::Isize(val.to_usize() as isize)));
                    } else {
                        panic!("convu: type mismatch");
                    }
                },
                Some(OpCode::Next) => {
                    let op = reader.read_u8_immut(&mut rip).unwrap();
                    match FromPrimitive::from_u8(op) {
                        Some(OpCode2::Arglist) => {
                            todo!();
                        },
                        Some(OpCode2::Ceq) => {
                            let v2 = self.stack.pop_back().unwrap();
                            let v1 = self.stack.pop_back().unwrap();
                            if v1 == v2 {
                                self.stack.push_back(ILType::Val(ILValType::Int32(1)));
                            } else {
                                self.stack.push_back(ILType::Val(ILValType::Int32(0)));
                            }
                        },
                        Some(OpCode2::Cgt) => {
                            let v2 = self.stack.pop_back().unwrap();
                            let v1 = self.stack.pop_back().unwrap();
                            if v1 > v2 {
                                self.stack.push_back(ILType::Val(ILValType::Int32(1)));
                            } else {
                                self.stack.push_back(ILType::Val(ILValType::Int32(0)));
                            }
                        },
                        Some(OpCode2::Cgtun) => {
                            todo!();
                        },
                        Some(OpCode2::Clt) => {
                            let v2 = self.stack.pop_back().unwrap();
                            let v1 = self.stack.pop_back().unwrap();
                            if v1 < v2 {
                                self.stack.push_back(ILType::Val(ILValType::Int32(1)));
                            } else {
                                self.stack.push_back(ILType::Val(ILValType::Int32(0)));
                            }
                        },
                        Some(OpCode2::Cltun) => {
                            todo!();
                        },
                        Some(OpCode2::Ldftn) => {
                            todo!();
                        },
                        Some(OpCode2::Ldvirtftn) => {
                            todo!();
                        },
                        Some(OpCode2::Ldarg) => {
                            todo!();
                        },
                        Some(OpCode2::Ldarga) => {
                            todo!();
                        },
                        Some(OpCode2::Starg) => {
                            todo!();
                        },
                        Some(OpCode2::Ldloc) => {
                            todo!();
                        },
                        Some(OpCode2::Ldloca) => {
                            todo!();
                        },
                        Some(OpCode2::Stloc) => {
                            todo!();
                        },
                        Some(OpCode2::Localloc) => {
                            let size = self.stack.pop_back().unwrap();
                            if let ILType::Val(val) = size {
                                let size = val.to_usize();
                                self.stack.push_back(ILType::NPtr(ILNPtr::new(size)));
                            } else {
                                panic!("localloc: type mismatch");
                            }
                        },
                        Some(OpCode2::Endfilter) => {
                            todo!();
                        },
                        Some(OpCode2::Unaligned) => {
                            todo!();
                        },
                        Some(OpCode2::Volatile) => {
                            todo!();
                        },
                        Some(OpCode2::Tail) => {
                            todo!();
                        },
                        Some(OpCode2::Initobj) => {
                            todo!();
                        },
                        Some(OpCode2::Constrained) => {
                            todo!();
                        },
                        Some(OpCode2::Cpblk) => {
                            todo!();
                        },
                        Some(OpCode2::Initblk) => {
                            todo!();
                        },
                        Some(OpCode2::No) => {
                            todo!();
                        },
                        Some(OpCode2::Rethrow) => {
                            todo!();
                        },
                        Some(OpCode2::Sizeof) => {
                            todo!();
                        },
                        Some(OpCode2::Refanytype) => {
                            todo!();
                        },
                        Some(OpCode2::Readonly) => {
                            todo!();
                        },
                        _ => {
                            panic!("Unknown OpCode: 0x{:02X}", op);
                        }
                    }
                }
                _ => {
                    panic!("Unknown OpCode: 0x{:02X}", op);
                }
            }
        }
        ctx.call_stack.pop();
        ctx.assembly_index = ctx.call_stack.last().unwrap().0;
        ctx.assembly = Rc::clone(&self.assemblies.index_get(ctx.assembly_index).unwrap());
        for _ in 0..call_depth {
            print!("-");
        }
        println!("exit: {}", method_name);
    }
}
