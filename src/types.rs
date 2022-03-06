//! Basic types used in voicemeeter
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

/// A macro button. Zero indexed
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct LogicalButton(pub ZIndex);

impl std::fmt::Display for LogicalButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MB:{}", self.0.0)
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

/// Voicemeeter Parameter
#[aliri_braid::braid()]
pub struct Parameter;

/// Voicemeeter application type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub enum VoicemeeterApplication {
    /// Standard "base" voicemeeter.
    Voicemeeter = 1,
    /// Voicemeeter Banana.
    VoicemeeterBanana = 2,
    /// Voicemeeter Potato.
    VoicemeeterPotato = 3,
    /// Voicemeeter Potato x64.
    PotatoX64Bits = 6,
    /// Unknown voicemeeter type
    Other,
}

impl std::fmt::Display for VoicemeeterApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoicemeeterApplication::Voicemeeter => f.write_str("Voicemeeter"),
            VoicemeeterApplication::VoicemeeterBanana => f.write_str("VoicemeeterBanana"),
            VoicemeeterApplication::VoicemeeterPotato => f.write_str("VoicemeeterPotato"),
            VoicemeeterApplication::PotatoX64Bits => f.write_str("VoicemeeterPotatoX64Bits"),
            VoicemeeterApplication::Other => f.write_str("VoicemeeterUnknown"),
        }
    }
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

/// Level type, used for [`VoicemeeterRemote::get_level`](super::VoicemeeterRemote::get_level)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub enum LevelType {
    /// Pre fader input levels.
    PreFaderInputLevels = 0,
    /// Pre fader output levels.
    PostFaderInputLevels = 1,
    /// Post mute input levels.
    PostMuteInputLevels = 2,
    /// Output levels
    OutputLevels = 3,
    #[doc(hidden)]
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

/// A device.
///
/// Used for callback in [`VoicemeeterRemote::audio_callback_register`](super::VoicemeeterRemote::audio_callback_register) with
/// [`BufferInData::read_write_buffer_on_device`](crate::interface::callback::BufferInData::read_write_buffer_on_device),
/// [`BufferOutData::read_write_buffer_on_device`](crate::interface::callback::BufferOutData::read_write_buffer_on_device) or
/// [`BufferMainData::read_write_buffer_on_device`](crate::interface::callback::BufferMainData::read_write_buffer_on_device)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Device {
    /// Strip 1. Available on all Voicemeeter versions.
    Strip1,
    /// Strip 2. Available on all Voicemeeter versions.
    Strip2,
    /// Strip 3. Available on Voicemeeter Banana and Potato.
    Strip3,
    /// Strip 4. Available on Voicemeeter Banana and Potato.
    Strip4,
    /// Strip 5. Available on Voicemeeter Potato.
    Strip5,
    /// Output A1. Available on all Voicemeeter versions.
    OutputA1,
    /// Output A2. Available on all Voicemeeter versions.
    OutputA2,
    /// Output A3. Available on Voicemeeter Banana and Potato.
    OutputA3,
    /// Output A4. Available on Voicemeeter Potato.
    OutputA4,
    /// Output A5. Available on Voicemeeter Potato.
    OutputA5,
    /// Virtual Output. Available on all Voicemeeter versions.
    VirtualOutput,
    /// Virtual Output B1. Available on all Voicemeeter versions. Alias for [`VirtualOutput`](Self::VirtualOutput)
    VirtualOutputB1,
    /// Virtual Output B2. Available on Voicemeeter Banana and Potato.
    VirtualOutputB2,
    /// Virtual Output B3. Available on Voicemeeter Potato.
    VirtualOutputB3,
    /// Virtual Input. Available on all Voicemeeter versions.
    VirtualInput,
    /// Virtual Input Aux. Available on Voicemeeter Banana and Potato.
    VirtualInputAux,
    /// Virtual Input8. Available on Voicemeeter Potato.
    VirtualInput8,
}

/// Index in the buffers for a [devices'](Device) channel.
#[derive(Debug, Clone, Copy)]
pub struct ChannelIndex {
    /// Start index.
    pub start: usize,
    /// End index.
    pub size: usize,
}

