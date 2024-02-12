//! Fx
use super::*;

/// Fx parameters
pub struct VoicemeeterFx<'a> {
    remote: &'a VoicemeeterRemote,
}

// Parameter Name Value Range Remark Ver.
// Fx.Reverb.On 0 (off) or 1 (on) Switch On/Off 3
// Fx.Reverb.AB 0 (A) or 1 (B) Change A/B Mem 3
// Fx.Delay.On 0 (off) or 1 (on) Switch On/Off 3
// Fx.Delay.AB 0 (A) or 1 (B) Change A/B Mem 3

impl<'a> VoicemeeterFx<'a> {
    #[doc(hidden)]
    pub fn new(remote: &'a VoicemeeterRemote) -> Self {
        Self { remote }
    }

    /// Get the identifier for an option: `Fx.{dot}`
    pub fn param(&self, dot: impl Display) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(format!("{FX}.{}", dot).into())
    }

    /// Reverb status
    pub fn reverb_on(&self) -> BoolParameter {
        BoolParameter::new(self.param("Reverb.On"), self.remote)
    }
    /// Reverb AB choice
    pub fn reverb_ab(&self) -> BoolParameter {
        BoolParameter::new(self.param("Reverb.AB"), self.remote)
    }
    /// Delay status
    pub fn delay_on(&self) -> BoolParameter {
        BoolParameter::new(self.param("Delay.On"), self.remote)
    }
    /// Delay AB choice
    pub fn delay_ab(&self) -> BoolParameter {
        BoolParameter::new(self.param("Delay.AB"), self.remote)
    }
}
