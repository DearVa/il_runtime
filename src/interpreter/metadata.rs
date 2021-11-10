mod image_data_directory;
mod image_option_header;
use image_option_header::*;
mod image_section_header;
use image_section_header::*;
mod image_cor20_header;
use image_cor20_header::*;
mod metadata_header;
use metadata_header::*;
mod table_stream;
use table_stream::*;
mod strings_stream;
use strings_stream::*;
use super::ImageReader;

use std::io;

#[derive(Debug)]
pub struct PE {
    nt_headers_offset: usize,
    machine: u16,
    num_of_sections: u16,
    timestamp: u32,
    pointer_to_symbol_table: u32,
    num_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
    image_option_header: ImageOptionHeader,
    image_section_headers: Vec<ImageSectionHeader>,
}

impl PE {
    pub fn new(reader: &mut ImageReader) -> io::Result<PE> {
        // 读DOS头
        let pe_sig = reader.read_u16()?;
        if pe_sig != 0x5A4D {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid DOS signature"));
        }
        reader.set_position(0x3C)?;
        let nt_headers_offset = reader.read_u32()? as usize;
        
        // 读NT头
        reader.set_position(nt_headers_offset)?;
        let nt_headers_sig = reader.read_u32()?;
        if nt_headers_sig != 0x4550 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid NT headers signature"));
        }
        let machine = reader.read_u16()?;
        let mut num_of_sections = reader.read_u16()?;
        let timestamp = reader.read_u32()?;
        let pointer_to_symbol_table  = reader.read_u32()?;
        let num_of_symbols = reader.read_u32()?;
        let size_of_optional_header = reader.read_u16()?;
        let characteristics = reader.read_u16()?;
        if size_of_optional_header == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid SizeOfOptionalHeader"));
        }

        // 读optional_header
        let image_option_header = ImageOptionHeader::new(reader, size_of_optional_header as u32)?;
        
        // 读secions
        reader.set_position(image_option_header.start_offset + size_of_optional_header as usize)?;
        if num_of_sections > 0 {
            let position = reader.get_position();
            reader.advance(0x14)?;
            let first_section_offset = reader.read_u32()? as usize;
            num_of_sections = u16::min(((first_section_offset - reader.get_position()) / 0x28) as u16, num_of_sections);
            reader.set_position(position)?;
        }
        let mut image_section_headers = Vec::with_capacity(num_of_sections as usize);
        for _ in 0..num_of_sections {
            image_section_headers.push(ImageSectionHeader::new(reader)?);
        }

        Ok(PE {
            nt_headers_offset,
            machine,
            num_of_sections,
            timestamp,
            pointer_to_symbol_table,
            num_of_symbols,
            size_of_optional_header,
            characteristics,
            image_option_header,
            image_section_headers,
        })
    }

    pub fn rva_to_file_offset(&self, rva: u32) -> usize {
        for secion in self.image_section_headers.iter() {
            if rva >= secion.virtual_address && rva < secion.virtual_address + secion.virtual_size {
                return (rva - secion.virtual_address + secion.pointer_to_raw_data) as usize;
            }
        }
        rva as usize
    }
}

#[derive(Debug, Default)]
pub struct RidList {
    pub start_rid: u32,
    pub count: u32,
    pub rids: Option<Vec<u32>>,
}

impl RidList {
    pub fn create(start_row: u32, row_count: u32) -> RidList {
        RidList {
            start_rid: start_row,
            count: row_count,
            rids: None,
        }
    }

    pub fn create_from_rids(rids: Vec<u32>) -> RidList {
        RidList {
            start_rid: 0,
            count: rids.len() as u32,
            rids: Some(rids),
        }
    }

    pub fn get(&self, index: u32) -> u32 {
        match self.rids {
            Some(ref rids) => {
                if index >= rids.len() as u32 {
                    0
                } else {
                    rids[index as usize]
                }
            },
            None => {
                if index >= self.count as u32 {
                    0
                } else {
                    self.start_rid + index
                }
            },
        }
    }
}

#[derive(Eq, PartialEq)]
enum MetadataType {
    Unknown,
    Compressed,
    ENC
}

pub struct Metadata {
    pub strings_stream: StringsStream,
    pub table_stream: TableStream,
}

