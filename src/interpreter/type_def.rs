use std::io;
use likely_stable::unlikely;

use crate::hash_vec::HashVec;
use super::metadata::{Metadata, RidList, md_token::CodedToken, table_stream::MDType};

pub struct TypeDef {
    pub token: u32,  // 形如0x02000001
    pub flags: u32,
    pub name: String,
    pub namespace: String,
    pub extends: u32,
    pub field_list: RidList,
    pub method_list: RidList,
}

impl TypeDef {
    pub fn read_type_defs(metadata: &Metadata) -> io::Result<HashVec<String, TypeDef>> {
        let mut type_defs = HashVec::new();
        let type_def_table = &metadata.table_stream.md_tables[2];
        for row in 0..type_def_table.row_count {
            let flags = type_def_table.columns[0].get_cell_u32(row);
            let name = metadata.strings_stream.get_string_clone(type_def_table.columns[1].get_cell_u16_or_u32(row))?;
            let namespace = metadata.strings_stream.get_string_clone(type_def_table.columns[2].get_cell_u16_or_u32(row))?;
            let extends = CodedToken::from_md_type(MDType::TypeDefOrRef).decode(type_def_table.columns[3].get_cell_u16(row) as u32).unwrap();
            let field_list = metadata.get_field_rid_list(row + 1);
            let method_list = metadata.get_method_rid_list(row + 1);

            let full_name = namespace.clone() + "." + &name;
            let type_def = TypeDef { 
                token: 0x02000001 + row as u32,
                flags, 
                name, 
                namespace,
                extends,
                field_list,
                method_list,
            };

            if unlikely(type_defs.contains(&full_name)) {  // 有些时候可能存在相同的fullname（就nm离谱）
                let mut final_full_name;
                let mut post_fix: u32 = 0;
                loop {
                    post_fix += 1;
                    final_full_name = full_name.clone();
                    final_full_name.push_str(&format!("`{}", post_fix));
                    if !type_defs.contains(&final_full_name) {
                        break;
                    }
                }
                println!("Same Type_def!! {}", final_full_name);
                type_defs.insert(final_full_name, type_def);
            } else {
                type_defs.insert(full_name, type_def);
            }
        }
        
        Ok(type_defs)
    }
}