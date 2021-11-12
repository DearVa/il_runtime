use std::io;
use crate::interpreter::image_reader::ImageReader;

#[derive(Debug, Default)]
pub struct CompressedStream {
    pub data: Vec<u8>,  // 原始数据
}

impl CompressedStream {
    pub fn new(reader: &mut ImageReader, size: u32) -> io::Result<CompressedStream> {
        let mut data = vec![0; size as usize];
        reader.read_bytes_vec(&mut data)?;

        Ok(CompressedStream { 
            data 
        })
    }

    pub fn try_read_compressed_u32(&self, offset: &mut usize) -> io::Result<u32> {
        let bytes_left = self.data.len() - *offset;
        if bytes_left == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of image"));
        }
        let b = self.read_u8(offset)?;
        if (b & 0x80) == 0 {
            return Ok(b as u32);
        }
        if (b & 0xC0) == 0x80 {
            if bytes_left < 2 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of image"));
            }
            return Ok((((b & 0x3F) as u32) << 8) | self.read_u8(offset)? as u32);
        }
        if bytes_left < 4 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of image"));
        }
        Ok((((b & 0x1F) as u32) << 24) | (self.read_u8(offset)? as u32) << 16 | (self.read_u8(offset)? as u32) << 8 | self.read_u8(offset)? as u32)
    }

    pub fn read_u8(&self, offset: &mut usize) -> io::Result<u8> {
        if *offset >= self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of image"));
        }
        let b = self.data[*offset];
        *offset += 1;
        Ok(b)
    }
}