#[repr(u8)]
#[derive(Copy, Clone)]
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
pub enum MDColumnSize {
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

pub struct CodedToken {
    pub table_types: Vec<MDTableType>,
    pub bits: i32,
    pub mask: i32,
}

impl CodedToken {
    pub fn new(bits: i32, table_types: Vec<MDTableType>) -> CodedToken {
        CodedToken {
            table_types: table_types,
            bits: bits,
            mask: (1 << bits) - 1,
        }
    }

    pub fn from_column_size(column_size: MDColumnSize) -> CodedToken {
        match column_size {
            MDColumnSize::TypeDefOrRef => CodedToken::new(2, vec![MDTableType::TypeDef, MDTableType::TypeRef, MDTableType::TypeSpec]),
            MDColumnSize::HasConstant => CodedToken::new(2, vec![MDTableType::Field, MDTableType::Param, MDTableType::Property]),
            MDColumnSize::HasCustomAttribute => CodedToken::new(5, vec![MDTableType::Method, MDTableType::Field, MDTableType::TypeRef, MDTableType::TypeDef, MDTableType::Param, MDTableType::InterfaceImpl, MDTableType::MemberRef, MDTableType::Module, MDTableType::DeclSecurity, MDTableType::Property, MDTableType::Event, MDTableType::StandAloneSig, MDTableType::ModuleRef, MDTableType::TypeSpec, MDTableType::Assembly, MDTableType::AssemblyRef, MDTableType::File, MDTableType::ExportedType, MDTableType::ManifestResource, MDTableType::GenericParam, MDTableType::GenericParamConstraint, MDTableType::MethodSpec, MDTableType::Module, MDTableType::Module]),
            MDColumnSize::HasFieldMarshal => CodedToken::new(1, vec![MDTableType::Field, MDTableType::Param]),
            MDColumnSize::HasDeclSecurity => CodedToken::new(2, vec![MDTableType::TypeDef, MDTableType::Method, MDTableType::Assembly]),
            MDColumnSize::MemberRefParent => CodedToken::new(3, vec![MDTableType::TypeDef, MDTableType::TypeRef, MDTableType::ModuleRef, MDTableType::Method, MDTableType::TypeSpec]),
            MDColumnSize::HasSemantic => CodedToken::new(1, vec![MDTableType::Event, MDTableType::Property]),
            MDColumnSize::MethodDefOrRef => CodedToken::new(1, vec![MDTableType::Method, MDTableType::MemberRef]),
            MDColumnSize::MemberForwarded => CodedToken::new(1, vec![MDTableType::Field, MDTableType::Method]),
            MDColumnSize::Implementation => CodedToken::new(2, vec![MDTableType::File, MDTableType::AssemblyRef, MDTableType::ExportedType]),
            MDColumnSize::CustomAttributeType => CodedToken::new(3, vec![MDTableType::Module, MDTableType::Module, MDTableType::Method, MDTableType::MemberRef]),
            MDColumnSize::ResolutionScope => CodedToken::new(2, vec![MDTableType::Module, MDTableType::ModuleRef, MDTableType::AssemblyRef, MDTableType::TypeRef]),
            MDColumnSize::TypeOrMethodDef => CodedToken::new(1, vec![MDTableType::TypeDef, MDTableType::Method]),
            MDColumnSize::HasCustomDebugInformation => CodedToken::new(5, vec![MDTableType::Method, MDTableType::Field, MDTableType::TypeRef, MDTableType::TypeDef, MDTableType::Param, MDTableType::InterfaceImpl, MDTableType::MemberRef, MDTableType::Module, MDTableType::DeclSecurity, MDTableType::Property, MDTableType::Event, MDTableType::StandAloneSig, MDTableType::ModuleRef, MDTableType::TypeSpec, MDTableType::Assembly, MDTableType::AssemblyRef, MDTableType::File, MDTableType::ExportedType, MDTableType::ManifestResource, MDTableType::GenericParam, MDTableType::GenericParamConstraint, MDTableType::MethodSpec, MDTableType::Document, MDTableType::LocalScope, MDTableType::LocalVariable, MDTableType::LocalConstant, MDTableType::ImportScope]),
            _ => panic!("Invalid column size"),
        }
    }
}

pub struct MDColumn {
    pub name: &'static str,
    pub size: u32,
    pub offset: u32,
    pub index: u8,
    pub column_size: MDColumnSize,
}

impl MDColumn {
    pub fn new(index: u8, name: &'static str, column_size: MDColumnSize) -> MDColumn {
        MDColumn {
            index,
            name,
            column_size,
            size: 0,
            offset: 0,
        }
    }
}

pub struct MDTable {
    pub table_type: MDTableType,
    pub name: &'static str,
    pub row_count: u32,
    pub row_size: u32,
    pub columns: Vec<MDColumn>,
    pub position: usize,  // 在文件中的真实位置
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
}

use std::io;
use bitflags::bitflags;
use crate::interpreter::image_reader::ImageReader;

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
                let col_size = TableStream::get_col_size(big_strings, big_guid, big_blob, col.column_size, &sizes);
                col.size = col_size;
                col_offset += col_size;
            }
            table.row_size = col_offset;
        }

        // 计算Table数据偏移
        let mut current_pos = reader.get_position();
        for table in md_tables.iter_mut() {
            table.position = current_pos;
            current_pos += (table.row_size * table.row_count) as usize;  // 这个table的真实大小
        }

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

    fn get_col_size(big_strings: bool, big_guid: bool, big_blob: bool, column_size: MDColumnSize, row_counts: &[u32]) -> u32 {
        if column_size >= MDColumnSize::Module && column_size <= MDColumnSize::CustomDebugInformation {
            let table_index = column_size as usize - MDColumnSize::Module as usize;
            if table_index >= row_counts.len() || row_counts[table_index] <= 0xFFFF {
                return 2;
            } else {
                return 4;
            }
        }
        if column_size >= MDColumnSize::TypeDefOrRef && column_size <= MDColumnSize::HasCustomDebugInformation {
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
            MDColumnSize::Byte => 1,
            MDColumnSize::Int16 => 2,
            MDColumnSize::UInt16 => 2,
            MDColumnSize::Int32 => 4,
            MDColumnSize::UInt32 => 4,
            MDColumnSize::Strings => {
                if big_strings {
                    4
                } else {
                    2
                }
            },
            MDColumnSize::GUID => {
                if big_guid {
                    4
                } else {
                    2
                }
            },
            MDColumnSize::Blob => {
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
                MDColumn::new(0, "Generation", MDColumnSize::UInt16),
                MDColumn::new(1, "Name", MDColumnSize::Strings),
                MDColumn::new(2, "Mvid", MDColumnSize::GUID),
                MDColumn::new(3, "EncId", MDColumnSize::GUID),
                MDColumn::new(4, "EncBaseId", MDColumnSize::GUID),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::TypeRef,
            "TypeRef",
            vec![
                MDColumn::new(0, "ResolutionScope", MDColumnSize::ResolutionScope),
                MDColumn::new(1, "Name", MDColumnSize::Strings),
                MDColumn::new(2, "Namespace", MDColumnSize::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::TypeDef,
            "TypeDef",
            vec![
                MDColumn::new(0, "Flags", MDColumnSize::UInt32),
                MDColumn::new(1, "Name", MDColumnSize::Strings),
                MDColumn::new(2, "Namespace", MDColumnSize::Strings),
                MDColumn::new(3, "Extends", MDColumnSize::TypeDefOrRef),
                MDColumn::new(4, "FieldList", MDColumnSize::Field),
                MDColumn::new(5, "MethodList", MDColumnSize::Method),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::FieldPtr,
            "FieldPtr",
            vec![
                MDColumn::new(0, "Field", MDColumnSize::Field),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Field,
            "Field",
            vec![
                MDColumn::new(0, "Flags", MDColumnSize::UInt16),
                MDColumn::new(1, "Name", MDColumnSize::Strings),
                MDColumn::new(2, "Signature", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodPtr,
            "MethodPtr",
            vec![
                MDColumn::new(0, "Method", MDColumnSize::Method),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Method,
            "Method",
            vec![
                MDColumn::new(0, "RVA", MDColumnSize::UInt32),
                MDColumn::new(1, "ImplFlags", MDColumnSize::UInt16),
                MDColumn::new(2, "Flags", MDColumnSize::UInt16),
                MDColumn::new(3, "Name", MDColumnSize::Strings),
                MDColumn::new(4, "Signature", MDColumnSize::Blob),
                MDColumn::new(5, "ParamList", MDColumnSize::Param),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ParamPtr,
            "ParamPtr",
            vec![
                MDColumn::new(0, "Param", MDColumnSize::Param),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Param,
            "Param",
            vec![
                MDColumn::new(0, "Flags", MDColumnSize::UInt16),
                MDColumn::new(1, "Sequence", MDColumnSize::UInt16),
                MDColumn::new(2, "Name", MDColumnSize::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::InterfaceImpl,
            "InterfaceImpl",
            vec![
                MDColumn::new(0, "Class", MDColumnSize::TypeDef),
                MDColumn::new(1, "Interface", MDColumnSize::TypeDefOrRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MemberRef,
            "MemberRef",
            vec![
                MDColumn::new(0, "Class", MDColumnSize::MemberRefParent),
                MDColumn::new(1, "Name", MDColumnSize::Strings),
                MDColumn::new(2, "Signature", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Constant,
            "Constant",
            vec![
                MDColumn::new(0, "Type", MDColumnSize::UInt16),
                MDColumn::new(1, "Padding", MDColumnSize::UInt16),
                MDColumn::new(2, "Parent", MDColumnSize::HasConstant),
                MDColumn::new(3, "Value", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::CustomAttribute,
            "CustomAttribute",
            vec![
                MDColumn::new(0, "Parent", MDColumnSize::HasCustomAttribute),
                MDColumn::new(1, "Type", MDColumnSize::CustomAttributeType),
                MDColumn::new(2, "Value", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::FieldMarshal,
            "FieldMarshal",
            vec![
                MDColumn::new(0, "Parent", MDColumnSize::HasFieldMarshal),
                MDColumn::new(1, "NativeType", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::DeclSecurity,
            "DeclSecurity",
            vec![
                MDColumn::new(0, "Action", MDColumnSize::UInt16),
                MDColumn::new(1, "Parent", MDColumnSize::HasDeclSecurity),
                MDColumn::new(2, "PermissionSet", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ClassLayout,
            "ClassLayout",
            vec![
                MDColumn::new(0, "PackingSize", MDColumnSize::UInt16),
                MDColumn::new(1, "ClassSize", MDColumnSize::UInt32),
                MDColumn::new(2, "Parent", MDColumnSize::TypeDef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::FieldLayout,
            "FieldLayout",
            vec![
                MDColumn::new(0, "OffSet", MDColumnSize::UInt32),
                MDColumn::new(1, "Field", MDColumnSize::Field),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::StandAloneSig,
            "StandAloneSig",
            vec![
                MDColumn::new(0, "Signature", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::EventMap,
            "EventMap",
            vec![
                MDColumn::new(0, "Parent", MDColumnSize::TypeDef),
                MDColumn::new(1, "EventList", MDColumnSize::Event),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::EventPtr,
            "EventPtr",
            vec![
                MDColumn::new(0, "Event", MDColumnSize::Event),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Event,
            "Event",
            vec![
                MDColumn::new(0, "EventFlags", MDColumnSize::UInt16),
                MDColumn::new(1, "Name", MDColumnSize::Strings),
                MDColumn::new(2, "EventType", MDColumnSize::TypeDefOrRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::PropertyMap,
            "PropertyMap",
            vec![
                MDColumn::new(0, "Parent", MDColumnSize::TypeDef),
                MDColumn::new(1, "PropertyList", MDColumnSize::Property),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::PropertyPtr,
            "PropertyPtr",
            vec![
                MDColumn::new(0, "Property", MDColumnSize::Property),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Property,
            "Property",
            vec![
                MDColumn::new(0, "PropFlags", MDColumnSize::UInt16),
                MDColumn::new(1, "Name", MDColumnSize::Strings),
                MDColumn::new(2, "Type", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodSemantics,
            "MethodSemantics",
            vec![
                MDColumn::new(0, "Semantics", MDColumnSize::UInt16),
                MDColumn::new(1, "Method", MDColumnSize::MethodDefOrRef),
                MDColumn::new(2, "Association", MDColumnSize::HasSemantic),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodImpl,
            "MethodImpl",
            vec![
                MDColumn::new(0, "Class", MDColumnSize::TypeDef),
                MDColumn::new(1, "MethodBody", MDColumnSize::MethodDefOrRef),
                MDColumn::new(2, "MethodDecl", MDColumnSize::MethodDefOrRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ModuleRef,
            "ModuleRef",
            vec![
                MDColumn::new(0, "Name", MDColumnSize::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::TypeSpec,
            "TypeSpec",
            vec![
                MDColumn::new(0, "Signature", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ImplMap,
            "ImplMap",
            vec![
                MDColumn::new(0, "MappingFlags", MDColumnSize::UInt16),
                MDColumn::new(1, "MemberForwarded", MDColumnSize::MemberForwarded),
                MDColumn::new(2, "ImportName", MDColumnSize::Strings),
                MDColumn::new(3, "ImportScope", MDColumnSize::ModuleRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::FieldRVA,
            "FieldRVA",
            vec![
                MDColumn::new(0, "RVA", MDColumnSize::UInt32),
                MDColumn::new(1, "Field", MDColumnSize::Field),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ENCLog,
            "ENCLog",
            vec![
                MDColumn::new(0, "Token", MDColumnSize::UInt32),
                MDColumn::new(1, "FuncCode", MDColumnSize::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ENCMap,
            "ENCMap",
            vec![
                MDColumn::new(0, "Token", MDColumnSize::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::Assembly,
            "Assembly",
            vec![
                MDColumn::new(0, "HashAlgId", MDColumnSize::UInt32),
                MDColumn::new(1, "MajorVersion", MDColumnSize::UInt16),
                MDColumn::new(2, "MinorVersion", MDColumnSize::UInt16),
                MDColumn::new(3, "BuildNumber", MDColumnSize::UInt16),
                MDColumn::new(4, "RevisionNumber", MDColumnSize::UInt16),
                MDColumn::new(5, "Flags", MDColumnSize::UInt32),
                MDColumn::new(6, "PublicKey", MDColumnSize::Blob),
                MDColumn::new(7, "Name", MDColumnSize::Strings),
                MDColumn::new(8, "Locale", MDColumnSize::Strings),
                MDColumn::new(9, "HashValue", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyProcessor,
            "AssemblyProcessor",
            vec![
                MDColumn::new(0, "Processor", MDColumnSize::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyOS,
            "AssemblyOS",
            vec![
                MDColumn::new(0, "OSPlatformId", MDColumnSize::UInt32),
                MDColumn::new(1, "OSMajorVersion", MDColumnSize::UInt32),
                MDColumn::new(2, "OSMinorVersion", MDColumnSize::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyRef,
            "AssemblyRef",
            vec![
                MDColumn::new(0, "MajorVersion", MDColumnSize::UInt16),
                MDColumn::new(1, "MinorVersion", MDColumnSize::UInt16),
                MDColumn::new(2, "BuildNumber", MDColumnSize::UInt16),
                MDColumn::new(3, "RevisionNumber", MDColumnSize::UInt16),
                MDColumn::new(4, "Flags", MDColumnSize::UInt32),
                MDColumn::new(5, "PublicKeyOrToken", MDColumnSize::Blob),
                MDColumn::new(6, "Name", MDColumnSize::Strings),
                MDColumn::new(7, "Locale", MDColumnSize::Strings),
                MDColumn::new(8, "HashValue", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyRefProcessor,
            "AssemblyRefProcessor",
            vec![
                MDColumn::new(0, "Processor", MDColumnSize::UInt32),
                MDColumn::new(1, "AssemblyRef", MDColumnSize::AssemblyRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::AssemblyRefOS,
            "AssemblyRefOS",
            vec![
                MDColumn::new(0, "OSPlatformId", MDColumnSize::UInt32),
                MDColumn::new(1, "OSMajorVersion", MDColumnSize::UInt32),
                MDColumn::new(2, "OSMinorVersion", MDColumnSize::UInt32),
                MDColumn::new(3, "AssemblyRef", MDColumnSize::AssemblyRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::File,
            "File",
            vec![
                MDColumn::new(0, "Flags", MDColumnSize::UInt32),
                MDColumn::new(1, "Name", MDColumnSize::Strings),
                MDColumn::new(2, "HashValue", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ExportedType,
            "ExportedType",
            vec![
                MDColumn::new(0, "Flags", MDColumnSize::UInt32),
                MDColumn::new(1, "TypeDefId", MDColumnSize::UInt32),
                MDColumn::new(2, "TypeName", MDColumnSize::Strings),
                MDColumn::new(3, "TypeNamespace", MDColumnSize::Strings),
                MDColumn::new(4, "Implementation", MDColumnSize::Implementation),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ManifestResource,
            "ManifestResource",
            vec![
                MDColumn::new(0, "Offset", MDColumnSize::UInt32),
                MDColumn::new(1, "Flags", MDColumnSize::UInt32),
                MDColumn::new(2, "Name", MDColumnSize::Strings),
                MDColumn::new(3, "Implementation", MDColumnSize::Implementation),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::NestedClass,
            "NestedClass",
            vec![
                MDColumn::new(0, "NestedClass", MDColumnSize::TypeDef),
                MDColumn::new(1, "EnclosingClass", MDColumnSize::TypeDef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::GenericParam,
            "GenericParam",
            vec![
                MDColumn::new(0, "Number", MDColumnSize::UInt16),
                MDColumn::new(1, "Flags", MDColumnSize::UInt16),
                MDColumn::new(2, "Owner", MDColumnSize::TypeOrMethodDef),
                MDColumn::new(3, "Name", MDColumnSize::Strings),
                MDColumn::new(4, "Kind", MDColumnSize::TypeDefOrRef),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodSpec,
            "MethodSpec",
            vec![
                MDColumn::new(0, "Method", MDColumnSize::MethodDefOrRef),
                MDColumn::new(1, "Instantiation", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::GenericParamConstraint,
            "GenericParamConstraint",
            vec![
                MDColumn::new(0, "Owner", MDColumnSize::GenericParam),
                MDColumn::new(1, "Constraint", MDColumnSize::TypeDefOrRef),
            ]
        ));
        tables.push(MDTable::new(MDTableType::X2D, "", vec![]));
        tables.push(MDTable::new(MDTableType::X2E, "", vec![]));
        tables.push(MDTable::new(MDTableType::X2F, "", vec![]));
        tables.push(MDTable::new(
            MDTableType::Document,
            "Document",
            vec![
                MDColumn::new(0, "Name", MDColumnSize::Strings),
                MDColumn::new(1, "HashAlgorithm", MDColumnSize::UInt32),
                MDColumn::new(2, "Hash", MDColumnSize::Blob),
                MDColumn::new(3, "Language", MDColumnSize::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::MethodDebugInformation,
            "MethodDebugInformation",
            vec![
                MDColumn::new(0, "Document", MDColumnSize::Document),
                MDColumn::new(1, "SequencePoints", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::LocalScope,
            "LocalScope",
            vec![
                MDColumn::new(0, "Method", MDColumnSize::Method),
                MDColumn::new(1, "ImportScope", MDColumnSize::ImportScope),
                MDColumn::new(2, "VariableList", MDColumnSize::LocalVariable),
                MDColumn::new(3, "ConstantList", MDColumnSize::LocalConstant),
                MDColumn::new(4, "StartOffset", MDColumnSize::UInt32),
                MDColumn::new(5, "Length", MDColumnSize::UInt32),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::LocalVariable,
            "LocalVariable",
            vec![
                MDColumn::new(0, "Attributes", MDColumnSize::UInt16),
                MDColumn::new(1, "Index", MDColumnSize::UInt16),
                MDColumn::new(2, "Name", MDColumnSize::Strings),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::LocalConstant,
            "LocalConstant",
            vec![
                MDColumn::new(0, "Name", MDColumnSize::Strings),
                MDColumn::new(1, "Signature", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::ImportScope,
            "ImportScope",
            vec![
                MDColumn::new(0, "Parent", MDColumnSize::ImportScope),
                MDColumn::new(1, "Imports", MDColumnSize::Blob),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::StateMachineMethod,
            "StateMachineMethod",
            vec![
                MDColumn::new(0, "MoveNextMethod", MDColumnSize::Method),
                MDColumn::new(1, "KickoffMethod", MDColumnSize::Method),
            ]
        ));
        tables.push(MDTable::new(
            MDTableType::CustomDebugInformation,
            "CustomDebugInformation",
            vec![
                MDColumn::new(0, "Parent", MDColumnSize::HasCustomDebugInformation),
                MDColumn::new(1, "Kind", MDColumnSize::GUID),
                MDColumn::new(2, "Value", MDColumnSize::Blob),
            ]
        ));

        max_present_tables
    }
}