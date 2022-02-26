use crate::bindings;

/// A Zero Indexed Index
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct LogicalButton(pub ZIndex);

impl std::fmt::Display for LogicalButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MB:{}", self.0 .0)
    }
}

impl From<usize> for LogicalButton {
    fn from(i: usize) -> Self {
        LogicalButton(ZIndex(i as i32))
    }
}

impl From<i32> for LogicalButton {
    fn from(i: i32) -> Self {
        LogicalButton(ZIndex(i as i32))
    }
}

impl From<u32> for LogicalButton {
    fn from(i: u32) -> Self {
        LogicalButton(ZIndex(i as i32))
    }
}

#[aliri_braid::braid()]
pub struct Parameter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub enum LevelType {
    PreFaderInputLevels = 0,
    PostFaderInputLevels = 1,
    PostMuteInputLevels = 2,
    OutputLevels = 3,
    Other,
}

impl From<i32> for LevelType {
    fn from(ty: i32) -> Self {
        match ty {
            0 => LevelType::PreFaderInputLevels,
            1 => LevelType::PostFaderInputLevels,
            2 => LevelType::PostMuteInputLevels,
            3 => LevelType::OutputLevels,
            _ => LevelType::Other,
        }
    }
}

#[repr(i32)]
pub enum CallbackCommand {
    Starting,
    Ending,
    Change,
    BufferIn,
    BufferOut,
    BufferMain,
    Other(bindings::VBVMR_CBCOMMAND),
}

impl From<bindings::VBVMR_CBCOMMAND> for CallbackCommand {
    fn from(n: bindings::VBVMR_CBCOMMAND) -> Self {
        match n {
            bindings::VBVMR_CBCOMMAND::STARTING => Self::Starting,
            bindings::VBVMR_CBCOMMAND::ENDING => Self::Ending,
            bindings::VBVMR_CBCOMMAND::CHANGE => Self::Change,
            bindings::VBVMR_CBCOMMAND::BUFFER_IN => Self::BufferIn,
            bindings::VBVMR_CBCOMMAND::BUFFER_OUT => Self::BufferOut,
            bindings::VBVMR_CBCOMMAND::BUFFER_MAIN => Self::BufferMain,
            i => Self::Other(i),
        }
    }
}
