use std::fmt::{Debug, Display, Error, Formatter};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bitflags::bitflags;

use super::{data_reader::DataReader, metadata::md_token::*, type_sig::TypeSig};
use crate::interpreter::metadata::table_stream::{MDColumnType, MDTableType};
use super::metadata::Metadata;

bitflags! {
    pub struct CallingConvention: u8 {
        const DEFAULT = 0x0;
		const C = 0x1;
		const STD_CALL = 0x2;
		const THIS_CALL = 0x3;
		const FAST_CALL = 0x4;
		const VAR_ARG = 0x5;
		const FIELD = 0x6;
		const LOCAL_SIG = 0x7;
		const PROPERTY = 0x8;
		/// Unmanaged calling convention encoded as modopts
		const UNMANAGED = 0x9;
		/// generic method instantiation
		const GENERIC_INST = 0xA;
		/// used ONLY for 64bit vararg PInvoke calls
		const NATIVE_VAR_ARG = 0xB;
		/// Calling convention is bottom 4 bits
		const MASK = 0x0F;
		/// Generic method
		const GENERIC = 0x10;
		/// Method needs a 'this' parameter
		const HAS_THIS = 0x20;
		/// 'this' parameter is the first arg if set (else it's hidden)
		const EXPLICIT_THIS = 0x40;
		/// Used internally by the CLR
		const RESERVED_BY_CLR = 0x80;
    }
}

#[derive(PartialEq, Eq)]
pub enum CallingConventionSig {
    FieldSig(FieldSig),
    MethodSig(MethodSig),
    PropertySig(MethodSig),
    LocalSig(LocalSig),
}

impl CallingConventionSig {
    pub fn to_method_sig(&self) -> &MethodSig {
        match self {
            CallingConventionSig::MethodSig(sig) => sig,
            _ => panic!("CallingConventionSig is not a MethodSig"),
        }
    }

    pub fn read_metadata_sig(metadata: &Metadata, signature: u32) -> Option<CallingConventionSig> {
        let data = metadata.blob_stream.read(signature).unwrap();
        let reader = DataReader::new(data);
        let mut offset = 0;
        Self::read_sig(&reader, &mut offset)
    }

    pub fn read_sig(reader: &DataReader, offset: &mut usize) -> Option<CallingConventionSig> {
        let calling_convention = CallingConvention::from_bits_truncate(reader.read_u8_immut(offset).unwrap());
        match calling_convention.intersection(CallingConvention::MASK) {
            CallingConvention::DEFAULT | 
            CallingConvention::C |
            CallingConvention::STD_CALL |
            CallingConvention::THIS_CALL |
            CallingConvention::FAST_CALL |
            CallingConvention::VAR_ARG |
            CallingConvention::NATIVE_VAR_ARG |
            CallingConvention::UNMANAGED => {
                Self::read_method(calling_convention, &reader, offset)
            },
            CallingConvention::FIELD => {
                Self::read_field(calling_convention, &reader, offset)
            },
            CallingConvention::PROPERTY => {
                Self::read_property(calling_convention, &reader, offset)
            },
            CallingConvention::LOCAL_SIG => {
                todo!();
                // Self::read_local_sig(calling_convention, &reader)
            },
            _=> None
        }
    }

    fn read_method(calling_convention: CallingConvention, reader: &DataReader, offset: &mut usize) -> Option<CallingConventionSig> {
        Self::read_sig_internal(MethodSig::new(calling_convention), reader, offset)
    }

    fn read_field(calling_convention: CallingConvention, reader: &DataReader, offset: &mut usize) -> Option<CallingConventionSig> {
        Some(CallingConventionSig::FieldSig(FieldSig::new(calling_convention, TypeSig::read_type(reader, offset))))
    }

    fn read_property(calling_convention: CallingConvention, reader: &DataReader, offset: &mut usize) -> Option<CallingConventionSig> {
        match Self::read_sig_internal(MethodSig::new(calling_convention), reader, offset) {
            Some(CallingConventionSig::MethodSig(method_sig)) => Some(CallingConventionSig::PropertySig(method_sig)),
            _ => None
        }
    }

