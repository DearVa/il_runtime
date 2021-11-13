use super::metadata::md_token::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bitflags::bitflags;

use super::metadata::{CompressedStream, Metadata};

bitflags! {
    pub struct CallingConvention: u8 {
        const Default = 0x0;
		const C = 0x1;
		const StdCall = 0x2;
		const ThisCall = 0x3;
		const FastCall = 0x4;
		const VarArg = 0x5;
		const Field = 0x6;
		const LocalSig = 0x7;
		const Property = 0x8;
		// Unmanaged calling convention encoded as modopts
		const Unmanaged = 0x9;
		// generic method instantiation
		const GenericInst = 0xA;
		// used ONLY for 64bit vararg PInvoke calls
		const NativeVarArg = 0xB;
		// Calling convention is bottom 4 bits
		const Mask = 0x0F;
		// Generic method
		const Generic = 0x10;
		// Method needs a 'this' parameter
		const HasThis = 0x20;
		// 'this' parameter is the first arg if set (else it's hidden)
		const ExplicitThis = 0x40;
		// Used internally by the CLR
		const ReservedByCLR = 0x80;
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

pub enum CallingConventionSig {
    FieldSig(FieldSig),
    MethodSig(MethodSig),
    PropertySig(MethodSig),
    LocalSig(LocalSig),
}

impl CallingConventionSig {
    pub fn read_metadata_sig(metadata: &Metadata, signature: u32) -> Option<CallingConventionSig> {
        let data = metadata.blob_stream.read(signature).unwrap();
        let stream = CompressedStream::from_data(data);
        let mut offset = 0;
        Self::read_sig(&stream, &mut offset)
    }

    pub fn read_sig(stream: &CompressedStream, offset: &mut usize) -> Option<CallingConventionSig> {
        let calling_convention = CallingConvention::from_bits_truncate(stream.read_u8(offset).unwrap());
        match calling_convention.intersection(CallingConvention::Mask) {
            CallingConvention::Default | 
            CallingConvention::C |
            CallingConvention::StdCall |
            CallingConvention::ThisCall |
            CallingConvention::FastCall |
            CallingConvention::VarArg |
            CallingConvention::NativeVarArg |
            CallingConvention::Unmanaged => {
                Self::read_method(calling_convention, &stream)
            },
            CallingConvention::Field => {
                Self::read_field(calling_convention, &stream)
            },
            CallingConvention::Property => {
                Self::read_property(calling_convention, &stream)
            },
            CallingConvention::LocalSig => {
                todo!();
                // Self::read_local_sig(calling_convention, &stream)
            },
            _=> None
        }
    }

    fn read_method(calling_convention: CallingConvention, stream: &CompressedStream) -> Option<CallingConventionSig> {
        Self::read_sig_internal(MethodSig::new(calling_convention), stream)
    }

    fn read_field(calling_convention: CallingConvention, stream: &CompressedStream) -> Option<CallingConventionSig> {
        let mut offset = 0;
        Some(CallingConventionSig::FieldSig(FieldSig::new(calling_convention, TypeSig::read_type(stream, &mut offset))))
    }

    fn read_property(calling_convention: CallingConvention, stream: &CompressedStream) -> Option<CallingConventionSig> {
        match Self::read_sig_internal(MethodSig::new(calling_convention), stream) {
            Some(CallingConventionSig::MethodSig(method_sig)) => Some(CallingConventionSig::PropertySig(method_sig)),
            _ => None
        }
    }

    fn read_sig_internal(method_sig: MethodSig, stream: &CompressedStream) -> Option<CallingConventionSig> {
        let mut offset = 1;
        let mut method_sig = method_sig;
        if method_sig.base.base.get_generic() {
            match stream.try_read_compressed_u32(&mut offset) {
                Ok(count) => {
                    method_sig.base.gen_param_count = count;
                },
                _ => return None,
            }
        }
        match stream.try_read_compressed_u32(&mut offset) {
            Ok(param_count) => {
                method_sig.base.ret_type = TypeSig::read_type(stream, &mut offset);
                for _ in 0..param_count {
                    match TypeSig::read_type(stream, &mut offset) {
                        // TypeSig::SentinelSig(_) => {
                        //     if method_sig.base.params_after_sentinel
                        // },
                        Some(type_sig) => method_sig.base.parameters.push(type_sig),
                        _ => return None,
                    }
                }
                return Some(CallingConventionSig::MethodSig(method_sig));
            },
            _ => return None,
        }
        None
    }
}

pub struct CallingConventionSigBase {
    pub calling_convention: CallingConvention,
    pub extra_data: Vec<u8>,
}

impl Default for CallingConventionSigBase {
    fn default() -> CallingConventionSigBase {
        CallingConventionSigBase {
            calling_convention: CallingConvention::Default,
            extra_data: Vec::new(),
        }
    }
}

impl CallingConventionSigBase {
    pub fn new(calling_convention: CallingConvention) -> CallingConventionSigBase {
        CallingConventionSigBase {
            calling_convention: calling_convention,
            extra_data: Vec::new(),
        }
    }

    pub fn get_is_default(&self) -> bool {
        self.calling_convention.intersection(CallingConvention::Mask) == CallingConvention::Default
    }

    pub fn get_generic(&self) -> bool {
        self.calling_convention.contains(CallingConvention::Generic)
    }

    pub fn set_generic(&mut self, value: bool) {
        if value {
            self.calling_convention.insert(CallingConvention::Generic);
        } else {
            self.calling_convention.remove(CallingConvention::Generic);
        }
    }

    pub fn get_has_this(&self) -> bool {
        self.calling_convention.contains(CallingConvention::HasThis)
    }

    pub fn set_has_this(&mut self, value: bool) {
        if value {
            self.calling_convention.insert(CallingConvention::HasThis);
        } else {
            self.calling_convention.remove(CallingConvention::HasThis);
        }
    }

    pub fn get_explicit_this(&self) -> bool {
        self.calling_convention.contains(CallingConvention::ExplicitThis)
    }

    pub fn set_explicit_this(&mut self, value: bool) {
        if value {
            self.calling_convention.insert(CallingConvention::ExplicitThis);
        } else {
            self.calling_convention.remove(CallingConvention::ExplicitThis);
        }
    }
}

pub enum TypeSig {
    TypeSig(TypeSigBase),
    LeafSig(LeafSig),
    NoLeafSig(NoLeafSig),
    CorLibType(CorLibType),
    PtrSig(NoLeafSig),
    ByRefSig(NoLeafSig),
    ValueTypeSig(TypeDefOrRefSig),
    ClassSig(TypeDefOrRefSig),
    ArraySig(ArraySigBase),
    SZArraySig(ArraySigBase),
    CModReqdSig(NoLeafSig),
    CModOptSig(NoLeafSig),
    PinnedSig(NoLeafSig),
    SentinelSig(LeafSig),
    FnPtrSig(FnPtrSig),
    GenericInstSig(LeafSig),
    GenericVar(GenericSig),
}

impl TypeSig {
    pub fn read_type(stream: &CompressedStream, offset: &mut usize) -> Option<TypeSig> {
        // let mut num;
        // let mut i;
        // let mut next_type;
        // let mut result = None;
        match stream.read_u8(offset) {
            Ok(val) => {
                match FromPrimitive::from_u8(val) {
                    Some(ElementType::Void) => Some(TypeSig::CorLibType(CorLibType::Void)),
                    Some(ElementType::Boolean) => Some(TypeSig::CorLibType(CorLibType::Boolean)),
                    Some(ElementType::Char) => Some(TypeSig::CorLibType(CorLibType::Char)),
                    Some(ElementType::I1) => Some(TypeSig::CorLibType(CorLibType::SByte)),
                    Some(ElementType::U1) => Some(TypeSig::CorLibType(CorLibType::Byte)),
                    Some(ElementType::I2) => Some(TypeSig::CorLibType(CorLibType::Int16)),
                    Some(ElementType::U2) => Some(TypeSig::CorLibType(CorLibType::UInt16)),
                    Some(ElementType::I4) => Some(TypeSig::CorLibType(CorLibType::Int32)),
                    Some(ElementType::U4) => Some(TypeSig::CorLibType(CorLibType::UInt32)),
                    Some(ElementType::I8) => Some(TypeSig::CorLibType(CorLibType::Int64)),
                    Some(ElementType::U8) => Some(TypeSig::CorLibType(CorLibType::UInt64)),
                    Some(ElementType::R4) => Some(TypeSig::CorLibType(CorLibType::Single)),
                    Some(ElementType::R8) => Some(TypeSig::CorLibType(CorLibType::Double)),
                    Some(ElementType::String) => Some(TypeSig::CorLibType(CorLibType::String)),
                    Some(ElementType::TypedByRef) => Some(TypeSig::CorLibType(CorLibType::TypedRef)),
                    Some(ElementType::I) => Some(TypeSig::CorLibType(CorLibType::IntPtr)),
                    Some(ElementType::U) => Some(TypeSig::CorLibType(CorLibType::UIntPtr)),
                    Some(ElementType::Object) => Some(TypeSig::CorLibType(CorLibType::Object)),

                    Some(ElementType::Ptr) => Some(TypeSig::PtrSig(NoLeafSig::new(Self::read_type(stream, offset)?))),
                    Some(ElementType::ByRef) => Some(TypeSig::ByRefSig(NoLeafSig::new(Self::read_type(stream, offset)?))),
                    Some(ElementType::ValueType) => Some(TypeSig::ValueTypeSig(Self::read_type_def_or_ref(false, stream, offset))),
                    Some(ElementType::Class) => Some(TypeSig::ClassSig(Self::read_type_def_or_ref(false, stream, offset))),
                    Some(ElementType::FnPtr) => Some(TypeSig::FnPtrSig(FnPtrSig::new(CallingConventionSig::read_sig(stream, offset)))),
                    Some(ElementType::SZArray) => Some(TypeSig::SZArraySig(ArraySigBase::new(Self::read_type(stream, offset)?))),
                    Some(ElementType::CModReqd) => Some(TypeSig::CModReqdSig(NoLeafSig::new(Self::read_type(stream, offset)?))),
                    Some(ElementType::CModOpt) => Some(TypeSig::CModOptSig(NoLeafSig::new(Self::read_type(stream, offset)?))),
                    Some(ElementType::Sentinel) => Some(TypeSig::SentinelSig(LeafSig::default())),
                    Some(ElementType::Pinned) => Some(TypeSig::PinnedSig(NoLeafSig::new(Self::read_type(stream, offset)?))),
                
                    Some(ElementType::Var) => {
                        todo!();
                    },
                    Some(ElementType::MVar) => {
                        todo!();
                    },
                    Some(ElementType::GenericInst) => {
                        todo!();
                    },
                    Some(ElementType::Array) => {
                        todo!();
                    },
                    Some(ElementType::Internal) => {
                        let address = stream.read_u64(offset).unwrap();
                        println!("address: {}", address);
                        None
                    },

                    _ => None
                }
            },
            _ => None
        }
    }

    fn read_type_def_or_ref(allow_type_spec: bool, stream: &CompressedStream, offset: &mut usize) -> TypeDefOrRefSig {
        // let coded_token = stream.try_read_compressed_u32(offset).unwrap();
        // CodedToken::from_column_size(CodedToken::TypeDefOrRef).decode(coded_token)
        todo!();
    }
}

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
    TypedRef,
    IntPtr,
    UIntPtr,
    Object,
}

#[derive(Default)]
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

#[derive(Default)]
pub struct LeafSig {
    pub base: TypeSigBase,
    pub next : Option<Box<TypeSig>>,
}

pub struct GenericSig {
    pub base: LeafSig,
    pub number: u32,
    pub param_provider: u32,  // TypeOrMethodDef
}

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

#[derive(Default)]
pub struct TypeDefOrRefSig {
    pub base: LeafSig,
    pub type_def_or_ref: i32,
}

impl TypeDefOrRefSig {
    pub fn new(type_def_or_ref: i32) -> TypeDefOrRefSig {
        TypeDefOrRefSig {
            base: LeafSig::default(),
            type_def_or_ref,
        }
    }
}

pub struct CorLibTypeSig {
    pub base: TypeDefOrRefSig,
    pub element_type: ElementType,
}

impl CorLibTypeSig {
    pub fn new(type_def_or_ref_tag: i32, element_type: ElementType) -> CorLibTypeSig {
        CorLibTypeSig {
            base: TypeDefOrRefSig::new(type_def_or_ref_tag),
            element_type,
        }
    }
}

impl Default for CorLibTypeSig {
    fn default() -> Self {
        CorLibTypeSig {
            base: TypeDefOrRefSig::default(),
            element_type: ElementType::Void,
        }
    }
}

#[derive(Default)]
pub struct NoLeafSig {
    pub base: TypeSigBase,
    pub nextSig : Option<Box<TypeSig>>,
}

impl NoLeafSig {
    pub fn new(next_sig: TypeSig) -> NoLeafSig {
        NoLeafSig {
            base: TypeSigBase::new(),
            nextSig: Some(Box::new(next_sig)),
        }
    }
}

#[derive(Default)]
pub struct ArraySigBase {
    pub base: NoLeafSig,
    pub rank: u32,
}

impl ArraySigBase {
    pub fn new(array_type: TypeSig) -> ArraySigBase {
        ArraySigBase {
            base: NoLeafSig::new(array_type),
            rank: 0,
        }
    }
}

pub struct FieldSig {
    pub base: CallingConventionSigBase,
    pub type_sig: Option<TypeSig>,
}

impl FieldSig {
    pub fn new(calling_convention: CallingConvention, type_sig: Option<TypeSig>) -> FieldSig {
        FieldSig {
            base: CallingConventionSigBase::new(calling_convention),
            type_sig,
        }
    }
}

#[derive(Default)]
pub struct MethodBaseSig {
    pub base: CallingConventionSigBase,
    pub ret_type: Option<TypeSig>,
    pub parameters: Vec<TypeSig>,
    pub gen_param_count: u32,
    pub params_after_sentinel: Vec<TypeSig>,
}

impl MethodBaseSig {
    pub fn new(calling_convention: CallingConvention) -> MethodBaseSig {
        MethodBaseSig {
            base: CallingConventionSigBase::new(calling_convention),
            ret_type: None,
            parameters: Vec::new(),
            gen_param_count: 0,
            params_after_sentinel: Vec::new(),
        }
    }
}

pub struct MethodSig {
    pub base: MethodBaseSig,
    pub origin_token: u32,
}

impl MethodSig {
    pub fn new(calling_convention: CallingConvention) -> MethodSig {
        MethodSig {
            base: MethodBaseSig::new(calling_convention),
            origin_token: 0,
        }
    }
}

pub struct LocalSig {
    pub base: CallingConventionSigBase,
    pub locals: Vec<TypeSig>,
}