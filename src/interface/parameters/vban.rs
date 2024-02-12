//! VBAN
use super::*;

/// Vban parameters
pub struct VoicemeeterVban<'a> {
    remote: &'a VoicemeeterRemote,
}

// vban.Enable 0 (off) or 1 (on) VBAN functions 1
// vban.instream[i].on 0 (off) or 1 (on) Stream On/Off 1
// vban.instream[i].name String Stream Name 1
// vban.instream[i].ip String IP Address from 1
// vban.instream[i].port 16 bit range PORT (Ethernet) 1
// vban.instream[i].sr 11025 to 96 kHz Read only 1
// vban.instream[i].channel 1 to 8 Read only 1
// vban.instream[i].bit VBAN data type Read only 1
// vban.instream[i].quality 0 to 4 0 = Optimal 1
// vban.instream[i].route 0 to 8 Strip Selector 1
// vban.outstream[i].on 0 (off) or 1 (on) Stream On/Off 1
// vban.outstream[i].name String Stream Name 1
// vban.outstream[i].ip String IP Address To 1
// vban.outstream[i].port 16 bit range PORT (Ethernet) 1
// vban.outstream[i].sr 11025 to 96 kHz 1
// vban.outstream[i].channel 1 to 8 1
// vban.outstream[i].bit VBAN data type 1 = 16 bits PCM 1
// vban.outstream[i].quality 0 to 4 0 = Optimal 1
// vban.outstream[i].route 0 to 8 BUS selector 1

impl<'a> VoicemeeterVban<'a> {
    #[doc(hidden)]
    pub fn new(remote: &'a VoicemeeterRemote) -> Self {
        Self { remote }
    }
    /// Get the identifier for an option: `Recorder.mode.{dot}`
    pub fn param(&self, dot: impl Display) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(format!("{VBAN}.{}", dot).into())
    }

    /// Turn VBAN on or off
    pub fn enable(&self) -> BoolParameter {
        BoolParameter::new(self.param("Enable"), self.remote)
    }

    /// Incoming VBAN stream
    pub fn incoming_stream(
        &self,
        index: impl Into<ZIndex>,
    ) -> Result<VoicemeeterVbanStream<'a, true>, ParameterError> {
        let index = index.into();
        const VALID: &[(VoicemeeterApplication, std::ops::RangeInclusive<u8>)] = &[
            (VoicemeeterApplication::Voicemeeter, 0..=3),
            (VoicemeeterApplication::VoicemeeterBanana, 0..=7),
            (VoicemeeterApplication::VoicemeeterPotato, 0..=7),
            (VoicemeeterApplication::PotatoX64Bits, 0..=7),
        ];
        let param = format!("{VBAN}.instream");
        match VALID.iter().find(|(app, _)| self.remote.program == *app) {
            None => {
                return Err(ParameterError::Version(InvalidVoicemeeterVersion {
                    expected: &[
                        VoicemeeterApplication::Voicemeeter,
                        VoicemeeterApplication::VoicemeeterBanana,
                        VoicemeeterApplication::VoicemeeterPotato,
                        VoicemeeterApplication::PotatoX64Bits,
                    ],
                    found: self.remote.program,
                    parameter: param,
                }));
            }
            Some((_, i)) if i.contains(&(index.0 as u8)) => Ok(
                VoicemeeterVbanStream::<'a, true>::new(self.remote, index.into()),
            ),
            _ => Err(ParameterError::OutOfRange(OutOfRangeError {
                name: format!("{param}"),
                index,
                program: self.remote.program,
            })),
        }
    }

    /// Outgoing VBAN stream
    pub fn outgoing_stream(
        &self,
        index: impl Into<ZIndex>,
    ) -> Result<VoicemeeterVbanStream<'a, false>, ParameterError> {
        let index = index.into();
        const VALID: &[(VoicemeeterApplication, std::ops::RangeInclusive<u8>)] = &[
            (VoicemeeterApplication::Voicemeeter, 0..=3),
            (VoicemeeterApplication::VoicemeeterBanana, 0..=7),
            (VoicemeeterApplication::VoicemeeterPotato, 0..=7),
            (VoicemeeterApplication::PotatoX64Bits, 0..=7),
        ];
        let param = format!("{VBAN}.outstream");
        match VALID.iter().find(|(app, _)| self.remote.program == *app) {
            None => {
                return Err(ParameterError::Version(InvalidVoicemeeterVersion {
                    expected: &[
                        VoicemeeterApplication::Voicemeeter,
                        VoicemeeterApplication::VoicemeeterBanana,
                        VoicemeeterApplication::VoicemeeterPotato,
                        VoicemeeterApplication::PotatoX64Bits,
                    ],
                    found: self.remote.program,
                    parameter: param,
                }));
            }
            Some((_, i)) if i.contains(&(index.0 as u8)) => Ok(
                VoicemeeterVbanStream::<'a, false>::new(self.remote, index.into()),
            ),
            _ => Err(ParameterError::OutOfRange(OutOfRangeError {
                name: format!("{param}"),
                index,
                program: self.remote.program,
            })),
        }
    }
}

