// 读取Assembly的元数据
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;

pub struct Method {
    token: u32,
    offset: u64,        // 该条Metadata在image中的偏移
    rva: u16,           // 函数入口在image中的偏移
    impl_flags: u32,    // 实现标志
    flags: u16,         // 函数标志
    name: u16,          // 函数名
    signature: u16,     // 签名
    param_list: u16,    // 参数列表
    info: String        // 函数信息
}

impl Method {
    pub fn read_methods(mut file: &File, count: usize) -> Result<Vec<Method>, io::Error> {
        let mut methods = Vec::new();
        let mut buf: [u8; 14] = [0; 14];
        for i in 0..count {
            let offset = file.stream_position()?;
            file.read_exact(&mut buf)?;
            methods.push(Method {
                token: 0x06000000 + i as u32,
                offset: offset,
                rva: u16::from_be_bytes(buf[0..1].try_into().unwrap()),
                impl_flags: u32::from_be_bytes(buf[2..5].try_into().unwrap()),
                flags: u16::from_be_bytes(buf[6..7].try_into().unwrap()),
                name: u16::from_be_bytes(buf[8..9].try_into().unwrap()),
                signature: u16::from_be_bytes(buf[10..11].try_into().unwrap()),
                param_list: u16::from_be_bytes(buf[12..13].try_into().unwrap()),
                info: String::new()
            });
        }
        Ok(methods)
    }
}

pub struct Metadata {
    pub methods: Vec<Method>
}

impl Metadata {
    pub fn new(mut file: &File) -> Metadata {
        Metadata {
            methods: Method::read_methods(&mut file, 3).unwrap()
        }
    }
}