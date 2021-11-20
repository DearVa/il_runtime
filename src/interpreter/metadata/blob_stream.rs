use std::io;
use crate::interpreter::data_reader::DataReader;

/// BlobStream中，每段数据前都有一个记录长度的字节
#[derive(Debug, Default)]
pub struct BlobStream {
    reader: DataReader
}

impl BlobStream {
    pub fn new(reader: &DataReader, size: usize) -> io::Result<BlobStream> {
        Ok(BlobStream { 
            reader: reader.slice(size)? 
        })
    }

    /// 给定一个offset，读取这个Blob数据，返回Vec
    pub fn read(&self, offset: u32) -> io::Result<Vec<u8>> {
        if offset == 0 {
            return Ok(Default::default());
        }
        let mut offset = offset as usize;
        match self.reader.try_read_compressed_u32_immut(&mut offset) {
            Some(length) => self.reader.read_bytes_vec_exact_immut(&mut offset, length as usize),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid offset")),
        }
    }

    /// 给定一个offset，返回一个用于读取这一段Blob数据的DataReader
    pub fn create_reader(&self, offset: u32) -> io::Result<DataReader> {
        if offset == 0 {
            return Ok(Default::default());
        }
        let mut offset = offset as usize;
        match self.reader.try_read_compressed_u32_immut(&mut offset) {
            Some(length) => self.reader.slice_from(offset, length as usize),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid offset")),
        }
    }
}
