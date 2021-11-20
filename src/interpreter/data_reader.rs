use std::io;
use std::convert::TryInto;

/// 一次性将整块数据传入
/// 之后可以用read方法读取数据，每次读取会自动推进position
/// 后缀immut的方法需要调用方传入position，read之后会更新position
#[derive(Debug, Default)]
pub struct DataReader {
    data: Vec<u8>,
    position: usize,
}

impl DataReader {
    pub fn new(data: Vec<u8>) -> DataReader {
        DataReader {
            data,
            position: 0,
        }
    }

    /// 将reader从self.position位置切下size大小的数据，生成一个独立的reader
    pub fn slice(&self, size: usize) -> io::Result<DataReader> {
        self.check_position(self.position, size)?;
        Ok(DataReader {
            data: self.data[self.position..(self.position + size)].to_vec(),
            position: 0,
        })
    }

    /// 将reader从position位置切下size大小的数据，生成一个独立的reader
    pub fn slice_from(&self, position: usize, size: usize) -> io::Result<DataReader> {
        self.check_position(position, size)?;
        Ok(DataReader {
            data: self.data[position..(position + size)].to_vec(),
            position: 0,
        })
    }

    pub fn check_position(&self, position: usize, size: usize) -> io::Result<()> {
        if position + size > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of image"));
        }
        Ok(())
    }
    
    pub fn set_position(&mut self, position: usize) -> io::Result<()> {
        self.check_position(position, 0)?;
        self.position = position;
        Ok(())
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn advance(&mut self, size: usize) -> io::Result<()> {
        self.check_position(self.position, size)?;
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
        let mut position = self.position;
        let result = self.read_u8_immut(&mut position);
        self.position = position;
        result
    }

    pub fn read_u8_immut(&self, position: &mut usize) -> io::Result<u8> {
        self.check_position(*position, 1)?;
        let value = self.data[*position];
        *position += 1;
        Ok(value)
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut position = self.position;
        let result = self.read_u16_immut(&mut position);
        self.position = position;
        result
    }

    pub fn read_u16_immut(&self, position: &mut usize) -> io::Result<u16> {
        self.check_position(*position, 2)?;
        let value = u16::from_le_bytes(self.data[*position..*position + 2].try_into().unwrap());
        *position += 2;
        Ok(value)
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut position = self.position;
        let result = self.read_u32_immut(&mut position);
        self.position = position;
        result
    }

    pub fn read_u32_immut(&self, position: &mut usize) -> io::Result<u32> {
        self.check_position(*position, 4)?;
        let value = u32::from_le_bytes(self.data[*position..*position + 4].try_into().unwrap());
        *position += 4;
        Ok(value)
    }

    pub fn read_f32_immut(&self, position: &mut usize) -> io::Result<f32> {
        self.check_position(*position, 4)?;
        let value = f32::from_le_bytes(self.data[*position..*position + 4].try_into().unwrap());
        *position += 4;
        Ok(value)
    }

    pub fn try_read_compressed_u32_immut(&self, position: &mut usize) -> Option<u32> {
        if self.check_position(*position, 0).is_err() {
            return None;
        }
        let b = self.read_u8_immut(position);
        if b.is_err() {
            return None;
        }
        let b = b.unwrap();
        if (b & 0x80) == 0 {
            return Some(b as u32);
        }
        if (b & 0xC0) == 0x80 {
            if self.check_position(*position, 2).is_err() {
                return None;
            }
            return Some((((b & 0x3F) as u32) << 8) | self.read_u8_immut(position).unwrap() as u32);
        }
        if self.check_position(*position, 4).is_err() {
            return None;
        }
        Some((((b & 0x1F) as u32) << 24) | (self.read_u8_immut(position).unwrap() as u32) << 16 | (self.read_u8_immut(position).unwrap() as u32) << 8 | self.read_u8_immut(position).unwrap() as u32)
    }

    pub fn try_read_compressed_i32_immut(&self, position: &mut usize) -> Option<i32> {
        if self.check_position(*position, 0).is_err() {
            return None;
        }
        let b = self.read_u8_immut(position);
        if b.is_err() {
            return None;
        }
        let b = b.unwrap();
        if (b & 0x80) == 0 {
            if (b & 1) != 0 {
                return Some(-0x40 | (b as i32 >> 1));
            } else {
                return Some(b as i32 >> 1);
            }
        }
        if (b & 0xC0) == 0x80 {
            if self.check_position(*position, 2).is_err() {
                return None;
            }
            let temp = (((b & 0x3F) as i32) << 8) | self.read_u8_immut(position).unwrap() as i32;
            if (temp & 1) != 0 {
                return Some(-0x2000 | (temp >> 1));
            } else {
                return Some(temp >> 1);
            }
        }
        if (b & 0xE0) == 0xC0 {
            if self.check_position(*position, 4).is_err() {
                return None;
            }
            let temp = (((b & 0x1F) as i32) << 24) | (self.read_u8_immut(position).unwrap() as i32) << 16 | (self.read_u8_immut(position).unwrap() as i32) << 8 | self.read_u8_immut(position).unwrap() as i32;
            if (temp & 1) != 0 {
                return Some(-0x10000000 | (temp >> 1));
            } else {
                return Some(temp >> 1);
            }
        }
        None
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut position = self.position;
        let result = self.read_u64_immut(&mut position);
        self.position = position;
        result
    }

    pub fn read_u64_immut(&self, position: &mut usize) -> io::Result<u64> {
        self.check_position(*position, 8)?;
        let value = u64::from_le_bytes(self.data[*position..*position + 8].try_into().unwrap());
        *position += 8;
        Ok(value)
    }

    pub fn read_f64_immut(&self, position: &mut usize) -> io::Result<f64> {
        self.check_position(*position, 8)?;
        let value = f64::from_le_bytes(self.data[*position..*position + 8].try_into().unwrap());
        *position += 8;
        Ok(value)
    }

    pub fn read_bytes(&mut self, array: &mut [u8]) -> io::Result<()> {
        let mut position = self.position;
        self.read_bytes_immut(&mut position, array)?;
        self.position = position;
        Ok(())
    }
    
    pub fn read_bytes_immut(&self, position: &mut usize, array: &mut [u8]) -> io::Result<()> {
        let length = array.len();
        self.check_position(*position, length)?;
        array.copy_from_slice(&self.data[*position..*position + length]);
        *position += length;
        Ok(())
    }

    pub fn read_bytes_exact(&mut self, array: &mut [u8], length: usize) -> io::Result<()> {
        let mut position = self.position;
        self.read_bytes_exact_immut(&mut position, array, length)?;
        self.position = position;
        Ok(())
    }

    pub fn read_bytes_exact_immut(&self, position: &mut usize, array: &mut [u8], length: usize) -> io::Result<()> {
        self.check_position(*position, length)?;
        array[..length].copy_from_slice(&self.data[*position..*position + length]);
        *position += length;
        Ok(())
    }

    pub fn read_bytes_vec(&mut self, vec: &mut Vec<u8>) -> io::Result<()> {
        let mut position = self.position;
        let result = self.read_bytes_vec_immut(&mut position, vec);
        self.position = position;
        result
    }

    pub fn read_bytes_vec_immut(&self, position: &mut usize, vec: &mut Vec<u8>) -> io::Result<()> {
        let length = vec.len();
        self.check_position(*position, length)?;
        vec.copy_from_slice(&self.data[*position..*position + length]);
        *position += length;
        Ok(())
    }

    pub fn read_bytes_vec_exact_immut(&self, position: &mut usize, length: usize) -> io::Result<Vec<u8>> {
        self.check_position(*position, length)?;
        let mut vec = Vec::with_capacity(length);
        vec.extend_from_slice(&self.data[*position..*position + length]);
        *position += length;
        Ok(vec)
    }

    pub fn read_string(&mut self, max_length: usize) -> io::Result<String> {
        let mut position = self.position;
        let result = self.read_string_immut(&mut position, max_length);
        self.position = position;
        result
    }

    pub fn read_string_immut(&self, position: &mut usize, max_length: usize) -> io::Result<String> {
        self.check_position(*position, max_length)?;
        let mut string = String::new();
        for i in 0..max_length {
            let c = self.data[*position + i];
            if c == 0 {
                break;
            }
            string.push(c as char);
        }
        *position += max_length;
        Ok(string)
    }

    pub fn read_string_to_0(&mut self) -> io::Result<String> {
        let mut bytes_left = self.data.len() - self.position;
        let mut string = String::new();
        loop {
            let c = self.data[self.position];
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