impl ChannelIndex {
    /// Create a new index.
    pub(crate) const fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }
}
const fn ci(start: usize, size: usize) -> Option<ChannelIndex> {
    Some(ChannelIndex::new(start, size))
}
impl Device {
    /// Get the [`ChannelIndex`] for this channel in the buffers when in [main mode](crate::interface::callback::CallbackCommand::BufferMain), if available in the current program.
    pub const fn main(
        &self,
        program: &VoicemeeterApplication,
    ) -> (Option<ChannelIndex>, Option<ChannelIndex>) {
        match program {
            VoicemeeterApplication::Voicemeeter => match self {
                Device::Strip1 => (ci(0, 2), None),
                Device::Strip2 => (ci(2, 2), None),
                Device::OutputA1 => (ci(12, 8), ci(0, 8)),
                Device::OutputA2 => (ci(12, 8), ci(0, 8)),
                Device::VirtualOutput | Device::VirtualOutputB1 => (ci(20, 8), ci(8, 8)),
                Device::VirtualInput => (ci(4, 8), None),
                _ => (None, None),
            },
            VoicemeeterApplication::VoicemeeterBanana => match self {
                Device::Strip1 => (ci(0, 2), None),
                Device::Strip2 => (ci(2, 2), None),
                Device::Strip3 => (ci(4, 2), None),
                Device::OutputA1 => (ci(22, 8), ci(0, 8)),
                Device::OutputA2 => (ci(30, 8), ci(8, 8)),
                Device::OutputA3 => (ci(38, 8), ci(16, 8)),
                Device::VirtualOutput | Device::VirtualOutputB1 => (ci(46, 8), ci(24, 8)),
                Device::VirtualOutputB2 => (ci(54, 8), ci(32, 8)),
                Device::VirtualInput => (ci(6, 8), None),
                Device::VirtualInputAux => (ci(14, 8), None),
                _ => (None, None),
            },
            VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
                match self {
                    Device::Strip1 => (ci(0, 8), None),
                    Device::Strip2 => (ci(2, 8), None),
                    Device::Strip3 => (ci(4, 8), None),
                    Device::Strip4 => (ci(6, 8), None),
                    Device::Strip5 => (ci(8, 8), None),
                    Device::OutputA1 => (ci(34, 8), ci(0, 8)),
                    Device::OutputA2 => (ci(42, 8), ci(8, 8)),
                    Device::OutputA3 => (ci(50, 8), ci(16, 8)),
                    Device::OutputA4 => (ci(58, 8), ci(24, 8)),
                    Device::OutputA5 => (ci(66, 8), ci(32, 8)),
                    Device::VirtualOutput | Device::VirtualOutputB1 => (ci(74, 8), ci(40, 8)),
                    Device::VirtualOutputB2 => (ci(82, 8), ci(48, 8)),
                    Device::VirtualOutputB3 => (ci(82, 8), ci(56, 8)),
                    Device::VirtualInput => (ci(10, 8), None),
                    Device::VirtualInputAux => (ci(18, 8), None),
                    Device::VirtualInput8 => (ci(26, 8), None),
                }
            }
            _ => (None, None),
        }
    }
    /// Get the [`ChannelIndex`] for this channel in the buffers when in [input mode](crate::interface::callback::CallbackCommand::BufferIn), if available in the current program.
    pub const fn input(&self, program: &VoicemeeterApplication) -> Option<ChannelIndex> {
        self.main(program).0
    }
    /// Get the [`ChannelIndex`] for this channel in the buffers when in [output mode](crate::interface::callback::CallbackCommand::BufferOut), if available in the current program.
    pub const fn output(&self, program: &VoicemeeterApplication) -> Option<ChannelIndex> {
        self.main(program).1
    }
    /// Get all channels available in Voicemeeter Potato.
    pub fn potato_channels() -> Vec<Device> {
        vec![
            Device::Strip1,
            Device::Strip2,
            Device::Strip3,
            Device::Strip4,
            Device::Strip5,
            Device::OutputA1,
            Device::OutputA2,
            Device::OutputA3,
            Device::OutputA4,
            Device::OutputA5,
            Device::VirtualOutput,
            Device::VirtualOutputB2,
            Device::VirtualOutputB3,
            Device::VirtualInput,
            Device::VirtualInputAux,
            Device::VirtualInput8,
        ]
    }
}
