use std::ops::{Add, Sub};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ILType {
    Val(ILValType),
    Ref(Option<usize>),  // 指向Objects堆
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
            ILType::Ref(ref v) => *v.as_ref().unwrap(),
            _ => panic!("not ref"),
        }
    }
}

impl ToString for ILType {
    fn to_string(&self) -> String {
        match self {
            ILType::Val(v) => match v {
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
            },
            ILType::Ref(r) => match r {
                Some(i) => format!("ref:{}", i),
                None => "ref:null".to_string(),
            },
        }
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
                    (ILValType::Single(i1), ILValType::Single(i2)) => ILType::Val(ILValType::Single(i1 + i2)),
                    (ILValType::Double(i1), ILValType::Double(i2)) => ILType::Val(ILValType::Double(i1 + i2)),
                    _ => panic!("Invalid Operation")
                }
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
                    (ILValType::Single(i1), ILValType::Single(i2)) => ILType::Val(ILValType::Single(i1 - i2)),
                    (ILValType::Double(i1), ILValType::Double(i2)) => ILType::Val(ILValType::Double(i1 - i2)),
                    _ => panic!("Invalid Operation")
                }
            }
            _ => panic!("Invalid Operation")
        }
    }
}