use std::io;
use crate::interpreter::data_reader::DataReader;

#[derive(Debug, Default)]
pub struct BlobStream {
    stream: DataReader
}

impl BlobStream {
    pub fn new(reader: &DataReader, size: usize) -> io::Result<BlobStream> {
        Ok(BlobStream { 
            stream: reader.slice(size)? 
        })
    }

    pub fn read(&self, offset: u32) -> io::Result<Vec<u8>> {
        if offset == 0 {
            return Ok(Default::default());
        }
        let mut offset = offset as usize;
        match self.stream.try_read_compressed_u32_immut(&mut offset) {
            Ok(length) => self.stream.read_bytes_vec_exact_immut(&mut offset, length as usize),
            Err(e) => Err(e),
        }
    }
}
