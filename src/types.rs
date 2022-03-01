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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Channel {
    Strip1,
    Strip2,
    Strip3,
    Strip4,
    Strip5,
    OutputA1,
    OutputA2,
    OutputA3,
    OutputA4,
    OutputA5,
    VirtualOutput,
    VirtualOutputB1,
    VirtualOutputB2,
    VirtualOutputB3,
    VirtualInput,
    VirtualInputAux,
    VirtualInput8,
}

#[derive(Debug, Clone, Copy)]
pub struct ChannelIndex {
    pub start: usize,
    pub size: usize,
}

impl ChannelIndex {
    pub const fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }
    pub const fn offset(&self) -> usize {
        self.start + self.size
    }
}
const fn ci(start: usize, size: usize) -> Option<ChannelIndex> {
    Some(ChannelIndex::new(start, size))
}
impl Channel {
    pub const fn main(
        &self,
        program: &VoicemeeterApplication,
    ) -> (Option<ChannelIndex>, Option<ChannelIndex>) {
        match program {
            VoicemeeterApplication::Voicemeeter => match self {
                Channel::Strip1 => (ci(0, 2), None),
                Channel::Strip2 => (ci(2, 2), None),
                Channel::OutputA1 => (ci(12, 8), ci(0, 8)),
                Channel::OutputA2 => (ci(12, 8), ci(0, 8)),
                Channel::VirtualOutput | Channel::VirtualOutputB1 => (ci(20, 8), ci(8, 8)),
                Channel::VirtualInput => (ci(4, 8), None),
                _ => (None, None),
            },
            VoicemeeterApplication::VoicemeeterBanana => match self {
                Channel::Strip1 => (ci(0, 2), None),
                Channel::Strip2 => (ci(2, 2), None),
                Channel::Strip3 => (ci(4, 2), None),
                Channel::OutputA1 => (ci(22, 8), ci(0, 8)),
                Channel::OutputA2 => (ci(30, 8), ci(8, 8)),
                Channel::OutputA3 => (ci(38, 8), ci(16, 8)),
                Channel::VirtualOutput | Channel::VirtualOutputB1 => (ci(46, 8), ci(24, 8)),
                Channel::VirtualOutputB2 => (ci(54, 8), ci(32, 8)),
                Channel::VirtualInput => (ci(6, 8), None),
                Channel::VirtualInputAux => (ci(14, 8), None),
                _ => (None, None),
            },
            VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
                match self {
                    Channel::Strip1 => (ci(0, 8), None),
                    Channel::Strip2 => (ci(2, 8), None),
                    Channel::Strip3 => (ci(4, 8), None),
                    Channel::Strip4 => (ci(6, 8), None),
                    Channel::Strip5 => (ci(8, 8), None),
                    Channel::OutputA1 => (ci(34, 8), ci(0, 8)),
                    Channel::OutputA2 => (ci(42, 8), ci(8, 8)),
                    Channel::OutputA3 => (ci(50, 8), ci(16, 8)),
                    Channel::OutputA4 => (ci(58, 8), ci(24, 8)),
                    Channel::OutputA5 => (ci(66, 8), ci(32, 8)),
                    Channel::VirtualOutput | Channel::VirtualOutputB1 => (ci(74, 8), ci(40, 8)),
                    Channel::VirtualOutputB2 => (ci(82, 8), ci(48, 8)),
                    Channel::VirtualOutputB3 => (ci(82, 8), ci(56, 8)),
                    Channel::VirtualInput => (ci(10, 8), None),
                    Channel::VirtualInputAux => (ci(18, 8), None),
                    Channel::VirtualInput8 => (ci(26, 8), None),
                }
            }
            _ => (None, None),
        }
    }
    pub const fn input(&self, program: &VoicemeeterApplication) -> Option<ChannelIndex> {
        self.main(program).0
    }
    pub const fn output(&self, program: &VoicemeeterApplication) -> Option<ChannelIndex> {
        self.main(program).1
    }

    pub fn potato_channels() -> Vec<Channel> {
        vec![
            Channel::Strip1,
            Channel::Strip2,
            Channel::Strip3,
            Channel::Strip4,
            Channel::Strip5,
            Channel::OutputA1,
            Channel::OutputA2,
            Channel::OutputA3,
            Channel::OutputA4,
            Channel::OutputA5,
            Channel::VirtualOutput,
            Channel::VirtualOutputB2,
            Channel::VirtualOutputB3,
            Channel::VirtualInput,
            Channel::VirtualInputAux,
            Channel::VirtualInput8,
        ]
    }
}
