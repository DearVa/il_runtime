use std::io;
use super::image_reader::*;

#[derive(Default, Copy, Clone)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

impl ImageDataDirectory {
    pub fn new(reader: &mut ImageReader) -> Result<ImageDataDirectory, io::Error> {
        let virtual_address = reader.read_u32()?;
        let size = reader.read_u32()?;
        Ok(ImageDataDirectory {
            virtual_address,
            size,
        })
    }
}

const DATA_DIR_COUNT: usize = 16;

pub struct ImageOptionHeader {
    pub start_offset: usize,
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [ImageDataDirectory; DATA_DIR_COUNT],
}

impl ImageOptionHeader {
    pub fn new(reader: &mut ImageReader, total_size: u32) -> Result<ImageOptionHeader, io::Error> {
        let start_offset = reader.get_position();
        let magic = reader.read_u16()?;
        let is64 = match magic {
            0x010B => {
                // PE 32
                false
            },
            0x020B => {
                // PE 64
                true
            },
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid optional header magic"));
            }
        };
        if (is64 && total_size < 0x70) || (!is64 && total_size < 0x60) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid optional header size"));
        }
        Ok(ImageOptionHeader {
            start_offset,
            magic: reader.read_u16()?,
            major_linker_version: reader.read_u8()?,
            minor_linker_version: reader.read_u8()?,
            size_of_code: reader.read_u32()?,
            size_of_initialized_data: reader.read_u32()?,
            size_of_uninitialized_data: reader.read_u32()?,
            address_of_entry_point: reader.read_u32()?,
            base_of_code: reader.read_u32()?,
            base_of_data: {
                if is64 {
                    reader.read_u32()?
                } else {
                    0
                }
            },
            image_base: reader.read_u64()?,
            section_alignment: reader.read_u32()?,
            file_alignment: reader.read_u32()?,
            major_operating_system_version: reader.read_u16()?,
            minor_operating_system_version: reader.read_u16()?,
            major_image_version: reader.read_u16()?,
            minor_image_version: reader.read_u16()?,
            major_subsystem_version: reader.read_u16()?,
            minor_subsystem_version: reader.read_u16()?,
            win32_version_value: reader.read_u32()?,
            size_of_image: reader.read_u32()?,
            size_of_headers: reader.read_u32()?,
            check_sum: reader.read_u32()?,
            subsystem: reader.read_u16()?,
            dll_characteristics: reader.read_u16()?,
            size_of_stack_reserve: reader.read_u64()?,
            size_of_stack_commit: reader.read_u64()?,
            size_of_heap_reserve: reader.read_u64()?,
            size_of_heap_commit: reader.read_u64()?,
            loader_flags: reader.read_u32()?,
            number_of_rva_and_sizes: reader.read_u32()?,
            data_directory: {
                let mut data_directories = [Default::default(); DATA_DIR_COUNT];
                for i in 0..DATA_DIR_COUNT {
                    let len = reader.get_position() - start_offset;
                    if len + 8 <= total_size as usize {
                        data_directories[i] = ImageDataDirectory::new(reader)?;
                    } else {
                        break;
                    }
                }
                data_directories
            },
        })
    }
}