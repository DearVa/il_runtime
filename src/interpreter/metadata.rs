mod flag_info;
use flag_info::*;

use std::collections::HashMap;
use std::io;
use std::convert::TryInto;

pub struct Method {
    pub token: u32,         // 和字典Key一致
    pub offset: usize,      // 该条Metadata在image中的偏移
    pub rva: u32,           // 函数入口在image中的偏移
    pub impl_flags: u16,    // 实现标志
    pub flags: u16,         // 函数标志
    pub name: u16,          // 函数名
    pub signature: u16,     // 签名
    pub param_list: u16,    // 参数列表
    pub info: String,       // 函数信息

    pub max_stack: u16,     // 最大堆栈大小
    pub header_size: u8,    // 函数头大小
    pub code_size: u32,     // 代码大小

    pub code_offset: usize, // IL指令在文件中的真实位置
}

impl Method {
    // 读取Assembly的元数据
    pub fn read_methods(image: &Vec<u8>, count: usize) -> Result<HashMap<u32, Method>, io::Error> {
        let mut methods = HashMap::new();
        let mut offset: usize = 0x3C2;  // 定位到Methods位置,目前这是通过16进制编辑器定位的
        for i in 0..count {
            let buf = &image[offset..offset + 14];
            let token = 0x06000001 + i as u32;
            let rva = u32::from_le_bytes(buf[0..4].try_into().unwrap());

            // 读取头部信息
            let mut flags: u16;
            let header_size: u8;
            let code_size: u32;
            let max_stack: u16;
            let local_var_sig_token: u32;

            let header_start = PE::get_file_offset(rva);
            let b = image[header_start] & 7;
            match b {
                2 | 6 => {  // Tiny header. [7:2] = code size, max stack is 8, no locals or exception handlers
                    flags = 2;
                    max_stack = 8;
                    code_size = (b >> 2) as u32;
                    local_var_sig_token = 0;
                    header_size = 1;
                },
                3 => {  // Fat header. Can have locals and exception handlers
                    let header_buf = &image[header_start + 1..header_start + 12];
                    flags = (header_buf[0] as u16) << 8;
                    header_size = 4 * (flags >> 12) as u8;
                    max_stack = u16::from_le_bytes(header_buf[1..3].try_into().unwrap());
                    code_size = u32::from_le_bytes(header_buf[3..7].try_into().unwrap());
                    local_var_sig_token = u32::from_le_bytes(header_buf[7..11].try_into().unwrap());
                    
                    // The CLR allows the code to start inside the method header. But if it does,
				    // the CLR doesn't read any exceptions.
                    if (header_size < 12) {
                        flags &= 0xFFF7;
                    }
                },
                _ => panic!("Invalid method header")
            }

            // TODO: 读取locals

            methods.insert(token, Method {
                token,
                offset,
                rva,
                impl_flags: u16::from_le_bytes(buf[4..6].try_into().unwrap()),
                flags: u16::from_le_bytes(buf[6..8].try_into().unwrap()),
                name: u16::from_le_bytes(buf[8..10].try_into().unwrap()),
                signature: u16::from_le_bytes(buf[10..12].try_into().unwrap()),
                param_list: u16::from_le_bytes(buf[12..14].try_into().unwrap()),
                info: String::new(),

                max_stack,
                header_size,
                code_size,
                code_offset: header_start + header_size as usize,
            });
            offset += 14;
        }
        Ok(methods)
    }

    pub fn has_impl_flag_info(flag: u16, flag_info: ImplAttrFlagInfo) -> bool {
        match flag_info {
            ImplAttrFlagInfo::CodeType(t) => {
                flag & 0x0003 & (t as u16) != 0
            },
            ImplAttrFlagInfo::Managed(m) => {
                flag & 0x0004 & (m as u16) != 0
            },
            ImplAttrFlagInfo::CommonImplAttrFlagInfo(c) => {
                flag & (c as u16) != 0
            }
        }
    }
}

pub struct Param {
    pub token: u32,
    pub flags: u16,
    pub sequence: u16,
    pub name: u16,
}

impl Param {
    pub fn read_params(image: &Vec<u8>, count: usize) -> Result<HashMap<u32, Param>, io::Error> {
        let mut params = HashMap::new();
        let mut offset: usize = 0x3EC;  // 定位到Params位置,目前这是通过16进制编辑器定位的
        for i in 0..count {
            let buf = &image[offset..offset + 6];
            let token = 0x08000001 + i as u32;
            params.insert(token, Param {
                token,
                flags: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
                sequence: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
                name: u16::from_le_bytes(buf[4..6].try_into().unwrap()),
            });
            offset += 6;
        }
        Ok(params)
    }
}

mod PE {
    pub fn get_file_offset(rva: u32) -> usize {
        (rva - 8192 + 512) as usize  // TODO: 根据PE文件的偏移量计算
    }
}

pub struct Metadata {
    pub methods: HashMap<u32, Method>,  // <token(0x06000001...), Method>
    pub params: HashMap<u32, Param>,    // <token(0x08000001...), Param>
}

impl Metadata {
    pub fn new(mut image: &Vec<u8>) -> Metadata {
        Metadata {
            methods: Method::read_methods(&mut image, 3).unwrap(),  // TODO: 目前是3个
            params: Param::read_params(&mut image, 2).unwrap(),     // TODO: 目前是2个
        }
    }

    pub fn get_param_count(&self, method: &Method) -> u16 {
        match self.params.get(&(0x08000000 + method.param_list as u32)) {
            Some(param) => {
                let mut param_count: u16 = 1;
                let mut token = param.token + 1;
                loop {
                    match self.params.get(&token) {
                        Some(p) => {
                            if p.sequence < param.sequence {
                                return param_count;
                            }
                            param_count += 1;
                            token += 1;
                        },
                        None => return param_count
                    }
                }
            }
            None => 0
        }
    }
}