impl Metadata {
    // 读取Assembly的元数据
    pub fn new(pe: &PE, reader: &mut ImageReader) -> io::Result<Metadata> {
        let dot_net_dir = pe.image_option_header.data_directories.get(14).unwrap();
        if dot_net_dir.virtual_address == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid .NET data directory RVA"));
        }
        reader.set_position(pe.rva_to_file_offset(dot_net_dir.virtual_address))?;
        let cor20_header = ImageCor20Header::new(reader)?;
        let md_rva = cor20_header.metadata.virtual_address;
        if md_rva == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid .NET metadata RVA"));
        }
        reader.set_position(pe.rva_to_file_offset(md_rva))?;
        let md_header = MetadataHeader::new(reader)?;

        let mut strings_stream = Default::default();
        let mut strings_stream_loaded = false;
        let mut table_stream = Default::default();
        let mut table_stream_loaded = false;

        match Metadata::get_metadata_type(&md_header.stream_headers) {
            Ok(MetadataType::Compressed) => {
                let metadata_base_offset = pe.rva_to_file_offset(cor20_header.metadata.virtual_address);
                for sh in md_header.stream_headers.iter().rev() {
                    match sh.name.as_str() {
                        "#Strings" => {
                            if !strings_stream_loaded {
                                reader.set_position(metadata_base_offset + sh.offset as usize)?;
                                strings_stream = StringsStream::new(reader, sh.size)?;
                                strings_stream_loaded = true;
                            }
                        },
                        "#US" => {

                        },
                        "#Blob" => {

                        },
                        "#GUID" => {

                        },
                        "#~" => {
                            if !table_stream_loaded {
                                reader.set_position(metadata_base_offset + sh.offset as usize)?;
                                table_stream = TableStream::new(reader)?;
                                table_stream_loaded = true;
                                continue;
                            }
                        },
                        "#Pdb" => {

                        },
                        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid .NET metadata stream name"))
                    }
                }
            },
            Ok(MetadataType::ENC) => {

            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "No #~ or #- stream found"))
        }

        if !table_stream_loaded {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "No #~ stream found"));
        }

        Ok(Metadata {
            strings_stream,
            table_stream,
        })
    }

    fn get_metadata_type(stream_headers: &Vec<StreamHeader>) -> io::Result<MetadataType> {
        let mut md_type = MetadataType::Unknown;
        for stream_header in stream_headers {
            if md_type == MetadataType::Unknown {
                if stream_header.name == "#~" {
                    md_type = MetadataType::Compressed;
                } else if stream_header.name == "#-" {
                    md_type = MetadataType::ENC;
                }
            }
            if stream_header.name == "#Schema" {
                md_type = MetadataType::ENC;
            }
        }
        Ok(md_type)
    }

    fn get_rid_list(src_table: &MDTable, src_rid: u32, col_index: i32, dst_table: &MDTable) -> RidList {
        assert!(src_table.row_count > 0);
        assert!(src_rid > 0);
        let col = &src_table.columns[col_index as usize];
        let mut start_rid = 0;
        if !col.try_read_rid(src_rid, &mut start_rid) {
            return Default::default()
        }
        let mut next_list_rid = 0;
        let has_next = col.try_read_rid(src_rid + 1, &mut next_list_rid);
        let last_rid = dst_table.row_count + 1;
        if start_rid >= last_rid {
            return Default::default()
        }
        let mut end_rid;
        if !has_next || (next_list_rid == 0 && src_rid + 1 == src_table.row_count && dst_table.row_count == 0xFFFF) {
            end_rid = last_rid;
        } else {
            end_rid = next_list_rid;
        }
        if end_rid < start_rid {
            end_rid = start_rid;
        }
        if end_rid > last_rid {
            end_rid = last_rid;
        }
        RidList::create(start_rid, end_rid - start_rid)
    }

    pub fn get_param_rid_list(&self, src_rid: u32) -> RidList {
        Metadata::get_rid_list(&self.table_stream.md_tables[6], src_rid, 5, &self.table_stream.md_tables[8])
    }

    // 获取参数的拥有者方法
    // pub fn get_param_owner(&self, row: u32) -> u32 {
    //     let index = row as usize - 1;
    //     if index >= self.param_row_to_owner_row.len() {
    //         0
    //     } else {
    //         self.param_row_to_owner_row[index]
    //     }
    // }
}