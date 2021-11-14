use std::io;
use crate::interpreter::data_reader::DataReader;

pub struct StreamHeader {
    pub offset: u32,
    pub size: u32,
    pub name: String,
}

impl StreamHeader {
    pub fn new(reader: &mut DataReader) -> io::Result<StreamHeader> {
        let offset = reader.read_u32()?;
        let size = reader.read_u32()?;

        let origin_pos = reader.get_position();
        let mut name = String::new();
        let mut i = 0;
        loop {
            let b = reader.read_u8()?;
            if b == 0 {
                break;
            }
            name.push(b as char);
            i += 1;
            if i == 32 {
                break;
            }
        }
        if i != 32 {
            reader.set_position(origin_pos + (i + 4) & !3)?;
        }
        Ok(StreamHeader {
            offset,
            size,
            name,
        })
    }
}

pub struct MetadataHeader {
    pub major_version: u16,
    pub minor_version: u16,
    pub reserved1: u32,
    pub string_length: u32,
    pub version_string: String,
    pub offset_2nd_part: u32,
    pub flags: u8,
    pub reserved2: u8,
    pub streams: u16,
    pub stream_headers: Vec<StreamHeader>,
}

impl MetadataHeader {
    pub fn new(reader: &mut DataReader) -> io::Result<MetadataHeader> {
        let signature = reader.read_u32()?;
        if signature != 0x424A5342 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid metadata header signature"));
        }
        let string_length: u32;
        let streams: u16;
        Ok(MetadataHeader {
            major_version: reader.read_u16()?,
            minor_version: reader.read_u16()?,
            reserved1: reader.read_u32()?,
            string_length: {
                string_length = reader.read_u32()?;
                string_length
            },
            version_string: reader.read_string(string_length as usize)?,
            offset_2nd_part: reader.get_position() as u32,
            flags: reader.read_u8()?,
            reserved2: reader.read_u8()?,
            streams: {
                streams = reader.read_u16()?;
                streams
            },
            stream_headers: {
                let mut stream_headers = Vec::with_capacity(streams as usize);
                for _ in 0..streams {
                    stream_headers.push(StreamHeader::new(reader)?);
                }
                stream_headers
            },
        })
    } 
}