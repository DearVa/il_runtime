use std::{hash::{Hash, Hasher}, ptr};

use crate::hash_vec::HashVec;

use super::{Interpreter, il_type::ILType};

pub struct Object {
    /// 包括locked、pinned、gc_mark和代
    flags: u8,
    /// 对象原始的type_token，不可改变
    origin_type_token: u32,
    /// 对象的type_token，可能经过castclass发生了改变
    pub type_token: u32,
    field_map: HashVec<u32, ILType>,
    /// 如果是box，那么这个存储原始数据
    pub box_value: Option<ILType>,
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::addr_of!(self).hash(state);
    }
}

impl Object {
    pub fn new(type_token: u32, field_map: HashVec<u32, ILType>) -> Object {
        Object {
            flags: 0,
            origin_type_token: type_token,
            type_token,
            field_map,
            box_value: None,
        }
    }

    pub fn new_box(type_token: u32, value: ILType) -> Object {
        Object {
            flags: 0,
            origin_type_token: type_token,
            type_token,
            field_map: HashVec::new(),
            box_value: Some(value),
        }
    }

    pub fn get_field(&self, field_token_or_rid: u32) -> Option<&ILType> {
        self.field_map.key_get(&(field_token_or_rid & 0x00FFFFFF))
    }

    pub fn set_field(&mut self, field_token_or_rid: u32, value: ILType) {
        let field = self.field_map.key_get_mut(&(field_token_or_rid & 0x00FFFFFF)).unwrap();
        *field = value;
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
        self.origin_type_token
    }

    pub fn to_string(&self, interpreter: &Interpreter) -> String {
        match self.box_value {
            Some(ref il_type) => format!("{}", interpreter.format_il_type(il_type)),
            None => format!("Object: type_token: {}", self.get_type()),
        }
    }
}