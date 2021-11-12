use bitflags::bitflags;

bitflags! {
    pub struct AssemblyNameFlags: u32 {
        const NONE = 0x0;  // 0v0
        const PUBLIC_KEY = 0x1;
        const RETARGETABLE = 0x100;
        const ENABLE_JIT_COMPILE_OPTIMIZER = 0x4000;
        const ENABLE_JIT_COMPILE_TRACKING = 0x8000;
    }
}

pub struct AssemblyName {
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,
    pub flags: u32,
    pub public_key_token: Vec<u8>,
    pub name: String,
}

impl AssemblyName {
    fn all_zero(vec: &Vec<u8>) -> bool {
        for i in vec.iter() {
            if *i != 0 {
                return false;
            }
        }
        true
    }
}

impl Clone for AssemblyName {
    fn clone(&self) -> Self {
        AssemblyName {
            major_version: self.major_version,
            minor_version: self.minor_version,
            build_number: self.build_number,
            revision_number: self.revision_number,
            flags: self.flags,
            public_key_token: self.public_key_token.clone(),
            name: self.name.clone(),
        }
    }
}

impl PartialEq for AssemblyName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        // if self.name != other.name {
        //     false
        // } else if AssemblyName::all_zero(&self.public_key_token) || AssemblyName::all_zero(&other.public_key_token) {
        //     true
        // } else {
        //     self.public_key_token == other.public_key_token
        // }
    }
}
