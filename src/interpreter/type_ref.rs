use std::io;

use crate::hash_vec::HashVec;
use super::metadata::Metadata;

pub struct TypeRef {
    pub token: u32,  // 形如0x01000001
    pub resolution_scope: u16,
    pub full_name: String,
    pub name: String,
    pub namespace: String,
}

impl TypeRef {
    pub fn read_type_refs(metadata: &Metadata) -> io::Result<HashVec<String, TypeRef>> {
        let mut type_refs = HashVec::new();
        let type_ref_table = &metadata.table_stream.md_tables[1];
        for row in 0..type_ref_table.row_count {
            let resolution_scope = type_ref_table.columns[0].get_cell_u16(row);
            let name = metadata.strings_stream.get_string_clone(type_ref_table.columns[1].get_cell_u16(row) as u32)?;
            let namespace = metadata.strings_stream.get_string_clone(type_ref_table.columns[2].get_cell_u16(row) as u32)?;
            let full_name = namespace.clone() + "." + &name;

            type_refs.insert(full_name.clone(), TypeRef { 
                token: 0x01000001 + row as u32,
                resolution_scope,
                full_name,
                name, 
                namespace,
            });
        }
        
        Ok(type_refs)
    }
}

impl Clone for TypeRef {
    fn clone(&self) -> TypeRef {
        TypeRef {
            token: self.token,
            resolution_scope: self.resolution_scope,
            full_name: self.full_name.clone(),
            name: self.name.clone(),
            namespace: self.namespace.clone(),
        }
    }
}
