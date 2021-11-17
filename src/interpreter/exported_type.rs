use std::io;

use crate::hash_vec::HashVec;

use super::metadata::{Metadata, md_token::{CodedToken, MDToken}, table_stream::MDType};

pub enum ExportedTypeImpl {
    File,
    AssemblyRef,
    ExportedType,
}

pub struct ExportedType {
    pub token: u32,  // 形如0x27000001
    pub flags: u32,
    pub type_def_id: u32,
    pub type_name: String,
    pub type_namespace: String,
    pub implementation_type: ExportedTypeImpl,
    pub implementation_rid: u32,
}

impl ExportedType {
    pub fn read_exported_types(metadata: &Metadata) -> io::Result<HashVec<String, ExportedType>> {
        let mut exported_types = HashVec::new();
        let exported_type_table = &metadata.table_stream.md_tables[0x27];
        for row in 0..exported_type_table.row_count {
            let flags = exported_type_table.columns[0].get_cell_u32(row);
            let type_def_id = exported_type_table.columns[1].get_cell_u32(row);
            let type_name = metadata.strings_stream.get_string_clone(exported_type_table.columns[2].get_cell_u16_or_u32(row))?;
            let type_namespace = metadata.strings_stream.get_string_clone(exported_type_table.columns[3].get_cell_u16_or_u32(row))?;
            let full_type_name = type_namespace.clone() + "." + &type_name;
            let implementation = Self::resolve_implementation(exported_type_table.columns[4].get_cell_u16_or_u32(row));

            exported_types.insert(full_type_name, ExportedType { 
                token: 0x27000001 + row as u32,
                flags,
                type_def_id,
                type_name, 
                type_namespace,
                implementation_type: implementation.0,
                implementation_rid: implementation.1,
            });
        }
        
        Ok(exported_types)
    }

    fn resolve_implementation(coded_token: u32) -> (ExportedTypeImpl, u32) {
        let token = CodedToken::from_md_type(MDType::Implementation).decode(coded_token).unwrap();
        let rid = MDToken::to_rid(token);
        match MDToken::to_table(token) {
            Some(MDType::File) => (ExportedTypeImpl::File, rid),
            Some(MDType::AssemblyRef) => (ExportedTypeImpl::AssemblyRef, rid),
            Some(MDType::ExportedType) => (ExportedTypeImpl::ExportedType, rid),
            _ => panic!("Invalid coded_token")
        }
    }
}