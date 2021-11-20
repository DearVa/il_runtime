use num_traits::FromPrimitive;

use crate::interpreter::metadata::table_stream::MDType;
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

    pub fn to_table(token: u32) -> Option<MDType> {
        FromPrimitive::from_u32(MDToken::to_table_type(token))
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

    pub fn from_md_type(column_size: MDType) -> CodedToken {
        match column_size {
            MDType::TypeDefOrRef => CodedToken::new(2, vec![MDTableType::TypeDef, MDTableType::TypeRef, MDTableType::TypeSpec]),
            MDType::HasConstant => CodedToken::new(2, vec![MDTableType::Field, MDTableType::Param, MDTableType::Property]),
            MDType::HasCustomAttribute => CodedToken::new(5, vec![MDTableType::Method, MDTableType::Field, MDTableType::TypeRef, MDTableType::TypeDef, MDTableType::Param, MDTableType::InterfaceImpl, MDTableType::MemberRef, MDTableType::Module, MDTableType::DeclSecurity, MDTableType::Property, MDTableType::Event, MDTableType::StandAloneSig, MDTableType::ModuleRef, MDTableType::TypeSpec, MDTableType::Assembly, MDTableType::AssemblyRef, MDTableType::File, MDTableType::ExportedType, MDTableType::ManifestResource, MDTableType::GenericParam, MDTableType::GenericParamConstraint, MDTableType::MethodSpec, MDTableType::Module, MDTableType::Module]),
            MDType::HasFieldMarshal => CodedToken::new(1, vec![MDTableType::Field, MDTableType::Param]),
            MDType::HasDeclSecurity => CodedToken::new(2, vec![MDTableType::TypeDef, MDTableType::Method, MDTableType::Assembly]),
            MDType::MemberRefParent => CodedToken::new(3, vec![MDTableType::TypeDef, MDTableType::TypeRef, MDTableType::ModuleRef, MDTableType::Method, MDTableType::TypeSpec]),
            MDType::HasSemantic => CodedToken::new(1, vec![MDTableType::Event, MDTableType::Property]),
            MDType::MethodDefOrRef => CodedToken::new(1, vec![MDTableType::Method, MDTableType::MemberRef]),
            MDType::MemberForwarded => CodedToken::new(1, vec![MDTableType::Field, MDTableType::Method]),
            MDType::Implementation => CodedToken::new(2, vec![MDTableType::File, MDTableType::AssemblyRef, MDTableType::ExportedType]),
            MDType::CustomAttributeType => CodedToken::new(3, vec![MDTableType::Module, MDTableType::Module, MDTableType::Method, MDTableType::MemberRef]),
            MDType::ResolutionScope => CodedToken::new(2, vec![MDTableType::Module, MDTableType::ModuleRef, MDTableType::AssemblyRef, MDTableType::TypeRef]),
            MDType::TypeOrMethodDef => CodedToken::new(1, vec![MDTableType::TypeDef, MDTableType::Method]),
            MDType::HasCustomDebugInformation => CodedToken::new(5, vec![MDTableType::Method, MDTableType::Field, MDTableType::TypeRef, MDTableType::TypeDef, MDTableType::Param, MDTableType::InterfaceImpl, MDTableType::MemberRef, MDTableType::Module, MDTableType::DeclSecurity, MDTableType::Property, MDTableType::Event, MDTableType::StandAloneSig, MDTableType::ModuleRef, MDTableType::TypeSpec, MDTableType::Assembly, MDTableType::AssemblyRef, MDTableType::File, MDTableType::ExportedType, MDTableType::ManifestResource, MDTableType::GenericParam, MDTableType::GenericParamConstraint, MDTableType::MethodSpec, MDTableType::Document, MDTableType::LocalScope, MDTableType::LocalVariable, MDTableType::LocalConstant, MDTableType::ImportScope]),
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

    pub fn decode_as_md_table_type(&self, coded_token: u32) -> Option<MDTableType> {
        match self.decode(coded_token) {
            Some(token) => FromPrimitive::from_u8(MDToken::to_table_type(token) as u8),
            None => None,
        }
    }
}