    fn read_sig_internal(method_sig: MethodSig, reader: &DataReader, offset: &mut usize) -> Option<CallingConventionSig> {
        let mut method_sig = method_sig;
        if method_sig.base.base.get_generic() {
            let count =  reader.try_read_compressed_u32_immut(offset)?;
            method_sig.base.gen_param_count = count;
        }
        let param_count = reader.try_read_compressed_u32_immut(offset)?;
        method_sig.base.ret_type = TypeSig::read_type(reader, offset);
        for _ in 0..param_count {
            match TypeSig::read_type(reader, offset) {
                Some(TypeSig::SentinelSig(_)) => {
                    if !method_sig.base.is_sentinel {
                        method_sig.base.is_sentinel = true;
                    }
                },
                Some(type_sig) => {
                    method_sig.base.parameters.push(type_sig);
                },
                _ => return None,
            }
        }
        Some(CallingConventionSig::MethodSig(method_sig))
    }
}

#[derive(PartialEq, Eq)]
pub struct CallingConventionSigBase {
    pub calling_convention: CallingConvention,
    pub extra_data: Vec<u8>,
}

impl Default for CallingConventionSigBase {
    fn default() -> CallingConventionSigBase {
        CallingConventionSigBase {
            calling_convention: CallingConvention::DEFAULT,
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
        self.calling_convention.intersection(CallingConvention::MASK) == CallingConvention::DEFAULT
    }

    pub fn get_generic(&self) -> bool {
        self.calling_convention.contains(CallingConvention::GENERIC)
    }

    pub fn set_generic(&mut self, value: bool) {
        if value {
            self.calling_convention.insert(CallingConvention::GENERIC);
        } else {
            self.calling_convention.remove(CallingConvention::GENERIC);
        }
    }

    pub fn get_has_this(&self) -> bool {
        self.calling_convention.contains(CallingConvention::HAS_THIS)
    }

    pub fn set_has_this(&mut self, value: bool) {
        if value {
            self.calling_convention.insert(CallingConvention::HAS_THIS);
        } else {
            self.calling_convention.remove(CallingConvention::HAS_THIS);
        }
    }

    pub fn get_explicit_this(&self) -> bool {
        self.calling_convention.contains(CallingConvention::EXPLICIT_THIS)
    }

    pub fn set_explicit_this(&mut self, value: bool) {
        if value {
            self.calling_convention.insert(CallingConvention::EXPLICIT_THIS);
        } else {
            self.calling_convention.remove(CallingConvention::EXPLICIT_THIS);
        }
    }
}

#[derive(PartialEq, Eq)]
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

#[derive(Default, PartialEq, Eq)]
pub struct MethodBaseSig {
    pub base: CallingConventionSigBase,
    pub ret_type: Option<TypeSig>,
    pub parameters: Vec<TypeSig>,
    pub gen_param_count: u32,
    //pub params_after_sentinel: Option<Vec<TypeSig>>,
    pub is_sentinel: bool,
}

impl MethodBaseSig {
    pub fn new(calling_convention: CallingConvention) -> MethodBaseSig {
        MethodBaseSig {
            base: CallingConventionSigBase::new(calling_convention),
            ret_type: None,
            parameters: Vec::new(),
            gen_param_count: 0,
            is_sentinel: false,
        }
    }
}

#[derive(PartialEq, Eq)]
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

    pub fn get_ret_type_string(&self) -> String {
        match &self.base.ret_type {
            None => String::default(),
            Some(type_sig) => format!("{}", type_sig),
        }
    }

    pub fn get_params_type_string(&self) -> String {
        format!("({})", self.base.parameters.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(", "))
    }
}

#[derive(PartialEq, Eq)]
pub struct LocalSig {
    pub base: CallingConventionSigBase,
    pub locals: Vec<TypeSig>,
}