use std::{cmp::Ordering, ops::{Add, BitAnd, BitOr, BitXor, Sub}};

use crate::interpreter::type_sig::{CorLibType, TypeSig};

use super::signature::CallingConventionSig;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ILValType {
    Boolean(bool),
    Byte(u8),
    SByte(i8),
    Char(char),
    Double(f64),
    Single(f32),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Short(i16),
    UShort(u16),
}

impl ILValType {
    pub fn is_false_type(&self) -> bool {
        match *self {
            ILValType::Boolean(b) => b == false,
            ILValType::Byte(b) => b == 0,
            ILValType::SByte(b) => b == 0,
            ILValType::Char(c) => c == '\0',
            ILValType::Double(d) => d == 0.0,
            ILValType::Single(f) => f == 0.0,
            ILValType::Int32(i) => i == 0,
            ILValType::UInt32(i) => i == 0,
            ILValType::Int64(i) => i == 0,
            ILValType::UInt64(i) => i == 0,
            ILValType::Short(i) => i == 0,
            ILValType::UShort(i) => i == 0,
        }
    }
}

impl ToString for ILValType {
    fn to_string(&self) -> String {
        match self {
            ILValType::Boolean(b) => b.to_string(),
            ILValType::Byte(b) => b.to_string(),
            ILValType::SByte(b) => b.to_string(),
            ILValType::Char(c) => c.to_string(),
            ILValType::Double(d) => d.to_string(),
            ILValType::Single(f) => f.to_string(),
            ILValType::Int32(i) => i.to_string(),
            ILValType::UInt32(i) => i.to_string(),
            ILValType::Int64(i) => i.to_string(),
            ILValType::UInt64(i) => i.to_string(),
            ILValType::Short(i) => i.to_string(),
            ILValType::UShort(i) => i.to_string(),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ILRefType {
    Null,
    String(usize),  // 指向Strings堆
    Object(usize),  // 指向Objects堆
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ILType {
    Val(ILValType),
    Ref(ILRefType),
    Ptr(*mut ILType),
}

impl ILType {
    pub fn is_ref(self) -> bool {
        match self {
            ILType::Ref(_) => true,
            _ => false,
        }
    }

    pub fn get_ref(&self) -> usize {
        match self {
            ILType::Ref(ref r) => {
                match r {
                    ILRefType::Object(o) => *o,
                    ILRefType::String(s) => *s,
                    _ => panic!("not an object"),
                }
            },
            _ => panic!("not a ref type"),
        }
    }

    pub fn is_false_type(&self) -> bool {
        match self {
            ILType::Val(val) => val.is_false_type(),
            ILType::Ref(r) => {
                match r {
                    ILRefType::Null => true,
                    _ => false,
                }
            },
            ILType::Ptr(p) => {
                p.is_null()
            },
        }
    }

    pub fn from_type_sig(sig: &TypeSig) -> ILType {
        match sig {
            TypeSig::CorLibTypeSig(c) => {
                match c {
                    CorLibType::Void => panic!("field cannot be void"),
                    CorLibType::Boolean => ILType::Val(ILValType::Boolean(false)),
                    CorLibType::Byte => ILType::Val(ILValType::Byte(0)),
                    CorLibType::SByte => ILType::Val(ILValType::SByte(0)),
                    CorLibType::Char => ILType::Val(ILValType::Char('\0')),
                    CorLibType::Double => ILType::Val(ILValType::Double(0.0)),
                    CorLibType::Single => ILType::Val(ILValType::Single(0.0)),
                    CorLibType::Int16 => ILType::Val(ILValType::Short(0)),
                    CorLibType::UInt16 => ILType::Val(ILValType::UShort(0)),
                    CorLibType::Int32 => ILType::Val(ILValType::Int32(0)),
                    CorLibType::UInt32 => ILType::Val(ILValType::UInt32(0)),
                    CorLibType::Int64 => ILType::Val(ILValType::Int64(0)),
                    CorLibType::UInt64 => ILType::Val(ILValType::UInt64(0)),
                    _ => ILType::Ref(ILRefType::Null),
                }
            },
            _ => ILType::Ref(ILRefType::Null),
        }
    }

    pub fn from_type_sigs(sigs: Vec<&TypeSig>) -> Vec<ILType> {
        sigs.iter().map(|s| ILType::from_type_sig(s)).collect()
    }

    /// 从CallingConventionSig转换
    pub fn from_signature(sig: &CallingConventionSig) -> ILType {
        match sig {
            CallingConventionSig::FieldSig(f) => {
                if let Some(sig) = &f.type_sig {
                    Self::from_type_sig(&sig)
                } else {
                    ILType::Ref(ILRefType::Null)
                }
            },
            _ => panic!("not a field sig"),
        }
    }

    pub fn from_signatures(sigs: Vec<&CallingConventionSig>) -> Vec<ILType> {
        sigs.into_iter().map(ILType::from_signature).collect()
    }
}

impl Add for ILType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ILType::Val(v1), ILType::Val(v2)) => {
                match (v1, v2) {
                    (ILValType::Int32(i1), ILValType::Int32(i2)) => ILType::Val(ILValType::Int32(i1 + i2)),
                    (ILValType::Int32(i1), ILValType::Int64(i2)) => ILType::Val(ILValType::Int64(i1 as i64 + i2)),
                    (ILValType::Int64(i1), ILValType::Int32(i2)) => ILType::Val(ILValType::Int64(i1 + i2 as i64)),
                    (ILValType::Int64(i1), ILValType::Int64(i2)) => ILType::Val(ILValType::Int64(i1 + i2)),
                    (ILValType::Single(i1), ILValType::Single(i2)) => ILType::Val(ILValType::Single(i1 + i2)),
                    (ILValType::Double(i1), ILValType::Double(i2)) => ILType::Val(ILValType::Double(i1 + i2)),
                    _ => panic!("Invalid Operation")
                }
            },
            (ILType::Ptr(p1), ILType::Val(ILValType::Int32(i2))) => ILType::Ptr(unsafe { p1.offset(i2 as isize) }),
            (ILType::Ptr(p1), ILType::Val(ILValType::Int64(i2))) => ILType::Ptr(unsafe { p1.offset(i2 as isize) }),
            (ILType::Val(ILValType::Int32(i1)), ILType::Ptr(p2)) => ILType::Ptr(unsafe { p2.offset(i1 as isize) }),
            (ILType::Val(ILValType::Int64(i1)), ILType::Ptr(p2)) => ILType::Ptr(unsafe { p2.offset(i1 as isize) }),
            _ => panic!("Invalid Operation")
        }
    }
}

