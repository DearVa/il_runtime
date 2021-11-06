use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

use std::collections::VecDeque;

mod op_codes;
mod metadata;
use op_codes::*;

use num_traits::FromPrimitive;

fn main() {
    let mut file = File::open(r"F:\SourceOffline\Rust\il_runtime\ILAsm\TestCsharp.dll").expect("Can't open assembly.");
    file.seek(SeekFrom::Start(0x3C2)).expect("Can't seek to metadata of assembly.");
    
    file.seek(SeekFrom::Start(0x254)).expect("Can't seek to Entry Point.");  // 目前这是通过16进制编辑器定位的
    il_run(&mut file)
}

fn il_call() {

}

fn il_run(mut file: &File) {
    let mut stack = VecDeque::new();
    let mut local0: i32;
    let mut local1: i32;
    let mut local2: i32;
    let mut local3: i32;

    let mut buf: [u8; 1] = [0; 1];
    loop {
        file.read_exact(&mut buf).expect("Can't read OpCode.");
        match FromPrimitive::from_u8(buf[0]) {
            Some(OpCode::Nop) => {
                println!("nop");
            },
            Some(OpCode::Break) => {
                println!("break");
            },
            Some(OpCode::Ldarg0) => {
                println!("ldarg.0");
            },
            Some(OpCode::Ldarg1) => {
                println!("ldarg.1");
            },
            Some(OpCode::Ldarg2) => {
                println!("ldarg.2");
            },
            Some(OpCode::Ldarg3) => {
                println!("ldarg.3");
            },
            Some(OpCode::Ldloc0) => {
                println!("ldloc.0")
            },
            Some(OpCode::Ldloc1) => {
                println!("ldloc.1")
            },
            Some(OpCode::Ldloc2) => {
                println!("ldloc.2")
            },
            Some(OpCode::Ldloc3) => {
                println!("ldloc.3")
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
                return
            },
            Some(OpCode::Calli) => {
                return
            },
            Some(OpCode::Ret) => {
                return
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
                println!("Unknown OpCode: 0x{:02x}", buf[0]);
                return
            }
        }
    }
}