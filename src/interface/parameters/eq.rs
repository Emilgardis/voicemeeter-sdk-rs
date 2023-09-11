//! Common structs for EQ
use super::*;

enum Mode {
    Strip,
    Bus,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Strip => f.write_str(STRIP),
            Mode::Bus => f.write_str(BUS),
        }
    }
}

/// Parameter for EQ on a specific channel and input/output (bus/strip)
pub struct EqChannelParameter<'a> {
    remote: &'a VoicemeeterRemote,
    mode: Mode,
    index: ZIndex,
    channel: usize,
}

impl<'a> EqChannelParameter<'a> {
    pub(crate) fn new_bus(remote: &'a VoicemeeterRemote, index: ZIndex, channel: usize) -> Self {
        Self {
            remote,
            mode: Mode::Bus,
            index,
            channel,
        }
    }

    pub(crate) fn new_strip(remote: &'a VoicemeeterRemote, index: ZIndex, channel: usize) -> Self {
        Self {
            remote,
            mode: Mode::Strip,
            index,
            channel,
        }
    }
    pub(crate) fn name(&self) -> impl std::fmt::Display + '_ {
        struct N<'s>(&'s Mode, &'s ZIndex, &'s usize);
        impl std::fmt::Display for N<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}[{}].EQ.channel[{}]", self.0, self.1, self.2)
            }
        }
        N(&self.mode, &self.index, &self.channel)
    }
    /// Get the identifier for a parameter on this equalizer: `Bus[i].EQ.channel[ch].cell[c].{dot}`
    pub fn param(&self, cell: usize, dot: impl ToString) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(format!("{}.cell[{}].{}", self.name(), cell, dot.to_string()).into())
    }
    /// Turn EQ cell on or off
    pub fn on(&self, cell: usize) -> BoolParameter {
        BoolParameter::new(self.param(cell, "on"), self.remote)
    }
    /// Type of EQ filter.
    pub fn type_(&self, cell: usize) -> IntParameter {
        // TODO: Enum Parameter
        IntParameter::new(self.param(cell, "type"), self.remote, 0..=6)
    }
    /// Frequency of the EQ filter.
    pub fn f(&self, cell: usize) -> FloatParameter {
        // TODO: Enum Parameter
        FloatParameter::new(self.param(cell, "f"), self.remote, 20.0..=20_000.0)
    }
    /// Gain of the EQ filter.
    pub fn gain(&self, cell: usize) -> FloatParameter {
        // TODO: Enum Parameter
        // NOTE: Docs say -12 to 12, but interface allows -36 to 18
        FloatParameter::new(self.param(cell, "gain"), self.remote, -36.0..=18.0)
    }
    /// Quality of the EQ filter.
    pub fn q(&self, cell: usize) -> IntParameter {
        // TODO: Enum Parameter
        IntParameter::new(self.param(cell, "q"), self.remote, 1..=100)
    }
}
