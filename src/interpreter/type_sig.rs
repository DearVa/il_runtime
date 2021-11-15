use std::fmt::{Display, Error, Formatter};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::interpreter::data_reader::DataReader;
use super::{metadata::{md_token::{CodedToken, MDToken}, table_stream::{MDColumnType, MDTableType}}, signature::CallingConventionSig};

#[derive(PartialEq, Eq)]
pub enum TypeSig {
    TypeSig(TypeSigBase),
    LeafSig(LeafSig),
    TypeDefOrRefSig(TypeDefOrRefSig),
    CorLibTypeSig(CorLibType),
    ClassOrValueTypeSig(ClassOrValueTypeSig),
    ValueTypeSig(Option<ClassOrValueTypeSig>),
    ClassSig(Option<ClassOrValueTypeSig>),
    GenericSig(GenericSig),
    GenericVar(GenericSig),
    GenericMVar(GenericSig),
    SentinelSig(LeafSig),
    FnPtrSig(FnPtrSig),
    GenericInstSig(GenericInstSig),
    NoLeafSig(NoLeafSig),
    PtrSig(NoLeafSig),
    ByRefSig(NoLeafSig),
    ArraySig(ArraySig),
    SZArraySig(ArraySigBase),
    ModifierSig(ModifierSig),
    CModReqdSig(NoLeafSig),
    CModOptSig(NoLeafSig),
    PinnedSig(NoLeafSig),
    ValueArraySig(ValueArraySig),
    ModuleSig(ModuleSig),
}

impl TypeSig {
    const MAX_ARRAY_RANK: u32 = 64;

