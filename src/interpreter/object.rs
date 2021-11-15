use std::{hash::{Hash, Hasher}, rc::Rc};

use super::{Assembly, il_type::ILType};

pub struct Object {
    /// 包括locked、pinned、gc_mark和代，第2位0表示TypeRef，1表示TypeDef
    pub flags: u8,
    pub hash: u32,
    pub size: u16,
    pub type_token: [u8; 3],
    /// 如果是box，那么这个存储原始数据
    pub box_value: Option<ILType>,
}

impl ToString for Object {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("Object: ");
        s.push_str(&format!("type_token: {}", self.get_type()));
        // s.push_str(&format!("value: {}", self.value));
        s
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl Object {
    pub fn new(type_token: u32) -> Object {
        Object {
            flags: Object::parse_flags(type_token),
            hash: 0,
            size: 8,
            type_token: Object::parse_type_token(type_token),
            box_value: None,
        }
    }

    pub fn new_box(type_token: u32, value: ILType) -> Object {
        Object {
            flags: Object::parse_flags(type_token),
            hash: 0,
            size: 8,
            type_token: Object::parse_type_token(type_token),
            box_value: Some(value),
        }
    }

    fn parse_flags(type_token: u32) -> u8 {
        if (type_token >> 24) & 0xF == 2 {
            1
        } else {
            0
        }
    }

    fn parse_type_token(type_token: u32) -> [u8; 3] {
        let mut result = [0u8; 3];
        result[2] = (type_token >> 16) as u8;
        result[1] = (type_token >> 8) as u8;
        result[0] = type_token as u8;
        result
    }

    pub fn is_locked(&self) -> bool {
        self.flags >> 7 != 0
    }

    pub fn is_pinned(&self) -> bool {
        (self.flags >> 6) & 1 != 0
    }

    pub fn get_generation(&self) -> u8 {
        self.flags >> 4 & 0b11
    }

    pub fn get_gc_mark(&self) -> bool {
        (self.flags >> 3) & 1 != 0
    }

    pub fn set_gc_mark(&mut self, mark: bool) {
        self.flags &= !(1 << 3);
        self.flags |= (mark as u8) << 3;
    }

    pub fn get_type(&self) -> u32 {
        let is_def = self.flags & 1;
        u32::from_le_bytes([self.type_token[0], self.type_token[1], self.type_token[2], (is_def + 1) as u8])
    }
}