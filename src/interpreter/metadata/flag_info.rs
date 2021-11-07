pub enum CodeType {
    IL,
    Native,
    OPTIL,
    Runtime
}

pub enum Managed {
    Managed,
    Unmanaged
}

pub enum CommonImplAttrFlagInfo {
    NoInlining = 0x0008,
    ForwardRef = 0x0010,
    Synchronized = 0x0020,
    NoOptimization = 0x0040,
    PreserveSig = 0x0080,
    AggressiveInlining = 0x0100,
    AggressiveOptimization = 0x0200,
    SecurityMitigations = 0x0400,
    InternalCall = 0x1000
}

pub enum ImplAttrFlagInfo {
    CodeType(CodeType),
    Managed(Managed),
    CommonImplAttrFlagInfo(CommonImplAttrFlagInfo)
}