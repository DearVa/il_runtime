use crate::interpreter::metadata::table_stream::MDColumnType;
use super::table_stream::MDTableType;

pub struct MDToken {
    pub token: u32,
}

impl MDToken {
    pub const RID_MASK: u32 = 0x00FFFFFF;
    pub const RID_MAX: u32 = 0x00FFFFFF;
    pub const TABLE_SHIFT: i32 = 24;

    pub fn to_rid(token: u32) -> u32 {
        token & MDToken::RID_MASK
    }

    pub fn to_table_type(token: u32) -> u32 {
        token >> MDToken::TABLE_SHIFT
    }
}

pub struct CodedToken {
    pub bits: i32,
    pub mask: i32,
    pub table_types: Vec<MDTableType>,
}

impl CodedToken {
    pub fn new(bits: i32, table_types: Vec<MDTableType>) -> CodedToken {
        CodedToken {
            bits,
            mask: (1 << bits) - 1,
            table_types,
        }
    }

    pub fn from_column_size(column_size: MDColumnType) -> CodedToken {
        match column_size {
            MDColumnType::TypeDefOrRef => CodedToken::new(2, vec![MDTableType::TypeDef, MDTableType::TypeRef, MDTableType::TypeSpec]),
            MDColumnType::HasConstant => CodedToken::new(2, vec![MDTableType::Field, MDTableType::Param, MDTableType::Property]),
            MDColumnType::HasCustomAttribute => CodedToken::new(5, vec![MDTableType::Method, MDTableType::Field, MDTableType::TypeRef, MDTableType::TypeDef, MDTableType::Param, MDTableType::InterfaceImpl, MDTableType::MemberRef, MDTableType::Module, MDTableType::DeclSecurity, MDTableType::Property, MDTableType::Event, MDTableType::StandAloneSig, MDTableType::ModuleRef, MDTableType::TypeSpec, MDTableType::Assembly, MDTableType::AssemblyRef, MDTableType::File, MDTableType::ExportedType, MDTableType::ManifestResource, MDTableType::GenericParam, MDTableType::GenericParamConstraint, MDTableType::MethodSpec, MDTableType::Module, MDTableType::Module]),
            MDColumnType::HasFieldMarshal => CodedToken::new(1, vec![MDTableType::Field, MDTableType::Param]),
            MDColumnType::HasDeclSecurity => CodedToken::new(2, vec![MDTableType::TypeDef, MDTableType::Method, MDTableType::Assembly]),
            MDColumnType::MemberRefParent => CodedToken::new(3, vec![MDTableType::TypeDef, MDTableType::TypeRef, MDTableType::ModuleRef, MDTableType::Method, MDTableType::TypeSpec]),
            MDColumnType::HasSemantic => CodedToken::new(1, vec![MDTableType::Event, MDTableType::Property]),
            MDColumnType::MethodDefOrRef => CodedToken::new(1, vec![MDTableType::Method, MDTableType::MemberRef]),
            MDColumnType::MemberForwarded => CodedToken::new(1, vec![MDTableType::Field, MDTableType::Method]),
            MDColumnType::Implementation => CodedToken::new(2, vec![MDTableType::File, MDTableType::AssemblyRef, MDTableType::ExportedType]),
            MDColumnType::CustomAttributeType => CodedToken::new(3, vec![MDTableType::Module, MDTableType::Module, MDTableType::Method, MDTableType::MemberRef]),
            MDColumnType::ResolutionScope => CodedToken::new(2, vec![MDTableType::Module, MDTableType::ModuleRef, MDTableType::AssemblyRef, MDTableType::TypeRef]),
            MDColumnType::TypeOrMethodDef => CodedToken::new(1, vec![MDTableType::TypeDef, MDTableType::Method]),
            MDColumnType::HasCustomDebugInformation => CodedToken::new(5, vec![MDTableType::Method, MDTableType::Field, MDTableType::TypeRef, MDTableType::TypeDef, MDTableType::Param, MDTableType::InterfaceImpl, MDTableType::MemberRef, MDTableType::Module, MDTableType::DeclSecurity, MDTableType::Property, MDTableType::Event, MDTableType::StandAloneSig, MDTableType::ModuleRef, MDTableType::TypeSpec, MDTableType::Assembly, MDTableType::AssemblyRef, MDTableType::File, MDTableType::ExportedType, MDTableType::ManifestResource, MDTableType::GenericParam, MDTableType::GenericParamConstraint, MDTableType::MethodSpec, MDTableType::Document, MDTableType::LocalScope, MDTableType::LocalVariable, MDTableType::LocalConstant, MDTableType::ImportScope]),
            _ => panic!("Invalid column size"),
        }
    }

    pub fn decode(&self, coded_token: u32) -> Option<u32> {
        let rid = coded_token >> self.bits;
        let index = coded_token as i32 & self.mask;
        if rid > MDToken::RID_MAX as u32 || index >= self.table_types.len() as i32 {
            return None;
        }
        Some(((self.table_types[index as usize] as u32) << MDToken::TABLE_SHIFT) | rid)
    }
}