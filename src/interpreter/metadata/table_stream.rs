use num_derive::FromPrimitive;
use std::convert::TryInto;
use std::fmt;

#[repr(u8)]
#[derive(Copy, Clone, FromPrimitive)]
pub enum MDTableType {
	// Module table (00h)
	Module,
	// TypeRef table (01h)
	TypeRef,
	// TypeDef table (02h)
	TypeDef,
	// FieldPtr table (03h)
	FieldPtr,
	// Field table (04h)
	Field,
	// MethodPtr table (05h)
	MethodPtr,
	// Method table (06h)
	Method,
	// ParamPtr table (07h)
	ParamPtr,
	// Param table (08h)
	Param,
	// InterfaceImpl table (09h)
	InterfaceImpl,
	// MemberRef table (0Ah)
	MemberRef,
	// Constant table (0Bh)
	Constant,
	// CustomAttribute table (0Ch)
	CustomAttribute,
	// FieldMarshal table (0Dh)
	FieldMarshal,
	// DeclSecurity table (0Eh)
	DeclSecurity,
	// ClassLayout table (0Fh)
	ClassLayout,
	// FieldLayout table (10h)
	FieldLayout,
	// StandAloneSig table (11h)
	StandAloneSig,
	// EventMap table (12h)
	EventMap,
	// EventPtr table (13h)
	EventPtr,
	// Event table (14h)
	Event,
	// PropertyMap table (15h)
	PropertyMap,
	// PropertyPtr table (16h)
	PropertyPtr,
	// Property table (17h)
	Property,
	// MethodSemantics table (18h)
	MethodSemantics,
	// MethodImpl table (19h)
	MethodImpl,
	// ModuleRef table (1Ah)
	ModuleRef,
	// TypeSpec table (1Bh)
	TypeSpec,
	// ImplMap table (1Ch)
	ImplMap,
	// FieldRVA table (1Dh)
	FieldRVA,
	// ENCLog table (1Eh)
	ENCLog,
	// ENCMap table (1Fh)
	ENCMap,
	// Assembly table (20h)
	Assembly,
	// AssemblyProcessor table (21h)
	AssemblyProcessor,
	// AssemblyOS table (22h)
	AssemblyOS,
	// AssemblyRef table (23h)
	AssemblyRef,
	// AssemblyRefProcessor table (24h)
	AssemblyRefProcessor,
	// AssemblyRefOS table (25h)
	AssemblyRefOS,
	// File table (26h)
	File,
	// ExportedType table (27h)
	ExportedType,
	// ManifestResource table (28h)
	ManifestResource,
	// NestedClass table (29h)
	NestedClass,
	// GenericParam table (2Ah)
	GenericParam,
	// MethodSpec table (2Bh)
	MethodSpec,
	// GenericParamConstraint table (2Ch)
	GenericParamConstraint,
    // Reserved
    X2D,
    X2E,
    X2F,
	// (Portable PDB) Document table (30h)
	Document,
	// (Portable PDB) MethodDebugInformation table (31h)
	MethodDebugInformation,
	// (Portable PDB) LocalScope table (32h)
	LocalScope,
	// (Portable PDB) LocalVariable table (33h)
	LocalVariable,
	// (Portable PDB) LocalConstant table (34h)
	LocalConstant,
	// (Portable PDB) ImportScope table (35h)
	ImportScope,
	// (Portable PDB) StateMachineMethod table (36h)
	StateMachineMethod,
	// (Portable PDB) CustomDebugInformation table (37h)
	CustomDebugInformation,
}

