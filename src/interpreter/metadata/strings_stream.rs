use std::io;
use std::collections::HashMap;
use crate::interpreter::image_reader::ImageReader;

#[derive(Default)]
pub struct StringsStream {
    strings: HashMap<u32, String>,  // offset, String
}

impl StringsStream {
    pub fn new(reader: &mut ImageReader, size: u32) -> io::Result<StringsStream> {
        let mut strings = HashMap::new();
        let start_pos = reader.get_position();
        loop {
            let offset = (reader.get_position() - start_pos) as u32;
            if offset >= size {
                break;
            }
            strings.insert(offset, reader.read_string_to_0()?);
        }
        Ok(StringsStream {
            strings,
        })
    }

    pub fn get_string(&self, position: u32) -> &String {
        &self.strings[&position]
    }
}