use std::io;

use crate::interpreter::{CallingConventionSig, metadata::md_token::MDToken};
use super::{Assembly, RidList, data_reader::DataReader, metadata::*};

pub struct Method {
    pub token: u32,                 // 形如0x06000001
    pub rva: u32,                   // 方法入口在image中的偏移
    pub impl_flags: u16,            // 实现标志
    pub attributes: u16,            // 方法属性，比如是否为static，virtual，abstract等
    pub flags: u16,                 // 方法标志，这和MDTable中的flag不同
    pub name: String,               // 方法名
    pub signature: Option<CallingConventionSig>, // 签名
    pub param_list: RidList,        // 参数列表，对应ParamTable
    pub owner_type: u32,            // 方法所属类型，加上0x02000001就是对应的类型

    pub max_stack: u16,             // 最大堆栈大小
    pub header_size: u8,            // 方法头大小
    pub code_size: u32,             // 代码大小
    pub local_var_rid: u32,         // 局部变量Signature

    pub header_position: usize,     // MethodHeader在Image中的真实位置
    pub code_position: usize,       // IL指令在Image中的真实位置
}

impl Method {
    pub fn read_methods(pe: &PE, metadata: &Metadata, method_to_type_map: Vec<u32>, reader: &mut DataReader) -> io::Result<Vec<Method>> {
        let mut methods = Vec::new();
        let method_table = &metadata.table_stream.md_tables[6];
        let mut type_map_index = 0;
        for row in 0..method_table.row_count {
            if type_map_index < method_to_type_map.len() {
                while row + 1 >= method_to_type_map[type_map_index] {
                    type_map_index += 1;
                    if type_map_index == method_to_type_map.len() {
                        break;
                    }
                }
            }

            let mut flags: u16;
            let header_size: u8;
            let code_size: u32;
            let max_stack: u16;
            let local_var_rid: u32;
            let header_position: usize;

            let rva = method_table.columns[0].get_cell_u32(row);  // RVA记录了方法具体实现的字节码位置
            if rva == 0 {  // 有些方法有记录但是没有实际的实现（例如InternalCall或者P/Invoke），此时RVA为0
                flags = 0;
                header_size = 0;
                code_size = 0;
                max_stack = 0;
                local_var_rid = 0;
                header_position = 0;
            } else {
                header_position = pe.rva_to_file_offset(rva);
                reader.set_position(header_position)?;
    
                let b = reader.read_u8()? & 7;
                match b {
                    2 | 6 => {  // Tiny header. [7:2] = code size, max stack is 8, no locals or exception handlers
                        flags = 2;
                        max_stack = 8;
                        code_size = (b >> 2) as u32;
                        local_var_rid = 0;
                        header_size = 1;
                    },
                    3 => {  // Fat header. Can have locals and exception handlers
                        flags = (reader.read_u8()? as u16) << 8;
                        header_size = 4 * (flags >> 12) as u8;
                        max_stack = reader.read_u16()?;
                        code_size = reader.read_u32()?;
                        local_var_rid = MDToken::to_rid(reader.read_u32()?);
                        
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
            }
            // TODO: 读取locals

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
                owner_type: type_map_index as u32 - 1,

                max_stack,
                header_size,
                code_size,
                local_var_rid,

                header_position,
                code_position: header_position + header_size as usize,
            });
        }

        Ok(methods)
    }

    pub fn check_impl_flag_info(&self, flag_info: ImplAttrFlagInfo) -> bool {
        match flag_info {
            ImplAttrFlagInfo::CodeType(t) => {
                self.impl_flags & 0x0003 & (t as u16) != 0
            },
            ImplAttrFlagInfo::Managed(m) => {
                self.impl_flags & 0x0004 & (m as u16) != 0
            },
            ImplAttrFlagInfo::CommonImplAttrFlagInfo(c) => {
                self.impl_flags & (c as u16) != 0
            }
        }
    }

    pub fn to_string(&self, assembly: &Assembly) -> String {
        match &self.signature {
            Some(CallingConventionSig::MethodSig(method)) => {
                let owner_type = assembly.type_defs.index_get(self.owner_type as usize).unwrap();
                let owner_type_full_name = owner_type.namespace.clone() + "." + &owner_type.name;
                format!("{} {}.{}{}", method.get_ret_type_string(), owner_type_full_name, self.name, method.get_params_type_string())
            },
            _ => panic!("Not a method!")
        }
    }

    pub fn is_static(&self) -> bool {
        self.attributes & 0x10 != 0
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