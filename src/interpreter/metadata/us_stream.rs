use std::io;
use crate::interpreter::data_reader::DataReader;

#[derive(Debug, Default)]
pub struct USStream {
    reader: DataReader
}

impl USStream {
    pub fn new(reader: &mut DataReader, size: usize) -> io::Result<USStream> {
        Ok(USStream { 
            reader: reader.slice(size)? 
        })
    }

    pub fn read(&self, offset: usize) -> io::Result<String> {
        if offset == 0 {
            return Ok(Default::default());
        }
        let mut offset = offset as usize;
        match self.reader.try_read_compressed_u32_immut(&mut offset) {
            Some(len) => Ok(self.read_utf16_string(offset, len as usize)?),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid USStream")),
        }
    }

    fn read_utf16_string(&self, offset: usize, length: usize) -> io::Result<String> {
        self.reader.check_position(offset, length)?;
        let mut offset = offset;
        let vec = self.reader.read_bytes_vec_exact_immut(&mut offset, length)?;
        let vec: Vec<u16> = vec.chunks_exact(2).into_iter().map(|a| u16::from_ne_bytes([a[0], a[1]])).collect();
        Ok(String::from_utf16_lossy(vec.as_slice()))
    }
}