//! Basic types used in voicemeeter

/// A Zero Indexed Index
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct ZIndex(pub(crate) i32);

impl std::fmt::Display for ZIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
impl From<usize> for ZIndex {
    fn from(i: usize) -> Self {
        ZIndex(i as i32)
    }
}

impl From<i32> for ZIndex {
    fn from(i: i32) -> Self {
        ZIndex(i)
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
        LogicalButton(ZIndex(i))
    }
}

impl From<u32> for LogicalButton {
    fn from(i: u32) -> Self {
        LogicalButton(ZIndex(i as i32))
    }
}

/// Voicemeeter Parameter
#[aliri_braid::braid()]
pub struct ParameterName;

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
    /// No voicmeeter running
    None = 255,
}

impl VoicemeeterApplication {
    /// Return all possible devices for this application
    pub const fn devices(&self) -> &'static [Device] {
        use self::Device::*;
        match self {
            VoicemeeterApplication::Voicemeeter => {
                &[Strip1, Strip2, VirtualInput, OutputA1, VirtualOutputB1]
            }
            VoicemeeterApplication::VoicemeeterBanana => &[
                Strip1,
                Strip2,
                Strip3,
                VirtualInput,
                VirtualInputAux,
                OutputA1,
                OutputA2,
                OutputA3,
                VirtualOutputB1,
                VirtualOutputB2,
            ],
            VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
                Device::all()
            }
            VoicemeeterApplication::Other | VoicemeeterApplication::None => &[],
        }
    }
}

