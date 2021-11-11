use std::{collections::HashMap, io};

use super::metadata::Metadata;

pub struct TypeDef {
    pub flags: u32,
    pub name: String,
    pub namespace: String,
    pub extends: u16,
    pub field_list: u16,
    pub method_list: u16,
}

impl TypeDef {
    pub fn read_type_defs(metadata: &Metadata) -> io::Result<HashMap<u32, TypeDef>> {
        let mut type_defs = HashMap::new();
        let type_def_table = &metadata.table_stream.md_tables[2];
        for row in 0..type_def_table.row_count {
            let token = 0x02000001 + row as u32;
            let flags = type_def_table.columns[0].get_cell_u32(row);
            let name = metadata.strings_stream.get_string_clone(type_def_table.columns[1].get_cell_u16(row) as u32)?;
            let namespace = metadata.strings_stream.get_string_clone(type_def_table.columns[2].get_cell_u16(row) as u32)?;
            let extends = type_def_table.columns[3].get_cell_u16(row);
            let field_list = type_def_table.columns[4].get_cell_u16(row);
            let method_list = type_def_table.columns[5].get_cell_u16(row);

            type_defs.insert(token, TypeDef { 
                flags, 
                name, 
                namespace,
                extends, 
                field_list,
                method_list,
            });
        }
        
        Ok(type_defs)
    }
}