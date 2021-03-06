use std::io;

use super::metadata::Metadata;

pub struct ModuleRef {
    pub token: u32,  // 形如0x01000001
    pub resolution_scope: u16,
    pub name: String,
    pub namespace: String,
}

impl ModuleRef {
    pub fn read_type_refs(metadata: &Metadata) -> io::Result<Vec<ModuleRef>> {
        let mut type_refs = Vec::new();
        let type_ref_table = &metadata.table_stream.md_tables[1];
        for row in 0..type_ref_table.row_count {
            let resolution_scope = type_ref_table.columns[0].get_cell_u16(row);
            let name = metadata.strings_stream.get_string_clone(type_ref_table.columns[1].get_cell_u16(row) as u32)?;
            let namespace = metadata.strings_stream.get_string_clone(type_ref_table.columns[2].get_cell_u16(row) as u32)?;

            type_refs.push(ModuleRef { 
                token: 0x01000001 + row as u32,
                resolution_scope, 
                name, 
                namespace 
            });
        }
        
        Ok(type_refs)
    }
}