/// A VBAN stream, input or output
pub struct VoicemeeterVbanStream<'a, const INPUT: bool> {
    remote: &'a VoicemeeterRemote,
    index: ZIndex,
}

impl<'a, const INPUT: bool> VoicemeeterVbanStream<'a, INPUT> {
    #[doc(hidden)]
    pub fn new(remote: &'a VoicemeeterRemote, index: ZIndex) -> Self {
        Self { remote, index }
    }

    /// Get the identifier for an option: `Vban.{input}stream.{dot}`
    pub fn param(&self, dot: impl Display) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(
            format!(
                "{VBAN}.{}stream[{}].{}",
                match INPUT {
                    true => "in",
                    false => "out",
                },
                self.index,
                dot
            )
            .into(),
        )
    }
    /// Stream On/Off
    pub fn on(&self) -> BoolParameter {
        BoolParameter::new(self.param("on"), self.remote)
    }
    /// Stream name
    pub fn name(&self) -> StringParameter {
        StringParameter::new(self.param("name"), self.remote)
    }
    /// IP Address
    pub fn ip(&self) -> StringParameter {
        StringParameter::new(self.param("ip"), self.remote)
    }

    /// Port
    pub fn port(&self) -> IntParameter {
        IntParameter::new(self.param("port"), self.remote, 0..=u16::MAX as i32)
    }

    /// Quality
    pub fn quality(&self) -> IntParameter {
        IntParameter::new(self.param("quality"), self.remote, 0..=4)
    }
    /// Strip Selector
    pub fn route(&self) -> IntParameter {
        IntParameter::new(self.param("route"), self.remote, 0..=8)
    }
}

impl<'a> VoicemeeterVbanStream<'a, true> {
    /// Sample rate
    pub fn sample_rate(&self) -> IntParameter<'a, false, true> {
        IntParameter::new(self.param("sr"), self.remote, 11025..=96000)
    }

    /// Channel
    ///
    /// 1 to 8
    pub fn channel(&self) -> IntParameter<'a, false, true> {
        IntParameter::new(self.param("channel"), self.remote, 1..=8)
    }
    /// VBAN data type
    ///
    /// |type|format|
    /// |----|------|
    /// |1|16 bits PCM|
    /// |2|24 bits PCM|
    pub fn bit(&self) -> IntParameter<'a, false, true> {
        IntParameter::new(self.param("bit"), self.remote, 1..=2)
    }
}

impl<'a> VoicemeeterVbanStream<'a, false> {
    /// Sample rate
    pub fn sample_rate(&self) -> IntParameter {
        IntParameter::new(self.param("sr"), self.remote, 11025..=96000)
    }
    /// Channel
    ///
    /// 1 to 8
    pub fn channel(&self) -> IntParameter {
        IntParameter::new(self.param("channel"), self.remote, 1..=8)
    }
    /// VBAN data type
    ///
    /// |type|format|
    /// |----|------|
    /// |1|16 bits PCM|
    /// |2|24 bits PCM|
    pub fn bit(&self) -> IntParameter {
        IntParameter::new(self.param("bit"), self.remote, 1..=2)
    }
}