impl MDTableType {
    pub fn is_sys_table(&self) -> bool {
        (*self as u8) < (MDTableType::Document as u8)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MDColumnType {
    	// RID into Module table
		Module,
		// RID into TypeRef table
		TypeRef,
		// RID into TypeDef table
		TypeDef,
		// RID into FieldPtr table
		FieldPtr,
		// RID into Field table
		Field,
		// RID into MethodPtr table
		MethodPtr,
		// RID into Method table
		Method,
		// RID into ParamPtr table
		ParamPtr,
		// RID into Param table
		Param,
		// RID into InterfaceImpl table
		InterfaceImpl,
		// RID into MemberRef table
		MemberRef,
		// RID into Constant table
		Constant,
		// RID into CustomAttribute table
		CustomAttribute,
		// RID into FieldMarshal table
		FieldMarshal,
		// RID into DeclSecurity table
		DeclSecurity,
		// RID into ClassLayout table
		ClassLayout,
		// RID into FieldLayout table
		FieldLayout,
		// RID into StandAloneSig table
		StandAloneSig,
		// RID into EventMap table
		EventMap,
		// RID into EventPtr table
		EventPtr,
		// RID into Event table
		Event,
		// RID into PropertyMap table
		PropertyMap,
		// RID into PropertyPtr table
		PropertyPtr,
		// RID into Property table
		Property,
		// RID into MethodSemantics table
		MethodSemantics,
		// RID into MethodImpl table
		MethodImpl,
		// RID into ModuleRef table
		ModuleRef,
		// RID into TypeSpec table
		TypeSpec,
		// RID into ImplMap table
		ImplMap,
		// RID into FieldRVA table
		FieldRVA,
		// RID into ENCLog table
		ENCLog,
		// RID into ENCMap table
		ENCMap,
		// RID into Assembly table
		Assembly,
		// RID into AssemblyProcessor table
		AssemblyProcessor,
		// RID into AssemblyOS table
		AssemblyOS,
		// RID into AssemblyRef table
		AssemblyRef,
		// RID into AssemblyRefProcessor table
		AssemblyRefProcessor,
		// RID into AssemblyRefOS table
		AssemblyRefOS,
		// RID into File table
		File,
		// RID into ExportedType table
		ExportedType,
		// RID into ManifestResource table
		ManifestResource,
		// RID into NestedClass table
		NestedClass,
		// RID into GenericParam table
		GenericParam,
		// RID into MethodSpec table
		MethodSpec,
		// RID into GenericParamConstraint table
		GenericParamConstraint,
		// RID into Document table
		Document = 0x30,
		// RID into MethodDebugInformation table
		MethodDebugInformation,
		// RID into LocalScope table
		LocalScope,
		// RID into LocalVariable table
		LocalVariable,
		// RID into LocalConstant table
		LocalConstant,
		// RID into ImportScope table
		ImportScope,
		// RID into StateMachineMethod table
		StateMachineMethod,
		// RID into CustomDebugInformation table
		CustomDebugInformation,
		// 8-bit byte
		Byte = 0x40,
		// 16-bit signed int
		Int16,
		// 16-bit unsigned int
		UInt16,
		// 32-bit signed int
		Int32,
		// 32-bit unsigned int
		UInt32,
		// Index into #Strings stream
		Strings,
		// Index into #GUID stream
		GUID,
		// Index into #Blob stream
		Blob,
		// TypeDefOrRef encoded token
		TypeDefOrRef,
		// HasConstant encoded token
		HasConstant,
		// HasCustomAttribute encoded token
		HasCustomAttribute,
		// HasFieldMarshal encoded token
		HasFieldMarshal,
		// HasDeclSecurity encoded token
		HasDeclSecurity,
		// MemberRefParent encoded token
		MemberRefParent,
		// HasSemantic encoded token
		HasSemantic,
		// MethodDefOrRef encoded token
		MethodDefOrRef,
		// MemberForwarded encoded token
		MemberForwarded,
		// Implementation encoded token
		Implementation,
		// CustomAttributeType encoded token
		CustomAttributeType,
		// ResolutionScope encoded token
		ResolutionScope,
		// TypeOrMethodDef encoded token
		TypeOrMethodDef,
		// HasCustomDebugInformation encoded token
		HasCustomDebugInformation,
}

pub struct MDColumn {
    pub name: &'static str,
    pub column_type: MDColumnType,  // 列的类型
    pub size: u8,                   // 列数据类型的字节大小
    pub offset: u32,                // 在该行中的偏移量
    pub index: u8,                  // 索引
    pub data: Vec<u8>,              // 列数据，用u8存储，读取时根据size来转换
}

impl MDColumn {
    pub fn new(index: u8, name: &'static str, column_size: MDColumnType) -> MDColumn {
        MDColumn {
            index,
            name,
            column_type: column_size,
            size: 0,
            offset: 0,
            data: Vec::new(),
        }
    }

    fn check_rid(&self, rid: u32) -> bool {
        ((rid * self.size as u32) as usize) <= self.data.len() && rid > 0
    }

    pub fn try_read_rid(&self, rid: u32, result: &mut u32) -> bool {
        if !self.check_rid(rid) {
            return false;
        }
        match self.size {
            2 => {
                *result = self.get_cell_u16(rid - 1) as u32;
                true
            },
            4 => {
                *result = self.get_cell_u32(rid - 1);
                true
            },
            _ => false,
        }
    }

    pub fn get_cell_u8(&self, row: u32) -> u8 {
        assert_eq!(self.size, 1);
        self.data[row as usize]
    }

    pub fn get_cell_u16(&self, row: u32) -> u16 {
        assert_eq!(self.size, 2);
        u16::from_le_bytes(self.data[row as usize * 2..(row + 1) as usize * 2].try_into().unwrap())
    }

    pub fn get_cell_u32(&self, row: u32) -> u32 {
        assert_eq!(self.size, 4);
        u32::from_le_bytes(self.data[row as usize * 4..(row + 1) as usize * 4].try_into().unwrap())
    }

    pub fn get_cell_u16_or_u32(&self, row: u32) -> u32 {
        if self.size == 2 {
            self.get_cell_u16(row) as u32
        } else if self.size == 4 {
            self.get_cell_u32(row)
        } else {
            panic!("unsupported size");
        }
    }

    pub fn get_cell_u64(&self, row: u32) -> u64 {
        assert_eq!(self.size, 8);
        u64::from_le_bytes(self.data[row as usize * 8..(row + 1) as usize * 8].try_into().unwrap())
    }
}

pub struct MDTable {
    pub table_type: MDTableType,
    pub name: &'static str,
    pub row_count: u32,             // 表格一共有几行
    pub row_size: u32,              // 表格一行的字节大小
    pub columns: Vec<MDColumn>,
    pub position: usize,            // 在文件中的真实位置
}

impl MDTable {
    pub fn new(table_type: MDTableType, name: &'static str, columns: Vec<MDColumn>) -> MDTable {
        MDTable {
            table_type,
            name,
            columns,
            row_count: 0,
            row_size: 0,
            position: 0,
        }
    }

    pub fn read_all(&mut self, reader: &mut ImageReader) -> io::Result<()> {
        if self.position == 0 {
            return Ok(())
        }
        for col in self.columns.iter_mut() {
            if col.data.len() != 0 {
                col.data.clear();
            }
        }
        reader.set_position(self.position)?;
        let mut buf = [0u8; 16];  // 确保buf容纳的下最大的数据类型
        for _ in 0..self.row_count {
            for col in self.columns.iter_mut() {
                reader.read_bytes_exact(&mut buf, col.size as usize)?;
                col.data.extend(buf[..col.size as usize].iter());
            }
        }
        Ok(())
    }
}

impl fmt::Debug for MDTable {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.name)?;
        formatter.write_str("\n----------------------------------\n")?;
        for col in self.columns.iter() {
            if col.name.len() > 6 {
                formatter.write_str(&col.name[0..6])?;
            } else {
                formatter.write_str(col.name)?;
            }
            formatter.write_str("\t")?;
        }
        formatter.write_str("\n")?;
        
        for row in 0..self.row_count {
            for col in self.columns.iter() {
                match col.size {
                    1 => formatter.write_str(&format!("0x{:01X}", col.get_cell_u8(row)))?,
                    2 => formatter.write_str(&format!("0x{:02X}", col.get_cell_u16(row)))?,
                    4 => formatter.write_str(&format!("0x{:03X}", col.get_cell_u32(row)))?,
                    8 => formatter.write_str(&format!("0x{:04X}", col.get_cell_u64(row)))?,
                    _ => panic!("Invalid column size"),
                }
                formatter.write_str("\t")?;
            }
            formatter.write_str("\n")?;
        }
        formatter.write_str("\n")?;
        Ok(())
    }
}

