/// A Zero Indexed Index
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ZIndex(pub(crate) i32);

impl From<usize> for ZIndex {
    fn from(i: usize) -> Self {
        ZIndex(i as i32)
    }
}

impl From<i32> for ZIndex {
    fn from(i: i32) -> Self {
        ZIndex(i as i32)
    }
}

impl From<u32> for ZIndex {
    fn from(i: u32) -> Self {
        ZIndex(i as i32)
    }
}

#[aliri_braid::braid()]
pub struct Parameter;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub enum VoicemeeterApplication {
    Voicemeeter = 1,
    VoicemeeterBanana = 2,
    VoicemeeterPotato = 3,
    PotatoX64Bits = 6,
    Other,
}

impl From<i32> for VoicemeeterApplication {
    fn from(ty: i32) -> Self {
        match ty {
            1 => VoicemeeterApplication::Voicemeeter,
            2 => VoicemeeterApplication::VoicemeeterBanana,
            3 => VoicemeeterApplication::VoicemeeterPotato,
            6 => VoicemeeterApplication::PotatoX64Bits,
            _ => VoicemeeterApplication::Other,
        }
    }
}

#[doc(inline)]
pub use crate::bindings::VbvmrDevType;
