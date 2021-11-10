use super::il_type::ILType;

pub struct Object {
    pub flags: u8,  // 包括locked、pinned、gc_mark和代，第2位0表示TypeRef，1表示TypeDef
    pub hash: u32,
    pub size: u16,
    pub type_token: [u8; 3],
    pub value: Box<ILType>,  // TODO: 暂时只存放i32
}

impl Object {
    pub fn new(type_token: u32, value: ILType) -> Object {
        Object {
            flags: {
                if (type_token >> 24) & 0xF == 2 {
                    1
                } else {
                    0
                }
            },
            hash: 0,
            size: 8,
            type_token: {
                let mut result = [0u8; 3];
                result[2] = (type_token >> 16) as u8;
                result[1] = (type_token >> 8) as u8;
                result[0] = type_token as u8;
                result
            },
            value: Box::new(value),
        }
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