use std::io;
use bitflags::bitflags;
use crate::interpreter::image_reader::ImageReader;
use crate::interpreter::metadata::md_token::CodedToken;

bitflags! {
    pub struct MDStreamFlags: u8 {
        const DEFAULT = 0x00;
        const BIG_STRINGS = 0x01;
        const BIS_GUID = 0x02;
        const BIG_BLOB = 0x04;
        const PADDING = 0x08;
        const DELTA_ONLY = 0x20;
        const EXTRA_DATA = 0x40;
        const HAS_DELETE = 0x80;
    }
}

impl Default for MDStreamFlags {
    fn default() -> Self {
        MDStreamFlags::DEFAULT
    }
}

#[derive(Default)]
pub struct TableStream {
    pub reserved1: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub flags: MDStreamFlags,
    pub log2_rid: u8,
    pub valid_mask: u64,
    pub sorted_mask: u64,
    pub md_tables: Vec<MDTable>,
    pub extra_data: u32,
    pub md_tables_pos: usize,
}

impl TableStream {
    pub fn new(reader: &mut ImageReader) -> io::Result<TableStream> {
        let reserved1 = reader.read_u32()?;
        let major_version = reader.read_u8()?;
        let minor_version = reader.read_u8()?;
        let flags = MDStreamFlags::from_bits_truncate(reader.read_u8()?);
        let log2_rid = reader.read_u8()?;
        let valid_mask = reader.read_u64()?;
        let sorted_mask = reader.read_u64()?;
        let mut md_tables = Vec::new();
        let max_present_tables = TableStream::create_tables(&mut md_tables, major_version, minor_version);

        let mut valid = valid_mask;
        let mut sizes = [0u32; 64];
        for i in 0..64usize {
            let mut rows;
            if valid & 1 == 0 {
                rows = 0;
            } else {
                rows = reader.read_u32()?;
            }
            rows &= 0x00FFFFFF;
            if i >= max_present_tables {
                rows = 0;
            }
            sizes[i] = rows;
            if i < md_tables.len() {
                md_tables[i].row_count = rows;
            }
            valid >>= 1;
        }

        let mut extra_data = 0;
        if flags & MDStreamFlags::EXTRA_DATA != MDStreamFlags::DEFAULT {
            extra_data = reader.read_u32()?;
        }

        // 计算Table大小
        for table in md_tables.iter_mut() {
            let mut col_offset = 0;
            for col in table.columns.iter_mut() {
                col.offset = col_offset;
                let big_strings = (flags & MDStreamFlags::BIG_STRINGS) != MDStreamFlags::DEFAULT;
                let big_guid = (flags & MDStreamFlags::BIS_GUID) != MDStreamFlags::DEFAULT;
                let big_blob = (flags & MDStreamFlags::BIG_BLOB) != MDStreamFlags::DEFAULT;
                let col_size = TableStream::get_col_size(big_strings, big_guid, big_blob, col.column_type, &sizes);
                col.size = col_size;
                col_offset += col_size as u32;
            }
            table.row_size = col_offset;
        }

        // 计算Table数据在文件中的地址
        let mut current_pos = reader.get_position();
        for table in md_tables.iter_mut() {
            table.position = current_pos;
            current_pos += (table.row_size * table.row_count) as usize;  // 这个table的真实大小
        }

        for md_table in md_tables.iter_mut() {
            md_table.read_all(reader)?;
        }

        // println!("{:?}", md_tables[6]);  // 输出Method和Param
        // println!("{:?}", md_tables[8]);

        Ok(TableStream {
            reserved1,
            major_version,
            minor_version,
            flags,
            log2_rid,
            valid_mask,
            sorted_mask,
            md_tables,
            extra_data,
            md_tables_pos: reader.get_position()
        })
    }

