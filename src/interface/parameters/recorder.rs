//! Recorder
use super::*;
use errors::*;

/// Recorder parameters
pub struct VoicemeeterRecorder<'a> {
    remote: &'a VoicemeeterRemote,
}

impl<'a> VoicemeeterRecorder<'a> {
    #[doc(hidden)]
    pub fn new(remote: &'a VoicemeeterRemote) -> Self {
        VoicemeeterRecorder { remote }
    }

    /// Get the identifier for an option: `Recorder.{dot}`
    pub fn param(&self, dot: impl Display) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(format!("{RECORDER}.{}", dot).into())
    }
    /// Stop the recorder
    pub fn stop(&self) -> BoolParameter {
        BoolParameter::new(self.param("stop"), self.remote)
    }
    /// Start the recorder
    pub fn play(&self) -> BoolParameter {
        BoolParameter::new(self.param("play"), self.remote)
    }
    /// Play from position
    pub fn replay(&self) -> BoolParameter {
        BoolParameter::new(self.param("replay"), self.remote)
    }
    /// Fast forward
    pub fn ff(&self) -> BoolParameter {
        BoolParameter::new(self.param("ff"), self.remote)
    }
    /// Rewind
    pub fn rew(&self) -> BoolParameter {
        BoolParameter::new(self.param("rew"), self.remote)
    }
    /// Goto position
    pub fn goto(&self) -> StringParameter {
        StringParameter::new(self.param("goto"), self.remote)
    }

    /// Set the assignation of the recorder
    pub fn out_bus_assignation(&self, bus: &Device) -> Result<BoolParameter, ParameterError> {
        if !bus.is_bus() {
            return Err(InvalidTypeError::ExpectedBus {
                device: format!("{:?}", bus),
            }
            .into());
        }
        Ok(BoolParameter::new(
            self.param(
                bus.as_bus_index(&self.remote.program)
                    .ok_or_else(|| DeviceError {
                        program: self.remote.program,
                        device: *bus,
                    })?
                    .1,
            ),
            self.remote,
        ))
    }

    /// Record
    pub fn record(&self) -> BoolParameter {
        BoolParameter::new(self.param("record"), self.remote)
    }

    /// Pause
    pub fn pause(&self) -> BoolParameter {
        BoolParameter::new(self.param("pause"), self.remote)
    }

    /// Load a file to play in the recorder
    pub fn load(&self) -> StringParameter<'a, true, false> {
        StringParameter::new(self.param("load"), self.remote)
    }

    /// Set samplerate
    pub fn samplerate(&self) -> IntParameter {
        IntParameter::new(self.param("samplerate"), self.remote, 0..=2)
    }

    /// Arm a strip to use as pre-fader input (multiple)
    pub fn arm_strip(&self, strip: impl StripIndex) -> Result<BoolParameter, ParameterError> {
        Ok(BoolParameter::new(
            self.param(format_args!(
                "ArmStrip({})",
                strip.into_strip_index(&self.remote.program)?
            )),
            self.remote,
        ))
    }

    /// Arm a bus to use as post-fader output (single)
    pub fn arm_bus(&self, bus: impl BusIndex) -> Result<BoolParameter, ParameterError> {
        Ok(BoolParameter::new(
            self.param(format_args!(
                "ArmBus({})",
                bus.into_bus_index(&self.remote.program)?
            )),
            self.remote,
        ))
    }

    /// Mode options
    pub fn mode(&self) -> VoicemeeterRecorderMode<'a> {
        VoicemeeterRecorderMode::new(self.remote)
    }

    /// Set the bit resolution. On of `8`, `16`, `24`, `32`
    pub fn bit_resolution(&self) -> IntParameter {
        IntParameter::new(self.param("bitResolution"), self.remote, 8..=32)
    }

    /// Channels to use for recording post-fader outputs, `2`, `4`, `6`, `8`
    pub fn channel(&self) -> IntParameter {
        IntParameter::new(self.param("Channel"), self.remote, 1..=8)
    }

    /// Set the bitrate for the recording of mp3
    pub fn kbps(&self) -> IntParameter {
        IntParameter::new(self.param("kbps"), self.remote, 32..=320)
    }

    /// Set the file type for the recording
    ///
    /// | i | Type |
    /// |---|------|
    /// |1  | WAV| ,
    /// |2  | AIFF|
    /// |3 | BWF|
    /// |100 | MP3|
    pub fn file_type(&self) -> IntParameter {
        IntParameter::new(self.param("FileType"), self.remote, 1..=100)
    }

    /// Set playback gain
    pub fn gain(&self) -> FloatParameter {
        FloatParameter::new(self.param("gain"), self.remote, -60.0..=12.0)
    }
}

/// Mode options for recorder
pub struct VoicemeeterRecorderMode<'a> {
    remote: &'a VoicemeeterRemote,
}

impl<'a> VoicemeeterRecorderMode<'a> {
    fn new(remote: &'a VoicemeeterRemote) -> Self {
        VoicemeeterRecorderMode { remote }
    }

    /// Get the identifier for an option: `Recorder.mode.{dot}`
    pub fn param(&self, dot: impl Display) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(format!("{RECORDER}.mode.{}", dot).into())
    }

    /// Record bus
    pub fn recbus(&self) -> BoolParameter {
        BoolParameter::new(self.param("recbus"), self.remote)
    }

    /// Play on load
    pub fn play_on_load(&self) -> BoolParameter {
        BoolParameter::new(self.param("PlayOnLoad"), self.remote)
    }

    /// Loop
    pub fn loop_(&self) -> BoolParameter {
        BoolParameter::new(self.param("Loop"), self.remote)
    }

    /// MultiTrack
    pub fn multi_track(&self) -> BoolParameter {
        BoolParameter::new(self.param("MultiTrack"), self.remote)
    }
}
