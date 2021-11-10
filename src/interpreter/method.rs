use std::{collections::HashMap, io};

use super::{image_reader::ImageReader, metadata::*, RidList};

#[derive(Debug)]
pub struct Method {
    pub token: u32,                 // 和字典Key一致
    pub rva: u32,                   // 函数入口在image中的偏移
    pub impl_flags: u16,            // 实现标志
    pub flags: u16,                 // 函数标志，这和MDTable中的flag不同
    pub name: String,               // 函数名
    pub signature: u16,             // 签名
    pub param_list: RidList,        // 参数列表，对应ParamTable

    pub max_stack: u16,             // 最大堆栈大小
    pub header_size: u8,            // 函数头大小
    pub code_size: u32,             // 代码大小
    pub local_var_sig_token: u32,   // 局部变量Signature

    pub header_position: usize,     // MethodHeader在Image中的真实位置
    pub code_position: usize,       // IL指令在Image中的真实位置
}

impl Method {
    pub fn read_methods(pe: &PE, metadata: &Metadata, reader: &mut ImageReader) -> io::Result<HashMap<u32, Method>> {
        let mut methods = HashMap::new();
        let method_table = &metadata.table_stream.md_tables[6];
        for row in 0..method_table.row_count {
            let token = 0x06000001 + row as u32;
            let rva = method_table.columns[0].get_cell_u32(row);
            let header_position = pe.rva_to_file_offset(rva);
            reader.set_position(header_position)?;

            let mut flags: u16;
            let header_size: u8;
            let code_size: u32;
            let max_stack: u16;
            let local_var_sig_token: u32;
            let b = reader.read_u8()? & 7;
            match b {
                2 | 6 => {  // Tiny header. [7:2] = code size, max stack is 8, no locals or exception handlers
                    flags = 2;
                    max_stack = 8;
                    code_size = (b >> 2) as u32;
                    local_var_sig_token = 0;
                    header_size = 1;
                },
                3 => {  // Fat header. Can have locals and exception handlers
                    flags = (reader.read_u8()? as u16) << 8;
                    header_size = 4 * (flags >> 12) as u8;
                    max_stack = reader.read_u16()?;
                    code_size = reader.read_u32()?;
                    local_var_sig_token = reader.read_u32()?;
                    
                    // The CLR allows the code to start inside the method header. But if it does,
				    // the CLR doesn't read any exceptions.
                    reader.back(12)?;
                    reader.advance(header_size as usize)?;
                    if header_size < 12 {
                        flags &= 0xFFF7;
                    }
                },
                _ => panic!("Invalid method header")
            }

            // TODO: 读取locals

            methods.insert(token, Method {
                token,
                rva,
                impl_flags: method_table.columns[1].get_cell_u16(row),
                flags,
                name: metadata.strings_stream.get_string_clone(method_table.columns[3].get_cell_u16(row) as u32)?.clone(),
                signature: method_table.columns[4].get_cell_u16(row),
                param_list: metadata.get_param_rid_list(row + 1),

                max_stack,
                header_size,
                code_size,
                local_var_sig_token,

                header_position,
                code_position: header_position + header_size as usize,
            });
        }

        Ok(methods)
    }

    pub fn check_impl_flag_info(flag: u16, flag_info: ImplAttrFlagInfo) -> bool {
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

pub enum CodeType {
    IL,
    Native,
    OPTIL,
    Runtime
}

pub enum Managed {
    Managed,
    Unmanaged
}

pub enum CommonImplAttrFlagInfo {
    NoInlining = 0x0008,
    ForwardRef = 0x0010,
    Synchronized = 0x0020,
    NoOptimization = 0x0040,
    PreserveSig = 0x0080,
    AggressiveInlining = 0x0100,
    AggressiveOptimization = 0x0200,
    SecurityMitigations = 0x0400,
    InternalCall = 0x1000
}

pub enum ImplAttrFlagInfo {
    CodeType(CodeType),
    Managed(Managed),
    CommonImplAttrFlagInfo(CommonImplAttrFlagInfo)
}