    fn get_col_size(big_strings: bool, big_guid: bool, big_blob: bool, column_size: MDColumnType, row_counts: &[u32]) -> u8 {
        if column_size >= MDColumnType::Module && column_size <= MDColumnType::CustomDebugInformation {
            let table_index = column_size as usize - MDColumnType::Module as usize;
            if table_index >= row_counts.len() || row_counts[table_index] <= 0xFFFF {
                return 2;
            } else {
                return 4;
            }
        }
        if column_size >= MDColumnType::TypeDefOrRef && column_size <= MDColumnType::HasCustomDebugInformation {
            let info = CodedToken::from_column_size(column_size);
            let mut max_rows: u32 = 0;
            for table_type in info.table_types.iter() {
                let table_index = *table_type as usize;
                let table_rows;
                if table_index >= row_counts.len() {
                    table_rows = 0;
                } else {
                    table_rows = row_counts[table_index];
                }
                if table_rows > max_rows {
                    max_rows = table_rows;
                }
            }
            // Can't overflow since maxRows <= 0x00FFFFFF and info.Bits < 8
            let final_rows = max_rows << info.bits;
            if final_rows <= 0xFFFF {
                return 2;
            } else {
                return 4;
            }
        }
        match column_size {
            MDColumnType::Byte => 1,
            MDColumnType::Int16 => 2,
            MDColumnType::UInt16 => 2,
            MDColumnType::Int32 => 4,
            MDColumnType::UInt32 => 4,
            MDColumnType::Strings => {
                if big_strings {
                    4
                } else {
                    2
                }
            },
            MDColumnType::GUID => {
                if big_guid {
                    4
                } else {
                    2
                }
            },
            MDColumnType::Blob => {
                if big_blob {
                    4
                } else {
                    2
                }
            },
            _ => panic!("Invalid column size")
        }
    }