    pub fn read_type(reader: &DataReader, offset: &mut usize) -> Option<TypeSig> {
        // let mut num;
        // let mut i;
        // let mut next_type;
        // let mut result = None;
        match reader.read_u8_immut(offset) {
            Ok(val) => {
                match FromPrimitive::from_u8(val) {
                    Some(ElementType::Void) => Some(TypeSig::CorLibTypeSig(CorLibType::Void)),
                    Some(ElementType::Boolean) => Some(TypeSig::CorLibTypeSig(CorLibType::Boolean)),
                    Some(ElementType::Char) => Some(TypeSig::CorLibTypeSig(CorLibType::Char)),
                    Some(ElementType::I1) => Some(TypeSig::CorLibTypeSig(CorLibType::SByte)),
                    Some(ElementType::U1) => Some(TypeSig::CorLibTypeSig(CorLibType::Byte)),
                    Some(ElementType::I2) => Some(TypeSig::CorLibTypeSig(CorLibType::Int16)),
                    Some(ElementType::U2) => Some(TypeSig::CorLibTypeSig(CorLibType::UInt16)),
                    Some(ElementType::I4) => Some(TypeSig::CorLibTypeSig(CorLibType::Int32)),
                    Some(ElementType::U4) => Some(TypeSig::CorLibTypeSig(CorLibType::UInt32)),
                    Some(ElementType::I8) => Some(TypeSig::CorLibTypeSig(CorLibType::Int64)),
                    Some(ElementType::U8) => Some(TypeSig::CorLibTypeSig(CorLibType::UInt64)),
                    Some(ElementType::R4) => Some(TypeSig::CorLibTypeSig(CorLibType::Single)),
                    Some(ElementType::R8) => Some(TypeSig::CorLibTypeSig(CorLibType::Double)),
                    Some(ElementType::String) => Some(TypeSig::CorLibTypeSig(CorLibType::String)),
                    Some(ElementType::TypedByRef) => Some(TypeSig::CorLibTypeSig(CorLibType::TypedReference)),
                    Some(ElementType::I) => Some(TypeSig::CorLibTypeSig(CorLibType::IntPtr)),
                    Some(ElementType::U) => Some(TypeSig::CorLibTypeSig(CorLibType::UIntPtr)),
                    Some(ElementType::Object) => Some(TypeSig::CorLibTypeSig(CorLibType::Object)),

                    Some(ElementType::Ptr) => Some(TypeSig::PtrSig(NoLeafSig::new(Self::read_type(reader, offset)))),
                    Some(ElementType::ByRef) => Some(TypeSig::ByRefSig(NoLeafSig::new(Self::read_type(reader, offset)))),
                    Some(ElementType::ValueType) => Some(TypeSig::ValueTypeSig(Self::read_class_or_value_type(false, reader, offset))),
                    Some(ElementType::Class) => Some(TypeSig::ClassSig(Self::read_class_or_value_type(false, reader, offset))),
                    Some(ElementType::FnPtr) => Some(TypeSig::FnPtrSig(FnPtrSig::new(CallingConventionSig::read_sig(reader, offset)))),
                    Some(ElementType::SZArray) => Some(TypeSig::SZArraySig(ArraySigBase::new(Self::read_type(reader, offset), 0))),
                    Some(ElementType::CModReqd) => Some(TypeSig::CModReqdSig(NoLeafSig::new(Self::read_type(reader, offset)))),
                    Some(ElementType::CModOpt) => Some(TypeSig::CModOptSig(NoLeafSig::new(Self::read_type(reader, offset)))),
                    Some(ElementType::Sentinel) => Some(TypeSig::SentinelSig(LeafSig::default())),
                    Some(ElementType::Pinned) => Some(TypeSig::PinnedSig(NoLeafSig::new(Self::read_type(reader, offset)))),
                
                    Some(ElementType::Var) => {
                        let num = reader.try_read_compressed_u32_immut(offset)?;
                        Some(TypeSig::GenericVar(GenericSig::new(true, num, 0)))
                    },
                    Some(ElementType::MVar) => {
                        let num = reader.try_read_compressed_u32_immut(offset)?;
                        Some(TypeSig::GenericMVar(GenericSig::new(false, num, 0)))
                    },
                    Some(ElementType::ValueArray) => {
                        let next_type = Self::read_type(reader, offset);
                        let num = reader.try_read_compressed_u32_immut(offset)?;
                        Some(TypeSig::ValueArraySig(ValueArraySig::new(next_type, num)))
                    },
                    Some(ElementType::Module) => {
                        let num =  reader.try_read_compressed_u32_immut(offset)?;
                        Some(TypeSig::ModuleSig(ModuleSig::new(num, Self::read_type(reader, offset))))
                    },
                    Some(ElementType::GenericInst) => {
                        let next_type = Self::read_type(reader, offset)?;
                        let num =  reader.try_read_compressed_u32_immut(offset)?;
                        let generic_type = match next_type {
                            TypeSig::ClassOrValueTypeSig(generic_type) => {
                                Some(generic_type)
                            },
                            _ => None
                        };
                        let mut args = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            args.push(Box::new(Self::read_type(reader, offset)?));
                        }
                        Some(TypeSig::GenericInstSig(GenericInstSig::new(generic_type, args)))
                    },
                    Some(ElementType::Array) => {
                        let next_type = Self::read_type(reader, offset);
                        let rank = reader.try_read_compressed_u32_immut(offset)?;
                        if rank > Self::MAX_ARRAY_RANK {
                            return None;
                        }
                        if rank == 0 {
                            return Some(TypeSig::ArraySig(ArraySig::new_zero(next_type)));
                        }
                        let num = reader.try_read_compressed_u32_immut(offset)?;
                        if num > Self::MAX_ARRAY_RANK {
                            return None;
                        }
                        let mut sizes = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            sizes.push(reader.try_read_compressed_u32_immut(offset)?);
                        }
                        let num = reader.try_read_compressed_u32_immut(offset)?;
                        if num > Self::MAX_ARRAY_RANK {
                            return None;
                        }
                        let mut lower_bounds = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            lower_bounds.push(reader.try_read_compressed_i32_immut(offset)?);
                        }
                        Some(TypeSig::ArraySig(ArraySig::new(next_type, rank, sizes, lower_bounds)))
                    },
                    Some(ElementType::Internal) => {
                        let address = reader.read_u64_immut(offset).unwrap();
                        println!("address: {}", address);
                        None
                    },

                    _ => None
                }
            },
            _ => None
        }
    }

    fn read_type_def_or_ref(allow_type_spec: bool, reader: &DataReader, offset: &mut usize) -> Option<TypeDefOrRefSig> {
        let coded_token =  reader.try_read_compressed_u32_immut(offset)?;
        match CodedToken::from_column_size(MDColumnType::TypeDefOrRef).decode_as_md_table_type(coded_token) {
            None => None,
            Some(table_type) => {
                if !allow_type_spec && table_type == MDTableType::TypeDef {
                    None
                } else {
                    Some(TypeDefOrRefSig::new(MDToken::to_table_type(coded_token)))
                }
            },
        }
    }

    fn read_class_or_value_type(allow_type_spec: bool, reader: &DataReader, offset: &mut usize) -> Option<ClassOrValueTypeSig> {
        let type_def_or_ref = Self::read_type_def_or_ref(allow_type_spec, reader, offset)?;
        Some(ClassOrValueTypeSig {
            base: type_def_or_ref
        })
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ElementType {
    /// <summary/>
    End			= 0x00,
    /// <summary>System.Void</summary>
    Void		= 0x01,
    /// <summary>System.Boolean</summary>
    Boolean		= 0x02,
    /// <summary>System.Char</summary>
    Char		= 0x03,
    /// <summary>System.SByte</summary>
    I1			= 0x04,
    /// <summary>System.Byte</summary>
    U1 			= 0x05,
    /// <summary>System.Int16</summary>
    I2 			= 0x06,
    /// <summary>System.UInt16</summary>
    U2 			= 0x07,
    /// <summary>System.Int32</summary>
    I4 			= 0x08,
    /// <summary>System.UInt32</summary>
    U4			= 0x09,
    /// <summary>System.Int64</summary>
    I8			= 0x0A,
    /// <summary>System.UInt64</summary>
    U8			= 0x0B,
    /// <summary>System.Single</summary>
    R4			= 0x0C,
    /// <summary>System.Double</summary>
    R8			= 0x0D,
    /// <summary>System.String</summary>
    String		= 0x0E,
    /// <summary>Pointer type (*)</summary>
    Ptr			= 0x0F,
    /// <summary>ByRef type (&amp;)</summary>
    ByRef		= 0x10,
    /// <summary>Value type</summary>
    ValueType	= 0x11,
    /// <summary>Reference type</summary>
    Class		= 0x12,
    /// <summary>Type generic parameter</summary>
    Var			= 0x13,
    /// <summary>Multidimensional array ([*], [,], [,,], ...)</summary>
    Array		= 0x14,
    /// <summary>Generic instance type</summary>
    GenericInst	= 0x15,
    /// <summary>Typed byref</summary>
    TypedByRef	= 0x16,
    /// <summary>Value array (don't use)</summary>
    ValueArray	= 0x17,
    /// <summary>System.IntPtr</summary>
    I			= 0x18,
    /// <summary>System.UIntPtr</summary>
    U			= 0x19,
    /// <summary>native real (don't use)</summary>
    R			= 0x1A,
    /// <summary>Function pointer</summary>
    FnPtr		= 0x1B,
    /// <summary>System.Object</summary>
    Object		= 0x1C,
    /// <summary>Single-dimension, zero lower bound array ([])</summary>
    SZArray		= 0x1D,
    /// <summary>Method generic parameter</summary>
    MVar		= 0x1E,
    /// <summary>Required C modifier</summary>
    CModReqd	= 0x1F,
    /// <summary>Optional C modifier</summary>
    CModOpt		= 0x20,
    /// <summary>Used internally by the CLR (don't use)</summary>
    Internal	= 0x21,
    /// <summary>Module (don't use)</summary>
    Module		= 0x3F,
    /// <summary>Sentinel (method sigs only)</summary>
    Sentinel	= 0x41,
    /// <summary>Pinned type (locals only)</summary>
    Pinned		= 0x45,
}

impl Display for TypeSig {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            TypeSig::CorLibTypeSig(val) => write!(f, "{:?}", val),
            _ => write!(f, "Non-CorLib Type")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CorLibType {
    Void,
    Boolean,
    Char,
    SByte,
    Byte,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Single,
    Double,
    String,
    TypedReference,
    IntPtr,
    UIntPtr,
    Object,
}

#[derive(Default, PartialEq, Eq)]
pub struct TypeSigBase {
    pub rid: u32,
}

impl TypeSigBase {
    pub fn new() -> TypeSigBase {
        TypeSigBase {
            rid: 0,
        }
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct LeafSig {
    pub base: TypeSigBase,
    pub next : Option<Box<TypeSig>>,
}

#[derive(Default, PartialEq, Eq)]
pub struct TypeDefOrRefSig {
    pub base: LeafSig,
    // 这条指向TypeDef或者TypeRef表的一项
    pub token: u32,
}

impl TypeDefOrRefSig {
    pub fn new(type_def_or_ref: u32) -> TypeDefOrRefSig {
        TypeDefOrRefSig {
            base: LeafSig::default(),
            token: type_def_or_ref,
        }
    }
}

/// 特殊，初始化时仅仅记录cor_lib_type，TypeDefOrRefSig需要解析
#[derive(PartialEq, Eq)]
pub struct CorLibTypeSig {
    pub base: TypeDefOrRefSig,
    pub cor_lib_type: CorLibType,
}

#[derive(PartialEq, Eq)]
pub struct ClassOrValueTypeSig {
    pub base: TypeDefOrRefSig,
}

// ValueTypeSig省略，继承ClassOrValueTypeSig而没有新成员

// ClassSig省略，继承ClassOrValueTypeSig而没有新成员

#[derive(PartialEq, Eq)]
pub struct GenericSig {
    pub base: LeafSig,
    pub is_type_var: bool,
    pub number: u32,
    /// ITypeOrMethodDef
    pub param_provider: u32,
}

impl GenericSig {
    pub fn new(is_type_var: bool, number: u32, param_provider: u32) -> GenericSig {
        GenericSig {
            base: LeafSig::default(),
            is_type_var,
            number,
            param_provider,
        }
    }
}

// GenericVar省略，继承GenericSig而没有新成员

// GenericMVar省略，继承GenericSig而没有新成员

// SentinelSig省略，继承LeafSig而没有新成员

#[derive(PartialEq, Eq)]
pub struct FnPtrSig {
    pub base: LeafSig,
    pub calling_convention_sig: Option<Box<CallingConventionSig>>,
}

impl FnPtrSig {
    pub fn new(calling_convention_sig: Option<CallingConventionSig>) -> FnPtrSig {
        FnPtrSig {
            base: LeafSig::default(),
            calling_convention_sig: match calling_convention_sig {
                Some(val) => Some(Box::new(val)),
                None => None,
            },
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct GenericInstSig {
    pub base: LeafSig,
    pub generic_type: Option<ClassOrValueTypeSig>,
    pub generic_args: Vec<Box<TypeSig>>,
}

impl GenericInstSig {
    pub fn new(generic_type: Option<ClassOrValueTypeSig>, generic_args: Vec<Box<TypeSig>>) -> GenericInstSig {
        GenericInstSig {
            base: LeafSig::default(),
            generic_type,
            generic_args,
        }
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct NoLeafSig {
    pub base: TypeSigBase,
    pub nextSig : Option<Box<TypeSig>>,
}

impl NoLeafSig {
    pub fn new(next_sig: Option<TypeSig>) -> NoLeafSig {
        NoLeafSig {
            base: TypeSigBase::new(),
            nextSig: match next_sig {
                Some(val) => Some(Box::new(val)),
                None => None,
            },
        }
    }
}

// PtrSig省略，继承NoLeafSig而没有新成员

// ByRefSig省略，继承NoLeafSig而没有新成员

#[derive(Default, PartialEq, Eq)]
pub struct ArraySigBase {
    pub base: NoLeafSig,
    pub rank: u32,
}

impl ArraySigBase {
    pub fn new(array_type: Option<TypeSig>, rank: u32) -> ArraySigBase {
        ArraySigBase {
            base: NoLeafSig::new(array_type),
            rank,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct ArraySig {
    pub base: ArraySigBase,
    pub sizes: Vec<u32>,
    pub lower_bounds: Vec<i32>,
}

impl ArraySig {
    pub fn new_zero(array_type: Option<TypeSig>) -> ArraySig {
        ArraySig {
            base: ArraySigBase::new(array_type, 0),
            sizes: vec![],
            lower_bounds: vec![],
        }
    }

    pub fn new(array_type: Option<TypeSig>, rank: u32, sizes: Vec<u32>, lower_bounds: Vec<i32>) -> ArraySig {
        ArraySig {
            base: ArraySigBase::new(array_type, rank),
            sizes,
            lower_bounds,
        }
    }
}

// SZArraySig省略，继承ArraySigBase而没有新成员

#[derive(PartialEq, Eq)]
pub struct ModifierSig {
    pub base: NoLeafSig,
    /// ITypeOrMethodDef
    pub modifier: u32,
}

// CModReqdSig省略，继承ModifierSig而没有新成员

// CModOptSig省略，继承ModifierSig而没有新成员

// PinnedSig省略，继承NonLeafSig而没有新成员

#[derive(PartialEq, Eq)]
pub struct ValueArraySig {
    pub base: NoLeafSig,
    pub size: u32,
}

impl ValueArraySig {
    pub fn new(next_sig: Option<TypeSig>, size: u32) -> ValueArraySig {
        ValueArraySig {
            base: NoLeafSig::new(next_sig),
            size,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct ModuleSig {
    pub base: NoLeafSig,
    pub index: u32,
}

impl ModuleSig {
    pub fn new(index: u32, next_sig: Option<TypeSig>) -> ModuleSig {
        ModuleSig {
            base: NoLeafSig::new(next_sig),
            index,
        }
    }
}