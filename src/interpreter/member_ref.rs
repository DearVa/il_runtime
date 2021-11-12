use std::io;

use super::metadata::Metadata;

pub struct MemberRef {
    pub token: u32,  // 形如0x0A000001
    pub class: u32,  // 指向一个表，例如0x01000001
    pub name: String,
    pub signature: u32,
}

impl MemberRef {
    pub fn read_member_refs(metadata: &Metadata) -> io::Result<Vec<MemberRef>> {
        let mut member_refs = Vec::new();
        let member_ref_table = &metadata.table_stream.md_tables[0xA];
        for row in 0..member_ref_table.row_count {
            let class = metadata.resolve_member_ref(member_ref_table.columns[0].get_cell_u16_or_u32(row)).unwrap();
            let name = metadata.strings_stream.get_string_clone(member_ref_table.columns[1].get_cell_u16_or_u32(row))?;
            let signature = member_ref_table.columns[2].get_cell_u16_or_u32(row);

            member_refs.push(MemberRef { 
                token: 0x0A000001 + row as u32,
                class,
                name, 
                signature,
            });
        }
        
        Ok(member_refs)
    }
}