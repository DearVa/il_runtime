use std::io;

use super::{metadata::*, signature::CallingConventionSig};

pub struct Field {
    /// 形如0x04000001
    pub token: u32,
    pub flags: u16,
    pub name: String,
    pub signature: Option<CallingConventionSig>,
    /// 字段所属类型，加上0x02000001就是对应的类型
    pub owner_type: u32,
}

impl Field {
    pub fn read_fields(metadata: &Metadata, field_to_type_map: Vec<u32>) -> io::Result<Vec<Field>> {
        let mut fields = Vec::new();
        let field_table = &metadata.table_stream.md_tables[4];
        let mut type_map_index = 0;
        for row in 0..field_table.row_count {
            if type_map_index < field_to_type_map.len() {
                while row + 1 >= field_to_type_map[type_map_index] {
                    type_map_index += 1;
                    if type_map_index == field_to_type_map.len() {
                        break;
                    }
                }
            }

            let flags = field_table.columns[0].get_cell_u16(row);
            let name = metadata.strings_stream.get_string_clone(field_table.columns[1].get_cell_u16_or_u32(row))?;
            let signature = CallingConventionSig::resolve_sig(metadata, field_table.columns[2].get_cell_u16_or_u32(row));

            fields.push(Field {
                token: 0x04000001 + row as u32,
                flags,
                name,
                signature,
                owner_type: type_map_index as u32 - 1,
            });
        }

        Ok(fields)
    }

    pub fn is_static(&self) -> bool {
        self.flags & 0x0010 != 0
    }
}
