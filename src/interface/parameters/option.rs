//! Option parameters
use super::*;

/// Option parameters for voicemeeter
pub struct VoicemeeterOption<'a> {
    remote: &'a VoicemeeterRemote,
}

impl<'a> VoicemeeterOption<'a> {
    #[doc(hidden)]
    pub fn new(remote: &'a VoicemeeterRemote) -> Self {
        VoicemeeterOption { remote }
    }

    fn param(&self, dot: impl Display) -> Cow<'static, ParameterNameRef> {
        // TODO: Should this maybe allow custom names?
        Cow::Owned(format!("{VOICEMEETER_OPTION}.{}", dot).into())
    }
    /// Preferred samplerate
    ///
    ///
    /// Valid Samplerates |
    /// ---
    /// `44100.0`
    /// `48000.0`
    /// `88200.0`
    /// `96000.0`
    /// `176400.0`
    /// `192000.0`
    pub fn sr(&self) -> FloatParameter {
        // FIXME: Enum parameter
        FloatParameter::new_unranged(self.param("sr"), self.remote)
    }
    /// Samplerate for ASIO driver on output A1.
    ///  Value | Description
    ///   ---  |   ---
    ///  `false` | default ASIO samplerate
    ///  `true`  | preferred samplerate
    pub fn asio_sr(&self) -> BoolParameter {
        BoolParameter::new(self.param("ASIOsr"), self.remote)
    }
    /// BUS output delay
    pub fn delay(&self, bus: usize) -> IntParameter {
        IntParameter::new(self.param(format!("delay[{bus}]")), self.remote, 0..=500)
    }
    /// MME buffer size
    pub fn buffer_mme(&self) -> IntParameter {
        IntParameter::new(self.param("buffer.mme"), self.remote, 128..=2048)
    }
    /// WDM buffer size
    pub fn buffer_wdm(&self) -> IntParameter {
        IntParameter::new(self.param("buffer.wdm"), self.remote, 128..=2048)
    }
    /// KS buffer size
    pub fn buffer_ks(&self) -> IntParameter {
        IntParameter::new(self.param("buffer.ks"), self.remote, 128..=2048)
    }
    /// ASIO buffer size
    pub fn buffer_asio(&self) -> IntParameter {
        IntParameter::new(self.param("buffer.asio"), self.remote, 128..=2048)
    }
    /// WDM input exclusive
    pub fn mode_exlusif(&self) -> BoolParameter {
        BoolParameter::new(self.param("mode.exclusif"), self.remote)
    }
    /// WDM swift mode
    pub fn mode_swift(&self) -> BoolParameter {
        BoolParameter::new(self.param("mode.swift"), self.remote)
    }
    /// Option Monitor on SEL
    pub fn monitor_on_sel(&self) -> BoolParameter {
        BoolParameter::new(self.param("MonitorOnSEL"), self.remote)
    }
}
