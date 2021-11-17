use std::io;

use crate::interpreter::CallingConventionSig;
use super::metadata::Metadata;

pub struct StandaloneSig {
    pub token: u32,  // 形如0x01000001
    pub signature: Option<CallingConventionSig>,
}

impl StandaloneSig {
    pub fn read_standalone_sigs(metadata: &Metadata) -> io::Result<Vec<StandaloneSig>> {
        let mut standalone_sigs = Vec::new();
        let standalone_sig_table = &metadata.table_stream.md_tables[0x11];
        for row in 0..standalone_sig_table.row_count {
            let signature = CallingConventionSig::read_metadata_sig(metadata, standalone_sig_table.columns[0].get_cell_u16_or_u32(row));

            standalone_sigs.push(StandaloneSig { 
                token: 0x11000001 + row as u32,
                signature,
            });
        }
        
        Ok(standalone_sigs)
    }
}
