mod flag_info;
use flag_info::*;
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

use std::collections::HashMap;
use std::io;
use std::convert::TryInto;

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

#[derive(Debug)]
pub struct Method {
    pub token: u32,         // 和字典Key一致
    pub rva: u32,           // 函数入口在image中的偏移
    pub impl_flags: u16,    // 实现标志
    pub flags: u16,         // 函数标志，这和MDTable中的flag不同
    pub name: String,       // 函数名
    pub signature: u16,     // 签名
    pub param_list: u16,    // 参数列表，需要在StringsStream查找

    pub max_stack: u16,     // 最大堆栈大小
    pub header_size: u8,    // 函数头大小
    pub code_size: u32,     // 代码大小
    pub local_var_sig_token: u32,   // 局部变量Signature

    pub header_position: usize,     // MethodHeader在Image中的真实位置
    pub code_position: usize,       // IL指令在Image中的真实位置
}

impl Method {
    pub fn read_methods(pe: &PE, table_stream: &TableStream, strings_stream: &StringsStream, reader: &mut ImageReader) -> io::Result<HashMap<u32, Method>> {
        let mut methods = HashMap::new();
        let method_table = &table_stream.md_tables[6];
        for i in 0..method_table.row_count {
            let token = 0x06000001 + i as u32;
            let rva = method_table.columns[0].get_cell_u32(i);
            let header_position = pe.rva_to_file_offset(rva);
            reader.set_position(header_position)?;

            let mut flags: u16;
            let header_size: u8;
            let code_size: u32;
            let max_stack: u16;
            let local_var_sig_token: u32;
            let b = reader.read_u8()? & 7;
            match b {
                2 | 6 => {  // Tiny header. [7:2] = code size, max stack is 8, no locals or exception handlers
                    flags = 2;
                    max_stack = 8;
                    code_size = (b >> 2) as u32;
                    local_var_sig_token = 0;
                    header_size = 1;
                },
                3 => {  // Fat header. Can have locals and exception handlers
                    flags = (reader.read_u8()? as u16) << 8;
                    header_size = 4 * (flags >> 12) as u8;
                    max_stack = reader.read_u16()?;
                    code_size = reader.read_u32()?;
                    local_var_sig_token = reader.read_u32()?;
                    
                    // The CLR allows the code to start inside the method header. But if it does,
				    // the CLR doesn't read any exceptions.
                    reader.back(12)?;
                    reader.advance(header_size as usize)?;
                    if header_size < 12 {
                        flags &= 0xFFF7;
                    }
                },
                _ => panic!("Invalid method header")
            }

            // TODO: 读取locals

            methods.insert(token, Method {
                token,
                rva,
                impl_flags: method_table.columns[1].get_cell_u16(i),
                flags,
                name: strings_stream.get_string(method_table.columns[3].get_cell_u16(i) as u32).clone(),
                signature: method_table.columns[4].get_cell_u16(i),
                param_list: method_table.columns[5].get_cell_u16(i),

                max_stack,
                header_size,
                code_size,
                local_var_sig_token,

                header_position,
                code_position: header_position + header_size as usize,
            });
        }
        Ok(methods)
    }

    pub fn check_impl_flag_info(flag: u16, flag_info: ImplAttrFlagInfo) -> bool {
        match flag_info {
            ImplAttrFlagInfo::CodeType(t) => {
                flag & 0x0003 & (t as u16) != 0
            },
            ImplAttrFlagInfo::Managed(m) => {
                flag & 0x0004 & (m as u16) != 0
            },
            ImplAttrFlagInfo::CommonImplAttrFlagInfo(c) => {
                flag & (c as u16) != 0
            }
        }
    }
}

pub struct Param {
    pub token: u32,
    pub flags: u16,
    pub sequence: u16,
    pub name: u16,
}

impl Param {
    pub fn read_params(reader: &mut ImageReader, count: usize) -> io::Result<HashMap<u32, Param>> {
        let mut params = HashMap::new();
        reader.set_position(0x3EC)?;  // 定位到Params位置,目前这是通过16进制编辑器定位的
        for i in 0..count {
            let mut buf = [0u8; 6];
            reader.read_bytes(&mut buf)?;
            let token = 0x08000001 + i as u32;
            params.insert(token, Param {
                token,
                flags: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
                sequence: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
                name: u16::from_le_bytes(buf[4..6].try_into().unwrap()),
            });
        }
        Ok(params)
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
    pub methods: HashMap<u32, Method>,  // <token(0x06000001...), Method>
    pub params: HashMap<u32, Param>,    // <token(0x08000001...), Param>
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

        let methods = Method::read_methods(pe, &table_stream, &strings_stream, reader)?;
        println!("{:?}", methods.get(&0x06000001).unwrap());
        println!("{:?}", methods.get(&0x06000002).unwrap());
        println!("{:?}", methods.get(&0x06000003).unwrap());

        Ok(Metadata {
            strings_stream,
            table_stream,
            methods,
            params: Param::read_params(reader, 2)?,     // TODO: 目前是2个
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

    pub fn get_param_count(&self, method: &Method) -> u16 {
        match self.params.get(&(0x08000000 + method.param_list as u32)) {
            Some(param) => {
                let mut param_count: u16 = 1;
                let mut token = param.token + 1;
                loop {
                    match self.params.get(&token) {
                        Some(p) => {
                            if p.sequence < param.sequence {
                                return param_count;
                            }
                            param_count += 1;
                            token += 1;
                        },
                        None => return param_count
                    }
                }
            }
            None => 0
        }
    }
}