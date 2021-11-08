use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use num_traits::FromPrimitive;
use std::convert::TryInto;
use colored::*;

mod op_codes;
use op_codes::*;
mod image_reader;
use image_reader::*;
mod metadata;
use metadata::*;

pub struct Interpreter {
    pub image: Vec<u8>,    // 映像，一次性读取
    pub pe: PE,
    pub metadata: Metadata,
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
        Ok(Interpreter {
            image: image,
            pe,
            metadata,
        })
    }

    pub fn run(&mut self) {
        let main = self.metadata.methods.get(&0x06000001).unwrap();  // Main方法，static
        let mut stack = VecDeque::new();
        stack.push_back(0);
        stack.push_back(0);
        self.il_call(main, &mut stack);
    }

    fn il_call(&self, method: &Method, stack: &mut VecDeque<i32>) {
        let param_count = self.metadata.get_param_count(method) as usize;
        let mut params = vec![0i32; param_count];
        for i in 0..param_count {
            params[param_count - i - 1] = stack.pop_back().unwrap();  // 逆向出栈
        }

        let mut local0: i32 = 0;
        let mut local1: i32 = 0;
        let mut local2: i32 = 0;
        let mut local3: i32 = 0;

        let mut rip = method.code_offset;  // 当前函数指针
        loop {
            let op = FromPrimitive::from_u8(self.image[rip]);
            rip += 1;
            match op {
                Some(OpCode::Nop) => {
                    println!("nop");
                },
                Some(OpCode::Break) => {
                    println!("break");
                },
                Some(OpCode::Ldarg0) => {
                    stack.push_back(params[0]);
                },
                Some(OpCode::Ldarg1) => {
                    stack.push_back(params[1]);
                },
                Some(OpCode::Ldarg2) => {
                    stack.push_back(params[2]);
                },
                Some(OpCode::Ldarg3) => {
                    stack.push_back(params[3]);
                },
                Some(OpCode::Ldloc0) => {
                    stack.push_back(local0);
                },
                Some(OpCode::Ldloc1) => {
                    stack.push_back(local1);
                },
                Some(OpCode::Ldloc2) => {
                    stack.push_back(local2);
                },
                Some(OpCode::Ldloc3) => {
                    stack.push_back(local3);
                },
                Some(OpCode::Stloc0) => {
                    local0 = stack.pop_back().unwrap();
                },
                Some(OpCode::Stloc1) => {
                    local1 = stack.pop_back().unwrap();
                },
                Some(OpCode::Stloc2) => {
                    local2 = stack.pop_back().unwrap();
                },
                Some(OpCode::Stloc3) => {
                    local3 = stack.pop_back().unwrap();
                },
                // 忽略一些
                Some(OpCode::Ldci4m1) => {
                    stack.push_back(-1);
                },
                Some(OpCode::Ldci40) => {
                    stack.push_back(0);
                },
                Some(OpCode::Ldci41) => {
                    stack.push_back(1);
                },
                Some(OpCode::Ldci42) => {
                    stack.push_back(2);
                },
                Some(OpCode::Ldci43) => {
                    stack.push_back(3);
                },
                Some(OpCode::Ldci44) => {
                    stack.push_back(4);
                },
                Some(OpCode::Ldci45) => {
                    stack.push_back(5);
                },
                Some(OpCode::Ldci46) => {
                    stack.push_back(6);
                },
                Some(OpCode::Ldci47) => {
                    stack.push_back(7);
                },
                Some(OpCode::Ldci48) => {
                    stack.push_back(8);
                },
                // 忽略一些
                Some(OpCode::Jmp) => {
                    return
                },
                Some(OpCode::Call) => {
                    let token = u32::from_le_bytes(self.image[rip..rip + 4].try_into().unwrap());
                    rip += 4;
                    println!("call 0x{:04X}", token);
                    if token == 0xA00000D {
                        println!("{}", stack.pop_back().unwrap().to_string().green());  // TODO: Console.PrintLine
                        return
                    }
                    self.il_call(self.metadata.methods.get(&token).expect("method not found"), stack);
                },
                Some(OpCode::Calli) => {
                    return
                },
                Some(OpCode::Ret) => {
                    return
                },
                Some(OpCode::Brs) => {
                    let target = self.image[rip];
                    rip += 1 + target as usize;
                },
                // 忽略一些
                Some(OpCode::Add) => {
                    let a = stack.pop_back().unwrap();
                    let b = stack.pop_back().unwrap();
                    stack.push_back(a + b);
                },
                Some(OpCode::Sub) => {
                    let a = stack.pop_back().unwrap();
                    let b = stack.pop_back().unwrap();
                    stack.push_back(a - b);
                },
                Some(OpCode::Mul) => {
                    let a = stack.pop_back().unwrap();
                    let b = stack.pop_back().unwrap();
                    stack.push_back(a * b);
                },
                Some(OpCode::Div) => {
                    let a = stack.pop_back().unwrap();
                    let b = stack.pop_back().unwrap();
                    stack.push_back(a / b);
                },
                Some(OpCode::Divun) => {
    
                },
                Some(OpCode::Rem) => {
    
                },
                Some(OpCode::Remun) => {
    
                },
                Some(OpCode::And) => {
    
                },
                Some(OpCode::Or) => {
    
                },
                Some(OpCode::Xor) => {
    
                },
                Some(OpCode::Shl) => {
    
                },
                Some(OpCode::Shr) => {
    
                },
                Some(OpCode::Shrun) => {
    
                },
                Some(OpCode::Neg) => {
    
                },
                Some(OpCode::Not) => {
    
                },
                // 忽略一些
                _ => {
                    println!("Unknown OpCode: 0x{:02X}", self.image[rip]);
                    return
                }
            }
        }
    }
}