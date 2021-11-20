use std::io;

use super::{metadata::Metadata, type_sig::TypeSig};

#[derive(Debug)]
pub struct TypeSpec {
    pub token: u32,  // 形如0x1B000001
    pub signature: Option<TypeSig>,
}

impl TypeSpec {
    pub fn read_type_specs(metadata: &Metadata) -> io::Result<Vec<TypeSpec>> {
        let mut type_specs = Vec::new();
        let type_spec_table = &metadata.table_stream.md_tables[0x1B];
        for row in 0..type_spec_table.row_count {
            let signature = type_spec_table.columns[0].get_cell_u16_or_u32(row);
            if signature == 6743 {
                println!();
            }
            let signature = TypeSig::resolve_sig(&metadata, signature);

            type_specs.push(TypeSpec { 
                token: 0x1B000001 + row as u32,
                signature,
            });
        }
        
        Ok(type_specs)
    }
}
