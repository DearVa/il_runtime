use std::io;

use super::metadata::*;

#[derive(Debug)]
pub struct Param {
    pub token: u32,  // 形如0x08000001
    pub flags: u16,
    pub sequence: u16,
    pub name: String,
}

impl Param {
    pub fn read_params(metadata: &Metadata) -> io::Result<Vec<Param>> {
        let mut params = Vec::new();
        let param_table = &metadata.table_stream.md_tables[8];
        for row in 0..param_table.row_count {
            let flags = param_table.columns[0].get_cell_u16(row);
            let sequence = param_table.columns[1].get_cell_u16(row);
            let name = metadata.strings_stream.get_string_clone(param_table.columns[2].get_cell_u16_or_u32(row))?;

            params.push(Param {
                token: 0x08000001 + row as u32,
                flags,
                sequence,
                name,
            });
        }

        Ok(params)
    }
}
