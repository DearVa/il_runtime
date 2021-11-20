use std::{cmp::Ordering, ops::{Add, BitAnd, BitOr, BitXor, Sub}};

use crate::interpreter::type_sig::{CorLibType, TypeSig};

use super::calling_convention_sig::CallingConventionSig;

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

    Isize(isize),
    Usize(usize),
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
            ILValType::Isize(i) => i == 0,
            ILValType::Usize(i) => i == 0,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match *self {
            ILValType::Boolean(b) => b as u32,
            ILValType::Byte(b) => b as u32,
            ILValType::SByte(b) => b as u32,
            ILValType::Char(c) => c as u32,
            ILValType::Double(d) => d as u32,
            ILValType::Single(f) => f as u32,
            ILValType::Int32(i) => i as u32,
            ILValType::UInt32(i) => i,
            ILValType::Int64(i) => i as u32,
            ILValType::UInt64(i) => i as u32,
            ILValType::Short(i) => i as u32,
            ILValType::UShort(i) => i as u32,
            ILValType::Isize(i) => i as u32,
            ILValType::Usize(i) => i as u32,
        }
    }

    pub fn to_usize(&self) -> usize {
        match *self {
            ILValType::Boolean(b) => b as usize,
            ILValType::Byte(b) => b as usize,
            ILValType::SByte(b) => b as usize,
            ILValType::Char(c) => c as usize,
            ILValType::Double(d) => d as usize,
            ILValType::Single(f) => f as usize,
            ILValType::Int32(i) => i as usize,
            ILValType::UInt32(i) => i as usize,
            ILValType::Int64(i) => i as usize,
            ILValType::UInt64(i) => i as usize,
            ILValType::Short(i) => i as usize,
            ILValType::UShort(i) => i as usize,
            ILValType::Isize(i) => i as usize,
            ILValType::Usize(i) => i,
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
            ILValType::Isize(i) => i.to_string(),
            ILValType::Usize(i) => i.to_string(),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ILRefType {
    Null,
    String(usize),  // 指向Strings堆
    Object(usize),  // 指向Objects堆
}

/// 表示一个托管的Ptr，可能指向Param，Local或者Static
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ILPtr {
    /// (栈ID, index)
    Param((usize, usize)),
    /// (栈ID, index)
    Local((usize, usize)),
    /// (Assembly_index, token)
    Static((usize, u32)),
}

/// 表示一个Native Ptr，但其实不是真的指针，使用安全的方式封装
#[derive(Debug, Clone, PartialEq)]
pub struct ILNPtr {
    data: Option<Box<Vec<u8>>>,
    offset: usize,
}

impl ILNPtr {
    pub fn new(size: usize) -> ILNPtr {
        ILNPtr {
            data: Some(Box::new(vec![0; size])),
            offset: 0,
        }
    }

    pub fn offset(&mut self, off: isize) {
        if off.is_negative() {
            self.offset.checked_sub(off.wrapping_abs() as usize);
        } else {
            self.offset.checked_add(off as usize);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ILType {
    Val(ILValType),
    Ref(ILRefType),
    Ptr(ILPtr),
    NPtr(ILNPtr),
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
                    _ => panic!("Null reference exception"),
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
            ILType::NPtr(p) => {
                p.data.is_none()
            },
            ILType::Ptr(_) => false,
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
            (ILType::NPtr(mut p1), ILType::Val(ILValType::Int32(i2))) => {
                p1.offset(i2 as isize);
                ILType::NPtr(p1)
            }
            (ILType::NPtr(mut p1), ILType::Val(ILValType::Int64(i2))) => {
                p1.offset(i2 as isize);
                ILType::NPtr(p1)
            }
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
