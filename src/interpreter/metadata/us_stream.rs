use std::io;
use crate::interpreter::metadata::CompressedStream;
use crate::interpreter::image_reader::ImageReader;

#[derive(Debug, Default)]
pub struct USStream {
    stream: CompressedStream
}

impl USStream {
    pub fn new(reader: &mut ImageReader, size: u32) -> io::Result<USStream> {
        Ok(USStream { 
            stream: CompressedStream::new(reader, size)? 
        })
    }

    pub fn read(&self, offset: u32) -> io::Result<String> {
        if offset == 0 {
            return Ok(Default::default());
        }
        let mut offset = offset as usize;
        match self.stream.try_read_compressed_u32(&mut offset) {
            Ok(len) => {
                Ok(self.read_utf16_string(offset, len as usize)?)
            }
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid USStream"));
            }
        }
    }

    fn read_utf16_string(&self, offset: usize, length: usize) -> io::Result<String> {
        if offset + length > self.stream.data.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected end of USStream"));
        }
        let array = Vec::from(&self.stream.data[offset..offset + length]);
        let array: Vec<u16> = array.chunks_exact(2).into_iter().map(|a| u16::from_ne_bytes([a[0], a[1]])).collect();
        Ok(String::from_utf16_lossy(array.as_slice()))
    }
}