use std::{collections::HashMap, io};

use super::metadata::{Metadata, RidList};

pub struct TypeDef {
    pub token: u32,  // 形如0x02000001
    pub flags: u32,
    pub name: String,
    pub namespace: String,
    pub extends: u16,
    pub field_list: u16,
    pub method_list: RidList,
}

impl TypeDef {
    pub fn read_type_defs(metadata: &Metadata) -> io::Result<HashMap<String, TypeDef>> {
        let mut type_defs = HashMap::new();
        let type_def_table = &metadata.table_stream.md_tables[2];
        for row in 0..type_def_table.row_count {
            let flags = type_def_table.columns[0].get_cell_u32(row);
            let name = metadata.strings_stream.get_string_clone(type_def_table.columns[1].get_cell_u16_or_u32(row))?;
            let namespace = metadata.strings_stream.get_string_clone(type_def_table.columns[2].get_cell_u16_or_u32(row))?;
            let extends = type_def_table.columns[3].get_cell_u16(row);
            let field_list = type_def_table.columns[4].get_cell_u16(row);
            let method_list = metadata.get_method_rid_list(row + 1);
            let full_name = namespace.clone() + "." + &name;

            type_defs.insert(full_name, TypeDef { 
                token: 0x02000001 + row as u32,
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

    // 当其他Assembly调用改Assembly成员时，使用该方法解析
    // pub fn resolve(member_ref: &MemberRef) {

    // }
}