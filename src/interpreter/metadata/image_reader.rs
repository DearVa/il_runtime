use std::io;
use std::convert::TryInto;

pub struct ImageReader<'a> {
    image: &'a Vec<u8>,
    position: usize,
}

impl ImageReader<'_> {
    pub fn new(image: &Vec<u8>) -> ImageReader {
        ImageReader {
            image: image,
            position: 0,
        }
    }

    fn check_position(&self, size: usize) -> io::Result<()> {
        if self.position + size > self.image.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of image",
            ));
        }
        Ok(())
    }
    
    pub fn set_position(&mut self, position: usize) -> io::Result<()> {
        self.position = position;
        self.check_position(0)
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn advance(&mut self, size: usize) -> io::Result<()> {
        self.position += size;
        self.check_position(size)
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        self.check_position(1)?;
        let value = self.image[self.position];
        self.position += 1;
        Ok(value)
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        self.check_position(2)?;
        let value = u16::from_le_bytes(self.image[self.position..self.position + 2].try_into().unwrap());
        self.position += 2;
        Ok(value)
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        self.check_position(4)?;
        let value = u32::from_le_bytes(self.image[self.position..self.position + 4].try_into().unwrap());
        self.position += 4;
        Ok(value)
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        self.check_position(8)?;
        let value = u64::from_le_bytes(self.image[self.position..self.position + 8].try_into().unwrap());
        self.position += 8;
        Ok(value)
    }

    pub fn read_bytes(&mut self, array: &mut [u8]) -> io::Result<()> {
        self.check_position(array.len())?;
        array.copy_from_slice(&self.image[self.position..self.position + array.len()]);
        self.position += array.len();
        Ok(())
    }
}