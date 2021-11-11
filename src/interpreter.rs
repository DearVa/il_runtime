use std::io;
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
mod type_def;
use type_def::*;
mod type_ref;
use type_ref::*;
mod method;
use method::*;
mod param;
use param::*;
mod object;
use object::*;


pub struct Interpreter {
    pub image: Vec<u8>,    // 映像，一次性读取
    pub pe: PE,
    pub metadata: Metadata,

    pub type_refs: HashMap<u32, TypeRef>,   //  <token(0x01000001...), TypeRef>
    pub type_defs: HashMap<u32, TypeDef>,   //  <token(0x01000001...), TypeRef>
    pub methods: HashMap<u32, Method>,      // <token(0x06000001...), Method>
    pub params: HashMap<u32, Param>,        // <token(0x08000001...), Param>

    pub stack: VecDeque<ILType>,
    pub objects: Vec<Object>,
    pub strings: Vec<String>,
}

impl Interpreter {
    pub fn new(asm_path: &'static str) -> io::Result<Interpreter> {
        let mut file = File::open(asm_path)?;
        let metadata = file.metadata().expect("unable to read metadata");
        let mut image = vec![0; metadata.len() as usize];
        file.read(&mut image).expect("Error reading assembly, overflow.");
        let mut reader = ImageReader::new(&image);
        let pe = PE::new(&mut reader)?;
        let metadata = Metadata::new(&pe, &mut reader)?;

        let type_refs = TypeRef::read_type_refs(&metadata)?;
        let type_defs = TypeDef::read_type_defs(&metadata)?;

        let methods = Method::read_methods(&pe, &metadata, &mut reader)?;
        println!("Methods:");
        for method in methods.iter() {
            println!("{:?}", method.1);
        }

        println!("\nParams:");
        let params = Param::read_params(&metadata)?;
        for param in params.iter() {
            println!("{:?}", param.1);
        }

        Ok(Interpreter {
            image,
            pe,
            metadata,

            type_refs,
            type_defs,
            methods,
            params,

            stack: VecDeque::new(),
            objects: Vec::new(),
            strings: Vec::new(),
        })
    }

    pub fn run(&mut self) {
        println!("\nstart run:\n");
        self.il_call(0x06000001);
    }

    fn il_new_obj(&mut self, token: u32, value: ILType) {
        self.objects.push(Object::new(token, value));
        self.stack.push_back(ILType::Ref(ILRefType::Object(self.objects.len() - 1)));
    }

    fn il_new_string(&mut self, string: String) {
        self.strings.push(string);
        self.stack.push_back(ILType::Ref(ILRefType::String(self.strings.len() - 1)));
    }

    fn il_call(&mut self, method_token: u32) {
        let method = self.methods.get(&method_token).unwrap();
        println!("call: {:?}", method);
        let param_count = method.param_list.count as usize;
        let mut params = VecDeque::new();
        for _ in 0..param_count {
            params.push_front(self.stack.pop_back().unwrap());  // 逆向出栈，获取参数
        }

        let mut locals = [ILType::Ref(ILRefType::Null); 8];  // TODO

        let mut rip = method.code_position;  // 当前函数指针
        loop {
            let op = FromPrimitive::from_u8(self.image[rip]);
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
                    let index = self.image[rip];
                    rip += 1;
                    todo!();
                },
                Some(OpCode::Ldargas) => {
                    let index = self.image[rip];
                    rip += 1;
                    todo!();
                },
                Some(OpCode::Stargs) => {
                    let index = self.image[rip];
                    rip += 1;
                    todo!();
                },
                Some(OpCode::Ldlocs) => {
                    let index = self.image[rip];
                    rip += 1;
                    self.stack.push_back(locals[index as usize]);
                },
                Some(OpCode::Ldlocas) => {
                    let index = self.image[rip];
                    rip += 1;
                    todo!();
                },
                Some(OpCode::Stlocs) => {
                    let index = self.image[rip];
                    rip += 1;
                    todo!();
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
                    let val = self.image[rip];
                    rip += 1;
                    self.stack.push_back(ILType::Val(ILValType::Byte(val)));
                },
                Some(OpCode::Ldci4) => {
                    let val = u32::from_le_bytes(self.image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    self.stack.push_back(ILType::Val(ILValType::UInt32(val)));
                },
                Some(OpCode::Ldci8) => {
                    let val = u64::from_le_bytes(self.image[rip..rip + 8].try_into().unwrap());
                    rip += 8;
                    self.stack.push_back(ILType::Val(ILValType::UInt64(val)));
                },
                Some(OpCode::Ldcr4) => {
                    let val = f32::from_le_bytes(self.image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    self.stack.push_back(ILType::Val(ILValType::Single(val)));
                },
                Some(OpCode::Ldcr8) => {
                    let val = f64::from_le_bytes(self.image[rip..rip + 8].try_into().unwrap());
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
                    todo!();
                },
                Some(OpCode::Call) => {
                    let token = u32::from_le_bytes(self.image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    if token == 0xA00000D || token == 0xA00000E {
                        let val = self.stack.pop_back().unwrap();
                        println!("{}", self.convert_to_string(&val).green());  // TODO: Console.PrintLine
                    } else {
                        self.il_call(token);
                    }
                },
                Some(OpCode::Calli) => {
                    todo!();
                },
                Some(OpCode::Ret) => {
                    return;
                },
                Some(OpCode::Brs) => {
                    let target = self.image[rip];
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
                    let token = u32::from_le_bytes(self.image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    let str = self.metadata.get_us_string(token).unwrap();
                    self.il_new_string(str);
                },
                Some(OpCode::Newobj) => {
                    todo!();  // 根据.ctor找到类
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
                    let token = u32::from_le_bytes(self.image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    let value = self.stack.pop_back().unwrap();
                    self.il_new_obj(token, value);
                },
                Some(OpCode::Newarr) => {
                    todo!();
                },
                Some(OpCode::Ldlen) => {
                    todo!();
                },
                // 忽略一些
                Some(OpCode::Unboxany) => {
                    let token = u32::from_le_bytes(self.image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    let boxed = self.stack.pop_back().unwrap();
                    let ref_obj = &self.objects[boxed.get_ref()];
                    if ref_obj.get_type() != token {
                        panic!("unboxany: type mismatch");
                    }
                    self.stack.push_back(*ref_obj.value.clone());
                },
                // 忽略一些
                _ => {
                    println!("Unknown OpCode: 0x{:02X}", self.image[rip - 1]);
                    return
                }
            }
        }
    }

    fn convert_to_string(&self, val: &ILType) -> String {
        match val {
            ILType::Ref(ILRefType::String(s)) => {
                self.strings[*s].to_string()
            },
            ILType::Ref(ILRefType::Object(o)) => {
                self.objects[*o].to_string()
            },
            ILType::Ref(ILRefType::Null) => {
                "null".to_string()
            },
            ILType::Val(v) => {
                v.to_string()
            },
        }
    }
}