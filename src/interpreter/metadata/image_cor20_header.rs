use std::io;
use crate::interpreter::data_reader::DataReader;
use super::image_data_directory::ImageDataDirectory;

pub struct ImageCor20Header {
    pub cb: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub metadata: ImageDataDirectory,
    pub flags: u32,
    pub entry_point_token: u32,
    pub resources: ImageDataDirectory,
    pub strong_name_signature: ImageDataDirectory,
    pub code_manager_table: ImageDataDirectory,
    pub vtable_fixups: ImageDataDirectory,
    pub export_address_table_jumps: ImageDataDirectory,
    pub managed_native_header: ImageDataDirectory,
}

impl ImageCor20Header {
    pub fn new(reader: &mut DataReader) -> io::Result<ImageCor20Header> {
        let cb = reader.read_u32()?;
        if cb < 0x48 {
            return Err(io::Error::new(io::ErrorKind::InvalidData,"Invalid IMAGE_COR20_HEADER.cb"));
        }
        Ok(ImageCor20Header {
            cb,
            major_runtime_version: reader.read_u16()?,
            minor_runtime_version: reader.read_u16()?,
            metadata: ImageDataDirectory::new(reader)?,
            flags: reader.read_u32()?,
            entry_point_token: reader.read_u32()?,
            resources: ImageDataDirectory::new(reader)?,
            strong_name_signature: ImageDataDirectory::new(reader)?,
            code_manager_table: ImageDataDirectory::new(reader)?,
            vtable_fixups: ImageDataDirectory::new(reader)?,
            export_address_table_jumps: ImageDataDirectory::new(reader)?,
            managed_native_header: ImageDataDirectory::new(reader)?,
        })
    }
}