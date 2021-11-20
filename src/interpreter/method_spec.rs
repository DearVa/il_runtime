use std::io;

use super::{metadata::{Metadata, md_token::CodedToken, table_stream::MDType}, calling_convention_sig::CallingConventionSig, type_sig::TypeSig};

/// 存储泛型方法的MethodSpec
#[derive(Debug)]
pub struct MethodSpec {
    /// 形如0x2B000001
    pub token: u32,
    /// method_def或者member_ref的token
    pub method: u32,
    pub instantiation: Option<CallingConventionSig>,
}

impl MethodSpec {
    pub fn read_method_specs(metadata: &Metadata) -> io::Result<Vec<MethodSpec>> {
        let mut method_specs = Vec::new();
        let method_spec_table = &metadata.table_stream.md_tables[0x2B];
        for row in 0..method_spec_table.row_count {
            let method_coded_token = method_spec_table.columns[0].get_cell_u16_or_u32(row);
            let method = CodedToken::from_md_type(MDType::MethodDefOrRef).decode(method_coded_token).unwrap();
            let instantiation = CallingConventionSig::resolve_sig(&metadata, method_spec_table.columns[1].get_cell_u16_or_u32(row));

            method_specs.push(MethodSpec { 
                token: 0x2B000001 + row as u32,
                method,
                instantiation,
            });
        }
        
        Ok(method_specs)
    }
}
