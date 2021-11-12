use std::io;
use crate::interpreter::metadata::CompressedStream;
use crate::interpreter::image_reader::ImageReader;

#[derive(Debug, Default)]
pub struct BlobStream {
    stream: CompressedStream
}

impl BlobStream {
    pub fn new(reader: &mut ImageReader, size: u32) -> io::Result<BlobStream> {
        Ok(BlobStream { 
            stream: CompressedStream::new(reader, size)? 
        })
    }

    pub fn read(&self, offset: u32) -> io::Result<Vec<u8>> {
        if offset == 0 {
            return Ok(Default::default());
        }
        let mut offset = offset as usize;
        match self.stream.try_read_compressed_u32(&mut offset) {
            Ok(len) => {
                let mut vec = vec![0; len as usize];
                vec.copy_from_slice(&self.stream.data[offset..offset + len as usize]);
                Ok(vec)
            }
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid USStream"));
            }
        }
    }
}