impl Sub for ILType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (ILType::Val(v1), ILType::Val(v2)) => {
                match (v1, v2) {
                    (ILValType::Int32(i1), ILValType::Int32(i2)) => ILType::Val(ILValType::Int32(i1 - i2)),
                    (ILValType::Int32(i1), ILValType::Int64(i2)) => ILType::Val(ILValType::Int64(i1 as i64 - i2)),
                    (ILValType::Int64(i1), ILValType::Int32(i2)) => ILType::Val(ILValType::Int64(i1 - i2 as i64)),
                    (ILValType::Int64(i1), ILValType::Int64(i2)) => ILType::Val(ILValType::Int64(i1 - i2)),
                    (ILValType::Single(i1), ILValType::Single(i2)) => ILType::Val(ILValType::Single(i1 - i2)),
                    (ILValType::Double(i1), ILValType::Double(i2)) => ILType::Val(ILValType::Double(i1 - i2)),
                    _ => panic!("Invalid Operation")
                }
            }
            _ => panic!("Invalid Operation")
        }
    }
}

impl PartialOrd for ILType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ILType::Val(v1), ILType::Val(v2)) => {
                match (v1, v2) {
                    (ILValType::Int32(i1), ILValType::Int32(i2)) => i1.partial_cmp(i2),
                    (ILValType::Int64(i1), ILValType::Int64(i2)) => i1.partial_cmp(i2),
                    (ILValType::Single(i1), ILValType::Single(i2)) => i1.partial_cmp(i2),
                    (ILValType::Double(i1), ILValType::Double(i2)) => i1.partial_cmp(i2),
                    _ => panic!("Invalid Operation")
                }
            }
            _ => panic!("Invalid Operation")
        }
    }
}

impl BitAnd for ILType {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (ILType::Val(v1), ILType::Val(v2)) => {
                match (v1, v2) {
                    (ILValType::Int32(i1), ILValType::Int32(i2)) => ILType::Val(ILValType::Int32(i1 & i2)),
                    (ILValType::Int64(i1), ILValType::Int64(i2)) => ILType::Val(ILValType::Int64(i1 & i2)),
                    _ => panic!("Invalid Operation")
                }
            }
            _ => panic!("Invalid Operation")
        }
    }
}

impl BitOr for ILType {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (ILType::Val(v1), ILType::Val(v2)) => {
                match (v1, v2) {
                    (ILValType::Int32(i1), ILValType::Int32(i2)) => ILType::Val(ILValType::Int32(i1 | i2)),
                    (ILValType::Int64(i1), ILValType::Int64(i2)) => ILType::Val(ILValType::Int64(i1 | i2)),
                    _ => panic!("Invalid Operation")
                }
            }
            _ => panic!("Invalid Operation")
        }
    }
}

impl BitXor for ILType {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        match (self, other) {
            (ILType::Val(v1), ILType::Val(v2)) => {
                match (v1, v2) {
                    (ILValType::Int32(i1), ILValType::Int32(i2)) => ILType::Val(ILValType::Int32(i1 ^ i2)),
                    (ILValType::Int64(i1), ILValType::Int64(i2)) => ILType::Val(ILValType::Int64(i1 ^ i2)),
                    _ => panic!("Invalid Operation")
                }
            }
            _ => panic!("Invalid Operation")
        }
    }
}