impl std::fmt::Display for VoicemeeterApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoicemeeterApplication::Voicemeeter => f.write_str("Voicemeeter"),
            VoicemeeterApplication::VoicemeeterBanana => f.write_str("VoicemeeterBanana"),
            VoicemeeterApplication::VoicemeeterPotato => f.write_str("VoicemeeterPotato"),
            VoicemeeterApplication::PotatoX64Bits => f.write_str("VoicemeeterPotatoX64Bits"),
            VoicemeeterApplication::Other => f.write_str("VoicemeeterUnknown"),
            VoicemeeterApplication::None => f.write_str("None"),
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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Device {
    /// Input Strip 1. Available on all Voicemeeter versions.
    Strip1,
    /// Input Strip 2. Available on all Voicemeeter versions.
    Strip2,
    /// Input Strip 3. Available on Voicemeeter Banana and Potato.
    Strip3,
    /// Input Strip 4. Available on Voicemeeter Potato.
    Strip4,
    /// Input Strip 5. Available on Voicemeeter Potato.
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
    /// Virtual Output B1. Available on all Voicemeeter versions.
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

    /// Get the channel index for
    pub fn get(&self, channel: usize) -> Option<usize> {
        (self.start..(self.start + self.size)).nth(channel)
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
                //Device::OutputA2 => (ci(12, 8), ci(0, 8)),
                Device::VirtualOutputB1 => (ci(20, 8), ci(8, 8)),
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
                Device::VirtualOutputB1 => (ci(46, 8), ci(24, 8)),
                Device::VirtualOutputB2 => (ci(54, 8), ci(32, 8)),
                Device::VirtualInput => (ci(6, 8), None),
                Device::VirtualInputAux => (ci(14, 8), None),
                _ => (None, None),
            },
            VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
                match self {
                    Device::Strip1 => (ci(0, 2), None),
                    Device::Strip2 => (ci(2, 2), None),
                    Device::Strip3 => (ci(4, 2), None),
                    Device::Strip4 => (ci(6, 2), None),
                    Device::Strip5 => (ci(8, 2), None),
                    Device::OutputA1 => (ci(34, 8), ci(0, 8)),
                    Device::OutputA2 => (ci(42, 8), ci(8, 8)),
                    Device::OutputA3 => (ci(50, 8), ci(16, 8)),
                    Device::OutputA4 => (ci(58, 8), ci(24, 8)),
                    Device::OutputA5 => (ci(66, 8), ci(32, 8)),
                    Device::VirtualOutputB1 => (ci(74, 8), ci(40, 8)),
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
        match self {
            Device::OutputA1
            | Device::OutputA2
            | Device::OutputA3
            | Device::OutputA4
            | Device::OutputA5
            | Device::VirtualOutputB1
            | Device::VirtualOutputB2
            | Device::VirtualOutputB3 => None,
            _ => self.main(program).0,
        }
    }
    /// Get the [`ChannelIndex`] for this channel in the buffers when in [output mode](crate::interface::callback::CallbackCommand::BufferOut), if available in the current program.
    pub const fn output(&self, program: &VoicemeeterApplication) -> Option<ChannelIndex> {
        self.main(program).1
    }
    /// Get all channels available.
    pub const fn all() -> &'static [Self] {
        &[
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
            Device::VirtualOutputB1,
            Device::VirtualOutputB2,
            Device::VirtualOutputB3,
            Device::VirtualInput,
            Device::VirtualInputAux,
            Device::VirtualInput8,
        ]
    }

    /// Gives the devices number for the current application for getting level.
    pub(crate) fn as_level_device_num(
        &self,
        program: &VoicemeeterApplication,
        level_type: LevelType,
        channel: usize,
    ) -> Option<usize> {
        match level_type {
            LevelType::PreFaderInputLevels
            | LevelType::PostFaderInputLevels
            | LevelType::PostMuteInputLevels => self.input(program)?.get(channel),
            LevelType::OutputLevels => self.output(program)?.get(channel),
            LevelType::Other => None,
        }
    }

    /// Get the strip index for this device in the current program.
    pub const fn as_strip_index(&self, program: &VoicemeeterApplication) -> Option<ZIndex> {
        let i = match program {
            VoicemeeterApplication::Voicemeeter => Some(match self {
                Device::Strip1 => 0,
                Device::Strip2 => 1,
                Device::VirtualInput => 2,
                _ => return None,
            }),
            VoicemeeterApplication::VoicemeeterBanana => Some(match self {
                Device::Strip1 => 0,
                Device::Strip2 => 1,
                Device::Strip3 => 2,
                Device::VirtualInput => 3,
                Device::VirtualInputAux => 4,
                _ => return None,
            }),
            VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
                Some(match self {
                    Device::Strip1 => 0,
                    Device::Strip2 => 1,
                    Device::Strip3 => 2,
                    Device::Strip4 => 3,
                    Device::Strip5 => 4,
                    Device::VirtualInput => 5,
                    Device::VirtualInputAux => 6,
                    Device::VirtualInput8 => 7,
                    _ => return None,
                })
            }
            _ => return None,
        };
        match i {
            Some(i) => Some(ZIndex(i)),
            None => None,
        }
    }

    /// Get the bus index for this device in the current program.
    pub const fn as_bus_index(&self, program: &VoicemeeterApplication) -> Option<ZIndex> {
        let i = match program {
            VoicemeeterApplication::Voicemeeter => Some(match self {
                Device::OutputA1 => 0,
                Device::OutputA2 => 1,
                Device::VirtualOutputB1 => 2,
                _ => return None,
            }),
            VoicemeeterApplication::VoicemeeterBanana => Some(match self {
                Device::OutputA1 => 0,
                Device::OutputA2 => 1,
                Device::OutputA3 => 2,
                Device::VirtualOutputB1 => 3,
                Device::VirtualOutputB2 => 4,
                _ => return None,
            }),
            VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
                Some(match self {
                    Device::OutputA1 => 0,
                    Device::OutputA2 => 1,
                    Device::OutputA3 => 2,
                    Device::OutputA4 => 3,
                    Device::OutputA5 => 4,
                    Device::VirtualOutputB1 => 5,
                    Device::VirtualOutputB2 => 6,
                    Device::VirtualOutputB3 => 7,
                    _ => return None,
                })
            }
            _ => return None,
        };
        match i {
            Some(i) => Some(ZIndex(i)),
            None => None,
        }
    }
}

/// Bus mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusMode {
    /// Bus mode normal
    Normal,
    /// Bus mode Amix
    Amix,
    /// Bus mode Bmix
    Bmix,
    /// Bus mode Repeat
    Repeat,
    /// Bus mode Composite
    Composite,
    /// Bus mode TVMix
    TvMix,
    /// Bus mode UpMix21
    UpMix21,
    /// Bus mode UpMix41
    UpMix41,
    /// Bus mode UpMix61
    UpMix61,
    /// Bus mode CenterOnly
    CenterOnly,
    /// Bus mode LFEOnly
    LfeOnly,
    /// Bus mode RearOnly
    RearOnly,
}

impl std::fmt::Display for BusMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusMode::Normal => f.write_str("normal"),
            BusMode::Amix => f.write_str("Amix"),
            BusMode::Bmix => f.write_str("Bmix"),
            BusMode::Repeat => f.write_str("Repeat"),
            BusMode::Composite => f.write_str("Composite"),
            BusMode::TvMix => f.write_str("TVMix"),
            BusMode::UpMix21 => f.write_str("UpMix21"),
            BusMode::UpMix41 => f.write_str("UpMix41"),
            BusMode::UpMix61 => f.write_str("UpMix61"),
            BusMode::CenterOnly => f.write_str("CenterOnly"),
            BusMode::LfeOnly => f.write_str("LFEOnly"),
            BusMode::RearOnly => f.write_str("RearOnly"),
        }
    }
}