    fn create_tables(tables: &mut Vec<MDTable>, major_version: u8, minor_version: u8) -> usize {
        let normal_max_tables = MDTableType::CustomDebugInformation as u32 + 1;
        let max_present_tables = match (major_version, minor_version) {
            (1, 0) => MDTableType::NestedClass as usize + 1,
            _ => normal_max_tables as usize
        };
        tables.push(MDTable::new(
            MDTableType::Module,
            "Module",
            vec![
                MDColumn::new(0, "Generation", MDColumnType::UInt16),
                MDColumn::new(1, "Name", MDColumnType::Strings),
                MDColumn::new(2, "Mvid", MDColumnType::GUID),
                MDColumn::new(3, "EncId", MDColumnType::GUID),
                MDColumn::new(4, "EncBaseId", MDColumnType::GUID),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::TypeRef,
            "TypeRef",
            vec![
                MDColumn::new(0, "ResolutionScope", MDColumnType::ResolutionScope),
                MDColumn::new(1, "Name", MDColumnType::Strings),
                MDColumn::new(2, "Namespace", MDColumnType::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::TypeDef,
            "TypeDef",
            vec![
                MDColumn::new(0, "Flags", MDColumnType::UInt32),
                MDColumn::new(1, "Name", MDColumnType::Strings),
                MDColumn::new(2, "Namespace", MDColumnType::Strings),
                MDColumn::new(3, "Extends", MDColumnType::TypeDefOrRef),
                MDColumn::new(4, "FieldList", MDColumnType::Field),
                MDColumn::new(5, "MethodList", MDColumnType::Method),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::FieldPtr,
            "FieldPtr",
            vec![
                MDColumn::new(0, "Field", MDColumnType::Field),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Field,
            "Field",
            vec![
                MDColumn::new(0, "Flags", MDColumnType::UInt16),
                MDColumn::new(1, "Name", MDColumnType::Strings),
                MDColumn::new(2, "Signature", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodPtr,
            "MethodPtr",
            vec![
                MDColumn::new(0, "Method", MDColumnType::Method),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Method,
            "Method",
            vec![
                MDColumn::new(0, "RVA", MDColumnType::UInt32),
                MDColumn::new(1, "ImplFlags", MDColumnType::UInt16),
                MDColumn::new(2, "Flags", MDColumnType::UInt16),
                MDColumn::new(3, "Name", MDColumnType::Strings),
                MDColumn::new(4, "Signature", MDColumnType::Blob),
                MDColumn::new(5, "ParamList", MDColumnType::Param),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ParamPtr,
            "ParamPtr",
            vec![
                MDColumn::new(0, "Param", MDColumnType::Param),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Param,
            "Param",
            vec![
                MDColumn::new(0, "Flags", MDColumnType::UInt16),
                MDColumn::new(1, "Sequence", MDColumnType::UInt16),
                MDColumn::new(2, "Name", MDColumnType::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::InterfaceImpl,
            "InterfaceImpl",
            vec![
                MDColumn::new(0, "Class", MDColumnType::TypeDef),
                MDColumn::new(1, "Interface", MDColumnType::TypeDefOrRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MemberRef,
            "MemberRef",
            vec![
                MDColumn::new(0, "Class", MDColumnType::MemberRefParent),
                MDColumn::new(1, "Name", MDColumnType::Strings),
                MDColumn::new(2, "Signature", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Constant,
            "Constant",
            vec![
                MDColumn::new(0, "Type", MDColumnType::Byte),
                MDColumn::new(1, "Padding", MDColumnType::Byte),
                MDColumn::new(2, "Parent", MDColumnType::HasConstant),
                MDColumn::new(3, "Value", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::CustomAttribute,
            "CustomAttribute",
            vec![
                MDColumn::new(0, "Parent", MDColumnType::HasCustomAttribute),
                MDColumn::new(1, "Type", MDColumnType::CustomAttributeType),
                MDColumn::new(2, "Value", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::FieldMarshal,
            "FieldMarshal",
            vec![
                MDColumn::new(0, "Parent", MDColumnType::HasFieldMarshal),
                MDColumn::new(1, "NativeType", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::DeclSecurity,
            "DeclSecurity",
            vec![
                MDColumn::new(0, "Action", MDColumnType::UInt16),
                MDColumn::new(1, "Parent", MDColumnType::HasDeclSecurity),
                MDColumn::new(2, "PermissionSet", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ClassLayout,
            "ClassLayout",
            vec![
                MDColumn::new(0, "PackingSize", MDColumnType::UInt16),
                MDColumn::new(1, "ClassSize", MDColumnType::UInt32),
                MDColumn::new(2, "Parent", MDColumnType::TypeDef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::FieldLayout,
            "FieldLayout",
            vec![
                MDColumn::new(0, "OffSet", MDColumnType::UInt32),
                MDColumn::new(1, "Field", MDColumnType::Field),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::StandAloneSig,
            "StandAloneSig",
            vec![
                MDColumn::new(0, "Signature", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::EventMap,
            "EventMap",
            vec![
                MDColumn::new(0, "Parent", MDColumnType::TypeDef),
                MDColumn::new(1, "EventList", MDColumnType::Event),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::EventPtr,
            "EventPtr",
            vec![
                MDColumn::new(0, "Event", MDColumnType::Event),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Event,
            "Event",
            vec![
                MDColumn::new(0, "EventFlags", MDColumnType::UInt16),
                MDColumn::new(1, "Name", MDColumnType::Strings),
                MDColumn::new(2, "EventType", MDColumnType::TypeDefOrRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::PropertyMap,
            "PropertyMap",
            vec![
                MDColumn::new(0, "Parent", MDColumnType::TypeDef),
                MDColumn::new(1, "PropertyList", MDColumnType::Property),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::PropertyPtr,
            "PropertyPtr",
            vec![
                MDColumn::new(0, "Property", MDColumnType::Property),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Property,
            "Property",
            vec![
                MDColumn::new(0, "PropFlags", MDColumnType::UInt16),
                MDColumn::new(1, "Name", MDColumnType::Strings),
                MDColumn::new(2, "Type", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodSemantics,
            "MethodSemantics",
            vec![
                MDColumn::new(0, "Semantics", MDColumnType::UInt16),
                MDColumn::new(1, "Method", MDColumnType::Method),
                MDColumn::new(2, "Association", MDColumnType::HasSemantic),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodImpl,
            "MethodImpl",
            vec![
                MDColumn::new(0, "Class", MDColumnType::TypeDef),
                MDColumn::new(1, "MethodBody", MDColumnType::MethodDefOrRef),
                MDColumn::new(2, "MethodDecl", MDColumnType::MethodDefOrRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ModuleRef,
            "ModuleRef",
            vec![
                MDColumn::new(0, "Name", MDColumnType::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::TypeSpec,
            "TypeSpec",
            vec![
                MDColumn::new(0, "Signature", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ImplMap,
            "ImplMap",
            vec![
                MDColumn::new(0, "MappingFlags", MDColumnType::UInt16),
                MDColumn::new(1, "MemberForwarded", MDColumnType::MemberForwarded),
                MDColumn::new(2, "ImportName", MDColumnType::Strings),
                MDColumn::new(3, "ImportScope", MDColumnType::ModuleRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::FieldRVA,
            "FieldRVA",
            vec![
                MDColumn::new(0, "RVA", MDColumnType::UInt32),
                MDColumn::new(1, "Field", MDColumnType::Field),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ENCLog,
            "ENCLog",
            vec![
                MDColumn::new(0, "Token", MDColumnType::UInt32),
                MDColumn::new(1, "FuncCode", MDColumnType::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ENCMap,
            "ENCMap",
            vec![
                MDColumn::new(0, "Token", MDColumnType::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Assembly,
            "Assembly",
            vec![
                MDColumn::new(0, "HashAlgId", MDColumnType::UInt32),
                MDColumn::new(1, "MajorVersion", MDColumnType::UInt16),
                MDColumn::new(2, "MinorVersion", MDColumnType::UInt16),
                MDColumn::new(3, "BuildNumber", MDColumnType::UInt16),
                MDColumn::new(4, "RevisionNumber", MDColumnType::UInt16),
                MDColumn::new(5, "Flags", MDColumnType::UInt32),
                MDColumn::new(6, "PublicKey", MDColumnType::Blob),
                MDColumn::new(7, "Name", MDColumnType::Strings),
                MDColumn::new(8, "Locale", MDColumnType::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyProcessor,
            "AssemblyProcessor",
            vec![
                MDColumn::new(0, "Processor", MDColumnType::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyOS,
            "AssemblyOS",
            vec![
                MDColumn::new(0, "OSPlatformId", MDColumnType::UInt32),
                MDColumn::new(1, "OSMajorVersion", MDColumnType::UInt32),
                MDColumn::new(2, "OSMinorVersion", MDColumnType::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyRef,
            "AssemblyRef",
            vec![
                MDColumn::new(0, "MajorVersion", MDColumnType::UInt16),
                MDColumn::new(1, "MinorVersion", MDColumnType::UInt16),
                MDColumn::new(2, "BuildNumber", MDColumnType::UInt16),
                MDColumn::new(3, "RevisionNumber", MDColumnType::UInt16),
                MDColumn::new(4, "Flags", MDColumnType::UInt32),
                MDColumn::new(5, "PublicKeyOrToken", MDColumnType::Blob),
                MDColumn::new(6, "Name", MDColumnType::Strings),
                MDColumn::new(7, "Locale", MDColumnType::Strings),
                MDColumn::new(8, "HashValue", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyRefProcessor,
            "AssemblyRefProcessor",
            vec![
                MDColumn::new(0, "Processor", MDColumnType::UInt32),
                MDColumn::new(1, "AssemblyRef", MDColumnType::AssemblyRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyRefOS,
            "AssemblyRefOS",
            vec![
                MDColumn::new(0, "OSPlatformId", MDColumnType::UInt32),
                MDColumn::new(1, "OSMajorVersion", MDColumnType::UInt32),
                MDColumn::new(2, "OSMinorVersion", MDColumnType::UInt32),
                MDColumn::new(3, "AssemblyRef", MDColumnType::AssemblyRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::File,
            "File",
            vec![
                MDColumn::new(0, "Flags", MDColumnType::UInt32),
                MDColumn::new(1, "Name", MDColumnType::Strings),
                MDColumn::new(2, "HashValue", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ExportedType,
            "ExportedType",
            vec![
                MDColumn::new(0, "Flags", MDColumnType::UInt32),
                MDColumn::new(1, "TypeDefId", MDColumnType::UInt32),
                MDColumn::new(2, "TypeName", MDColumnType::Strings),
                MDColumn::new(3, "TypeNamespace", MDColumnType::Strings),
                MDColumn::new(4, "Implementation", MDColumnType::Implementation),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ManifestResource,
            "ManifestResource",
            vec![
                MDColumn::new(0, "Offset", MDColumnType::UInt32),
                MDColumn::new(1, "Flags", MDColumnType::UInt32),
                MDColumn::new(2, "Name", MDColumnType::Strings),
                MDColumn::new(3, "Implementation", MDColumnType::Implementation),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::NestedClass,
            "NestedClass",
            vec![
                MDColumn::new(0, "NestedClass", MDColumnType::TypeDef),
                MDColumn::new(1, "EnclosingClass", MDColumnType::TypeDef),
            ]
        ));
        if major_version == 1 && minor_version == 1 {
            tables.push(MDTable::new(
                MDTableType::GenericParam,
                "GenericParam",
                vec![
                    MDColumn::new(0, "Number", MDColumnType::UInt16),
                    MDColumn::new(1, "Flags", MDColumnType::UInt16),
                    MDColumn::new(2, "Owner", MDColumnType::TypeOrMethodDef),
                    MDColumn::new(3, "Name", MDColumnType::Strings),
                    MDColumn::new(4, "Kind", MDColumnType::TypeDefOrRef),
                ]
            ));
        } else {
            tables.push(MDTable::new(
                MDTableType::GenericParam,
                "GenericParam",
                vec![
                    MDColumn::new(0, "Number", MDColumnType::UInt16),
                    MDColumn::new(1, "Flags", MDColumnType::UInt16),
                    MDColumn::new(2, "Owner", MDColumnType::TypeOrMethodDef),
                    MDColumn::new(3, "Name", MDColumnType::Strings),
                ]
            ));
        }
        tables.push(MDTable::new(
            MDTableType::MethodSpec,
            "MethodSpec",
            vec![
                MDColumn::new(0, "Method", MDColumnType::MethodDefOrRef),
                MDColumn::new(1, "Instantiation", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::GenericParamConstraint,
            "GenericParamConstraint",
            vec![
                MDColumn::new(0, "Owner", MDColumnType::GenericParam),
                MDColumn::new(1, "Constraint", MDColumnType::TypeDefOrRef),
            ]
        ));
        tables.push(MDTable::new(MDTableType::X2D, "", vec![]));
        tables.push(MDTable::new(MDTableType::X2E, "", vec![]));
        tables.push(MDTable::new(MDTableType::X2F, "", vec![]));
        tables.push(MDTable::new(
            MDTableType::Document,
            "Document",
            vec![
                MDColumn::new(0, "Name", MDColumnType::Blob),
                MDColumn::new(1, "HashAlgorithm", MDColumnType::GUID),
                MDColumn::new(2, "Hash", MDColumnType::Blob),
                MDColumn::new(3, "Language", MDColumnType::GUID),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodDebugInformation,
            "MethodDebugInformation",
            vec![
                MDColumn::new(0, "Document", MDColumnType::Document),
                MDColumn::new(1, "SequencePoints", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::LocalScope,
            "LocalScope",
            vec![
                MDColumn::new(0, "Method", MDColumnType::Method),
                MDColumn::new(1, "ImportScope", MDColumnType::ImportScope),
                MDColumn::new(2, "VariableList", MDColumnType::LocalVariable),
                MDColumn::new(3, "ConstantList", MDColumnType::LocalConstant),
                MDColumn::new(4, "StartOffset", MDColumnType::UInt32),
                MDColumn::new(5, "Length", MDColumnType::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::LocalVariable,
            "LocalVariable",
            vec![
                MDColumn::new(0, "Attributes", MDColumnType::UInt16),
                MDColumn::new(1, "Index", MDColumnType::UInt16),
                MDColumn::new(2, "Name", MDColumnType::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::LocalConstant,
            "LocalConstant",
            vec![
                MDColumn::new(0, "Name", MDColumnType::Strings),
                MDColumn::new(1, "Signature", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ImportScope,
            "ImportScope",
            vec![
                MDColumn::new(0, "Parent", MDColumnType::ImportScope),
                MDColumn::new(1, "Imports", MDColumnType::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::StateMachineMethod,
            "StateMachineMethod",
            vec![
                MDColumn::new(0, "MoveNextMethod", MDColumnType::Method),
                MDColumn::new(1, "KickoffMethod", MDColumnType::Method),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::CustomDebugInformation,
            "CustomDebugInformation",
            vec![
                MDColumn::new(0, "Parent", MDColumnType::HasCustomDebugInformation),
                MDColumn::new(1, "Kind", MDColumnType::GUID),
                MDColumn::new(2, "Value", MDColumnType::Blob),
            ]
        ));

        max_present_tables
    }
}