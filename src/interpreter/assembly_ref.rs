use std::io;

use super::{assembly_name::AssemblyName, metadata::Metadata};

pub struct AssemblyRef {
    pub token: u32,               // 形如 0x230000001
    pub assembly_name: AssemblyName,
    pub locale: u16,
    pub hash_value: u16,
}

impl AssemblyRef {
    pub fn read_assembly_refs(metadata: &Metadata) -> io::Result<Vec<AssemblyRef>> {
        let mut assembly_refs = Vec::new();
        let assembly_ref_table = &metadata.table_stream.md_tables[0x23];
        for row in 0..assembly_ref_table.row_count {
            let major_version = assembly_ref_table.columns[0].get_cell_u16(row);
            let minor_version = assembly_ref_table.columns[1].get_cell_u16(row);
            let build_number = assembly_ref_table.columns[2].get_cell_u16(row);
            let revision_number = assembly_ref_table.columns[3].get_cell_u16(row);
            let flags = assembly_ref_table.columns[4].get_cell_u32(row);
            let public_key_token = metadata.blob_stream.read(assembly_ref_table.columns[5].get_cell_u16(row) as u32)?;
            let locale = assembly_ref_table.columns[7].get_cell_u16(row);
            let name = metadata.strings_stream.get_string_clone(assembly_ref_table.columns[6].get_cell_u16_or_u32(row))?;
            let hash_value = assembly_ref_table.columns[8].get_cell_u16(row);

            assembly_refs.push(AssemblyRef { 
                token: 0x23000001 + row as u32,
                assembly_name: AssemblyName {
                    major_version,
                    minor_version,
                    build_number,
                    revision_number,
                    flags,
                    public_key_token,
                    name,
                },
                locale,
                hash_value,
            });
        }
        
        Ok(assembly_refs)
    }
}