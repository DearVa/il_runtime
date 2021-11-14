use std::io;
use crate::interpreter::data_reader::DataReader;

#[derive(Debug, Default, Copy, Clone)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

impl ImageDataDirectory {
    pub fn new(reader: &mut DataReader) -> Result<ImageDataDirectory, io::Error> {
        let virtual_address = reader.read_u32()?;
        let size = reader.read_u32()?;
        Ok(ImageDataDirectory {
            virtual_address,
            size,
        })
    }
}