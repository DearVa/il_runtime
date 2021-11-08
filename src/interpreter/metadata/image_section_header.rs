use std::io;
use super::image_reader::*;

#[derive(Debug, Copy, Clone)]
pub struct ImageSectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_line_numbers: u32,
    pub number_of_relocations: u16,
    pub number_of_line_numbers: u16,
    pub characteristics: u32,
}

impl ImageSectionHeader {
    pub fn new(reader: &mut ImageReader) -> io::Result<ImageSectionHeader> {
        let mut name = [0u8; 8];
        reader.read_bytes(&mut name)?;
        Ok(ImageSectionHeader {
            name,
            virtual_size: reader.read_u32()?,
            virtual_address: reader.read_u32()?,
            size_of_raw_data: reader.read_u32()?,
            pointer_to_raw_data: reader.read_u32()?,
            pointer_to_relocations: reader.read_u32()?,
            pointer_to_line_numbers: reader.read_u32()?,
            number_of_relocations: reader.read_u16()?,
            number_of_line_numbers: reader.read_u16()?,
            characteristics: reader.read_u32()?,
        })
    }
}