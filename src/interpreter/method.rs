use std::{fmt::{self, Debug, Formatter}, io};

use crate::interpreter::CallingConventionSig;

use super::{data_reader::DataReader, metadata::*, RidList};

pub struct Method {
    pub token: u32,                 // 形如0x06000001
    pub rva: u32,                   // 方法入口在image中的偏移
    pub impl_flags: u16,            // 实现标志
    pub attributes: u16,            // 方法属性，比如是否为static，virtual，abstract等
    pub flags: u16,                 // 方法标志，这和MDTable中的flag不同
    pub name: String,               // 方法名
    pub signature: Option<CallingConventionSig>, // 签名
    pub param_list: RidList,        // 参数列表，对应ParamTable
    pub owner_type: u32,            // 方法所属类型，加上0x06000001就是对应的方法

    pub max_stack: u16,             // 最大堆栈大小
    pub header_size: u8,            // 方法头大小
    pub code_size: u32,             // 代码大小
    pub local_var_sig_token: u32,   // 局部变量Signature

    pub header_position: usize,     // MethodHeader在Image中的真实位置
    pub code_position: usize,       // IL指令在Image中的真实位置
}

impl Method {
    pub fn read_methods(pe: &PE, metadata: &Metadata, method_to_type_map: Vec<u32>, reader: &mut DataReader) -> io::Result<Vec<Method>> {
        let mut methods = Vec::new();
        let method_table = &metadata.table_stream.md_tables[6];
        let mut type_map_index = 0;
        for row in 0..method_table.row_count {
            let rid = row + 1;

            if type_map_index < method_to_type_map.len() {
                while rid >= method_to_type_map[type_map_index] {
                    type_map_index += 1;
                    if type_map_index == method_to_type_map.len() {
                        break;
                    }
                }
            }

            let rva = method_table.columns[0].get_cell_u32(row);
            if rva == 0 {
                continue;
            }
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

            let owner_type_rid = type_map_index as u32;
            methods.push(Method {
                token: 0x06000001 + row as u32,
                rva,
                impl_flags: method_table.columns[1].get_cell_u16(row),
                attributes: method_table.columns[2].get_cell_u16(row),
                flags,
                // namespace: metadata.table_stream.md_tables[2].get_cell_string(method_table.columns[3].get_cell_u32(row))?,
                name: metadata.strings_stream.get_string_clone(method_table.columns[3].get_cell_u16_or_u32(row))?,
                signature: CallingConventionSig::read_metadata_sig(metadata, method_table.columns[4].get_cell_u16_or_u32(row)),
                param_list: metadata.get_param_rid_list(row + 1),
                owner_type: owner_type_rid - 1,

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

    pub fn is_static(&self) -> bool {
        self.attributes & 0x10 != 0
    }
}

impl Debug for Method {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.signature {
            Some(CallingConventionSig::MethodSig(method)) => {
                write!(f, "{} {}{}", method.get_ret_type_string(), self.name, method.get_params_type_string())?;
            },
            _ => {
                write!(f, "Method: {}", self.name)?;
            },
        }
        Ok(())
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