use std::io;
use std::collections::HashMap;
use crate::interpreter::data_reader::DataReader;

#[derive(Default)]
pub struct StringsStream {
    strings: HashMap<u32, String>,  // offset, String
    data: Vec<u8>,                  // 原始数据  
}

impl StringsStream {
    pub fn new(reader: &mut DataReader, size: u32) -> io::Result<StringsStream> {
        let mut strings = HashMap::new();
        let start_pos = reader.get_position();
        loop {
            let offset = (reader.get_position() - start_pos) as u32;
            if offset >= size {
                break;
            }
            strings.insert(offset, reader.read_string_to_0()?);
        }

        let mut data = vec![0; size as usize];
        reader.set_position(start_pos)?;
        reader.read_bytes_vec(&mut data)?;

        Ok(StringsStream {
            strings,
            data,
        })
    }

    pub fn get_string_clone(&self, offset: u32) -> io::Result<String> {
        match self.strings.get(&offset) {
            Some(s) => Ok(s.clone()),
            None => self.read_string_to_0(offset)
        }
    }

    fn read_string_to_0(&self, mut offset: u32) -> io::Result<String> {
        let mut bytes_left = self.data.len() - offset as usize;
        let mut string = String::new();
        loop {
            let c = self.data[offset as usize];
            if c == 0 {
                break;
            }
            string.push(c as char);
            bytes_left -= 1;
            if bytes_left == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of image"));
            }
            offset += 1;
        }
        Ok(string)
    }
}