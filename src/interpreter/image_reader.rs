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
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of image"));
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
        self.check_position(size)?;
        self.position += size;
        Ok(())
    }

    pub fn back(&mut self, size: usize) -> io::Result<()> {
        if self.position < size {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected ahead of image"));
        }
        self.position -= size;
        Ok(())
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

    pub fn read_bytes_exact(&mut self, array: &mut [u8], length: usize) -> io::Result<()> {
        self.check_position(length)?;
        array[..length].copy_from_slice(&self.image[self.position..self.position + length]);
        self.position += length;
        Ok(())
    }

    pub fn read_string(&mut self, max_length: usize) -> io::Result<String> {
        self.check_position(max_length)?;
        let mut string = String::new();
        for i in 0..max_length {
            let c = self.image[self.position + i];
            if c == 0 {
                break;
            }
            string.push(c as char);
        }
        self.position += max_length;
        Ok(string)
    }

    pub fn read_string_to_0(&mut self) -> io::Result<String> {
        let mut bytes_left = self.image.len() - self.position;
        let mut string = String::new();
        loop {
            let c = self.image[self.position];
            if c == 0 {
                break;
            }
            string.push(c as char);
            bytes_left -= 1;
            if bytes_left == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of image"));
            }
            self.position += 1;
        }
        self.position += 1;
        Ok(string)
    }
}