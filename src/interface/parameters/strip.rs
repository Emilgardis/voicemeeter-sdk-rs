//! Strip parameters
use super::*;

/// Parameters for a strip.
///
/// A strip is a physical or virtual input.
///
/// Returned by [`VoicemeeterRemote::parameters().strip(i)`](VoicemeeterRemote::parameters)
///
///
/// # Example
///
/// ```rust,no_run
/// use voicemeeter::VoicemeeterRemote;
///
/// // Get the client.
/// let remote = VoicemeeterRemote::new()?;
///
/// // Get the label of strip 1 (index 0)
/// println!("{}", remote.parameters().strip(0)?.label().get()?);
/// // Set strip 3 (index 2) to output to A1
/// remote.parameters().strip(2)?.a1().set(true)?;
///
/// // Ensure the change is registered.
/// remote.is_parameters_dirty()?;
///
/// // We need to sleep here because otherwise the change won't be registered,
/// // in a long running program this is not needed.
/// std::thread::sleep(std::time::Duration::from_millis(50));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Strip<'a> {
    remote: &'a VoicemeeterRemote,
    strip_index: ZIndex,
}

impl<'a> Strip<'a> {
    #[doc(hidden)]
    pub fn new(remote: &'a VoicemeeterRemote, strip_index: ZIndex) -> Self {
        Strip {
            remote,
            strip_index,
        }
    }
    /// Get the identifier for a parameter on this strip: `Strip[i].{dot}`
    pub fn param(&self, dot: impl Display) -> Cow<'static, ParameterNameRef> {
        // TODO: Should this maybe allow custom names?
        Cow::Owned(format!("{STRIP}[{}].{}", self.strip_index, dot).into())
    }
    /// Strip is physical
    #[rustfmt::skip]
    pub fn is_physical(&self) -> bool {
        matches!((self.remote.program, self.strip_index.0),
            | (VoicemeeterApplication::Voicemeeter, 0..=1)
            | (VoicemeeterApplication::VoicemeeterBanana, 0..=2)
            | (VoicemeeterApplication::VoicemeeterPotato, 0..=4)
            | (VoicemeeterApplication::PotatoX64Bits, 0..=4)
        )
    }

    /// Strip is virtual
    pub fn is_virtual(&self) -> bool {
        !(self.is_physical() || matches!(self.remote.program, VoicemeeterApplication::Other))
    }

    /// Mono Button
    pub fn mono(&self) -> BoolParameter {
        BoolParameter::new(self.param("Mono"), self.remote)
    }

    /// Mute Button
    pub fn mute(&self) -> BoolParameter {
        BoolParameter::new(self.param("Mute"), self.remote)
    }

    /// Solo Button
    pub fn solo(&self) -> BoolParameter {
        BoolParameter::new(self.param("Solo"), self.remote)
    }

    // FIXME: Only available in virtual input and input8
    /// Mute Center Button
    pub fn mute_center(&self) -> BoolParameter {
        BoolParameter::new(self.param("MC"), self.remote)
    }

    /// Gain slider
    pub fn gain(&self) -> FloatParameter {
        FloatParameter::new(self.param("Gain"), self.remote, -60.0..=12.0)
    }

    // TODO: zindex for bus
    /// Gain slider for a bus
    pub fn gain_layer(&self, layer: impl Into<ZIndex>) -> FloatParameter {
        let layer = layer.into();
        let name = self.param(format!("GainLayer[{layer}]"));
        FloatParameter::new(name, self.remote, -60.0..=12.0)
    }

    // TODO: zindex for bus
    /// Pan in x direction
    pub fn pan_x(&self) -> FloatParameter {
        FloatParameter::new(self.param("Pan_x"), self.remote, -0.5..=0.5)
    }

    /// Pan in y direction
    pub fn pan_y(&self) -> FloatParameter {
        // FIXME: docs says for range: 0 to 1.0 (-0.5 to 0.5 for 5.1 pan pot)
        FloatParameter::new_unranged(self.param("Pan_y"), self.remote)
    }

    /// Color of physical strip in x direction
    pub fn color_x(&self) -> Result<FloatParameter, InvalidTypeError> {
        if self.is_virtual() {
            Err(InvalidTypeError::ExpectedPhysical {
                name: STRIP,
                strip_index: self.strip_index,
                parameter: "Color_x".to_string(),
            })
        } else {
            Ok(FloatParameter::new(
                self.param("Color_x"),
                self.remote,
                -0.5..=0.5,
            ))
        }
    }

    /// Color of physical strip in y direction
    pub fn color_y(&self) -> Result<FloatParameter, InvalidTypeError> {
        if self.is_virtual() {
            Err(InvalidTypeError::ExpectedPhysical {
                name: STRIP,
                strip_index: self.strip_index,
                parameter: "Color_y".to_string(),
            })
        } else {
            Ok(FloatParameter::new(
                self.param("Color_y"),
                self.remote,
                0.0..=1.0,
            ))
        }
    }

    /// FX of physical strip in x direction
    pub fn fx_x(&self) -> Result<FloatParameter, InvalidTypeError> {
        if self.is_virtual() {
            Err(InvalidTypeError::ExpectedPhysical {
                name: STRIP,
                strip_index: self.strip_index,
                parameter: "fx_x".to_string(),
            })
        } else {
            Ok(FloatParameter::new(
                self.param("fx_x"),
                self.remote,
                -0.5..=0.5,
            ))
        }
    }

    /// FX of physical strip in y direction
    pub fn fx_y(&self) -> Result<FloatParameter, InvalidTypeError> {
        if self.is_virtual() {
            Err(InvalidTypeError::ExpectedPhysical {
                name: STRIP,
                strip_index: self.strip_index,
                parameter: "fx_y".to_string(),
            })
        } else {
            Ok(FloatParameter::new(
                self.param("fx_y"),
                self.remote,
                0.0..=1.0,
            ))
        }
    }

    /// Audability
    pub fn audability(&self) -> FloatParameter {
        FloatParameter::new(self.param("Audability"), self.remote, 0.0..=10.0)
    }
    // FIXME: Only available in virtual input aux
    /// Compression
    ///
    /// See also [Strip::comp_detailed] for detailed compressor settings
    pub fn comp(&self) -> FloatParameter {
        FloatParameter::new(self.param("Comp"), self.remote, 0.0..=10.0)
    }

    /// Compressor detailed parameters/settings
    ///
    /// Only works on Voicemeeter Potato
    pub fn comp_detailed(&self) -> Result<StripCompressor, ParameterError> {
        const VALID: &[VoicemeeterApplication] = &[
            VoicemeeterApplication::VoicemeeterPotato,
            VoicemeeterApplication::PotatoX64Bits,
        ];
        if VALID.contains(&self.remote.program) {
            if self.is_physical() {
                Ok(StripCompressor::new(self.remote, self.strip_index))
            } else {
                Err(InvalidTypeError::ExpectedPhysical {
                    name: STRIP,
                    strip_index: self.strip_index,
                    parameter: "Comp".to_string(),
                }
                .into())
            }
        } else {
            Err(InvalidVoicemeeterVersion {
                expected: VALID,
                found: self.remote.program,
                parameter: self.param("Comp").to_string(),
            }
            .into())
        }
    }

    /// Gate
    ///
    /// See also [Strip::gate_detailed] for detailed gate settings
    pub fn gate(&self) -> FloatParameter {
        FloatParameter::new(self.param("Gate"), self.remote, 0.0..=10.0)
    }

    /// Gate detailed parameters/settings
    ///
    /// Only works on Voicemeeter Potato
    pub fn gate_detailed(&self) -> Result<StripGate, ParameterError> {
        const VALID: &[VoicemeeterApplication] = &[
            VoicemeeterApplication::VoicemeeterPotato,
            VoicemeeterApplication::PotatoX64Bits,
        ];

        if VALID.contains(&self.remote.program) {
            if self.is_physical() {
                Ok(StripGate::new(self.remote, self.strip_index))
            } else {
                Err(InvalidTypeError::ExpectedPhysical {
                    name: STRIP,
                    strip_index: self.strip_index,
                    parameter: "Gate".to_string(),
                }
                .into())
            }
        } else {
            Err(InvalidVoicemeeterVersion {
                expected: VALID,
                found: self.remote.program,
                parameter: self.param("Gate").to_string(),
            }
            .into())
        }
    }

    /// Denoiser Knob
    pub fn denoiser(&self) -> Result<FloatParameter, ParameterError> {
        const VALID: &[VoicemeeterApplication] = &[
            VoicemeeterApplication::VoicemeeterPotato,
            VoicemeeterApplication::PotatoX64Bits,
        ];

        if VALID.contains(&self.remote.program) {
            if self.is_physical() {
                Ok(FloatParameter::new(
                    self.param("Denoiser"),
                    self.remote,
                    0.0..10.0,
                ))
            } else {
                Err(InvalidTypeError::ExpectedPhysical {
                    name: STRIP,
                    strip_index: self.strip_index,
                    parameter: "Gate".to_string(),
                }
                .into())
            }
        } else {
            Err(InvalidVoicemeeterVersion {
                expected: VALID,
                found: self.remote.program,
                parameter: self.param("Gate").to_string(),
            }
            .into())
        }
    }

    /// Karaoke
    pub fn karaoke(&self) -> IntParameter {
        IntParameter::new(self.param("Karaoke"), self.remote, 0..=4)
    }

    /// Limit
    pub fn limit(&self) -> IntParameter {
        IntParameter::new(self.param("Limit"), self.remote, -40..=12)
    }

    /// EQGain1 of virtual strip
    pub fn eq_gain1(&self) -> Result<FloatParameter, InvalidTypeError> {
        if self.is_physical() {
            Err(InvalidTypeError::ExpectedPhysical {
                name: STRIP,
                strip_index: self.strip_index,
                parameter: "EQGain1".to_string(),
            })
        } else {
            Ok(FloatParameter::new(
                self.param("EQGain1"),
                self.remote,
                -12.0..=12.0,
            ))
        }
    }

    /// EQGain2 of virtual strip
    pub fn eq_gain2(&self) -> Result<FloatParameter, InvalidTypeError> {
        if self.is_physical() {
            Err(InvalidTypeError::ExpectedPhysical {
                name: STRIP,
                strip_index: self.strip_index,
                parameter: "EQGain2".to_string(),
            })
        } else {
            Ok(FloatParameter::new(
                self.param("EQGain2"),
                self.remote,
                -12.0..=12.0,
            ))
        }
    }

    /// EQGain3 of virtual strip
    pub fn eq_gain3(&self) -> Result<FloatParameter, InvalidTypeError> {
        if self.is_physical() {
            Err(InvalidTypeError::ExpectedPhysical {
                name: STRIP,
                strip_index: self.strip_index,
                parameter: "EQGain3".to_string(),
            })
        } else {
            Ok(FloatParameter::new(
                self.param("EQGain3"),
                self.remote,
                -12.0..=12.0,
            ))
        }
    }

    /// Label
    pub fn label(&self) -> StringParameter {
        StringParameter::new(self.param("Label"), self.remote)
    }

    /// Out BUS Assignation for A1
    pub fn a1(&self) -> BoolParameter {
        BoolParameter::new(self.param("A1"), self.remote)
    }
    /// Out BUS Assignation for A2
    pub fn a2(&self) -> BoolParameter {
        BoolParameter::new(self.param("A2"), self.remote)
    }
    /// Out BUS Assignation for A3
    pub fn a3(&self) -> BoolParameter {
        BoolParameter::new(self.param("A3"), self.remote)
    }
    /// Out BUS Assignation for A4
    pub fn a4(&self) -> BoolParameter {
        BoolParameter::new(self.param("A4"), self.remote)
    }
    /// Out BUS Assignation for A5
    pub fn a5(&self) -> BoolParameter {
        BoolParameter::new(self.param("A5"), self.remote)
    }
    /// Out BUS Assignation for B1
    pub fn b1(&self) -> BoolParameter {
        BoolParameter::new(self.param("B1"), self.remote)
    }
    /// Out BUS Assignation for B2
    pub fn b2(&self) -> BoolParameter {
        BoolParameter::new(self.param("B2"), self.remote)
    }
    /// Out BUS Assignation for B3
    pub fn b3(&self) -> BoolParameter {
        BoolParameter::new(self.param("B3"), self.remote)
    }
    /// EQ Button
    pub fn eq_on(&self) -> BoolParameter {
        BoolParameter::new(self.param("EQ.on"), self.remote)
    }
    /// EQ Memory Slot
    pub fn eq_ab(&self) -> BoolParameter {
        BoolParameter::new(self.param("EQ.AB"), self.remote)
    }
    /// EQ on channel
    pub fn eq(&self, channel: usize) -> Result<EqChannelParameter, ParameterError> {
        const VALID: &[VoicemeeterApplication] = &[
            VoicemeeterApplication::VoicemeeterPotato,
            VoicemeeterApplication::PotatoX64Bits,
        ];
        let eq = EqChannelParameter::new_strip(self.remote, self.strip_index, channel);
        if VALID.contains(&self.remote.program) {
            if self.is_physical() {
                Ok(eq)
            } else {
                Err(InvalidTypeError::ExpectedPhysical {
                    name: STRIP,
                    strip_index: self.strip_index,
                    parameter: eq.name().to_string(),
                }
                .into())
            }
        } else {
            Err(InvalidVoicemeeterVersion {
                expected: VALID,
                found: self.remote.program,
                parameter: eq.name().to_string(),
            }
            .into())
        }
    }
    /// Fade to
    pub fn fade_to(&self) -> TupleParameter<'_, i32, usize> {
        TupleParameter::new(self.param("FadeTo"), self.remote)
    }
    /// Fade by
    pub fn fade_by(&self) -> TupleParameter<'_, i32, usize> {
        TupleParameter::new(self.param("FadeBy"), self.remote)
    }
    /// Send Level To Reverb
    pub fn reverb(&self) -> FloatParameter {
        FloatParameter::new(self.param("Reverb"), self.remote, 0.0..=10.0)
    }
    /// Send Level To Delay
    pub fn delay(&self) -> FloatParameter {
        FloatParameter::new(self.param("Delay"), self.remote, 0.0..=10.0)
    }
    /// Send Level To External Fx1
    pub fn fx1(&self) -> FloatParameter {
        FloatParameter::new(self.param("Fx1"), self.remote, 0.0..=10.0)
    }
    /// Send Level To External Fx2
    pub fn fx2(&self) -> FloatParameter {
        FloatParameter::new(self.param("Fx2"), self.remote, 0.0..=10.0)
    }
    /// Post Reverb button
    pub fn post_reverb(&self) -> BoolParameter {
        BoolParameter::new(self.param("PostReverb"), self.remote)
    }
    /// Post Delay button
    pub fn post_delay(&self) -> BoolParameter {
        BoolParameter::new(self.param("PostDelay"), self.remote)
    }
    /// Post Fx1 button
    pub fn post_fx1(&self) -> BoolParameter {
        BoolParameter::new(self.param("PostFx1"), self.remote)
    }
    /// Post Fx2 button
    pub fn post_fx2(&self) -> BoolParameter {
        BoolParameter::new(self.param("PostFx2"), self.remote)
    }

    /// Application gain
    pub fn app_gain_indexed(&self, application_index: ZIndex) -> FloatParameter<'_, true, false> {
        FloatParameter::new(
            self.param(format!("App[{application_index}].Gain")),
            self.remote,
            0.0..=1.0,
        )
    }

    /// Application Mute
    pub fn app_mute_indexed(&self, application_index: ZIndex) -> BoolParameter<'_, true, false> {
        BoolParameter::new(
            self.param(format!("App[{application_index}].Mute")),
            self.remote,
        )
    }

    /// Application gain
    pub fn app_gain(&self) -> TupleParameter<'_, String, f32, true, false> {
        TupleParameter::new(self.param("AppGain"), self.remote)
    }

    /// Application Mute
    pub fn app_mute(&self) -> TupleParameter<'_, String, bool, true, false> {
        TupleParameter::new(self.param("AppMute"), self.remote)
    }

    /// Audio Device information
    pub fn device(&self) -> Result<StripDevice<'a>, InvalidTypeError> {
        if self.is_virtual() {
            Err(InvalidTypeError::ExpectedPhysical {
                name: STRIP,
                strip_index: self.strip_index,
                parameter: "device".to_string(),
            })
        } else {
            Ok(StripDevice::new(self.remote, self.strip_index))
        }
    }
}

/// Bus device parameters
pub struct StripDevice<'a> {
    remote: &'a VoicemeeterRemote,
    strip_index: ZIndex,
}

impl<'a> StripDevice<'a> {
    fn new(remote: &'a VoicemeeterRemote, strip_index: ZIndex) -> Self {
        Self {
            remote,
            strip_index,
        }
    }

    /// Get the identifier for a device parameter on this strip: `Strip[i].device.{dot}`
    pub fn param(&self, dot: impl ToString) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(format!("{STRIP}[{}].device.{}", self.strip_index, dot.to_string()).into())
    }

    /// Name of the device.
    pub fn name(&self) -> StringParameter<'a, false, true> {
        StringParameter::new(self.param("name"), self.remote)
    }

    /// Samplerate of the device.
    pub fn sr(&self) -> IntParameter<'a, false, true> {
        IntParameter::new_unranged(self.param("sr"), self.remote)
    }
    /// WDM device
    pub fn wdm(&self) -> StringParameter<'a, true, false> {
        StringParameter::new(self.param("wdm"), self.remote)
    }
    /// KS device
    pub fn ks(&self) -> StringParameter<'a, true, false> {
        StringParameter::new(self.param("ks"), self.remote)
    }
    /// MME device
    pub fn mme(&self) -> StringParameter<'a, true, false> {
        StringParameter::new(self.param("mme"), self.remote)
    }
    /// ASIO device
    pub fn asio(&self) -> StringParameter<'a, true, false> {
        StringParameter::new(self.param("asio"), self.remote)
    }
}

/// Compressor detailed parameters/settings
///
/// Only works on Voicemeeter Potato
pub struct StripCompressor<'a> {
    remote: &'a VoicemeeterRemote,
    strip_index: ZIndex,
}

impl<'a> StripCompressor<'a> {
    fn new(remote: &'a VoicemeeterRemote, strip_index: ZIndex) -> Self {
        Self {
            remote,
            strip_index,
        }
    }

    /// Get the identifier for a compressor parameter on this strip: `Strip[i].compressor.{dot}`
    pub fn param(&self, dot: impl ToString) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(format!("{STRIP}[{}].comp.{}", self.strip_index, dot.to_string()).into())
    }

    /// Input Gain
    ///
    /// To control the gain before compression.
    pub fn gain_in(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("GainIn"), self.remote, -24.0..24.0)
    }
    /// Ratio
    ///
    /// Gives the compression rate.
    pub fn ratio(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Ratio"), self.remote, 1.0..8.0)
    }
    /// Threshold
    ///
    /// Define a level to start the compression when
    /// the input signal goes over this threshold.
    pub fn threshold(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Threshold"), self.remote, -40.0..-3.0)
    }
    /// Attack Time (ms)
    ///
    /// to control the compression behavior on
    /// sound attack (when the input signal starts to go over the threshold)
    pub fn attack(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Attack"), self.remote, 0.0..200.0)
    }
    /// Release Time (ms)
    ///  to control the compression behavior when
    /// the signal goes down
    pub fn release(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Release"), self.remote, 0.0..5000.0)
    }
    /// Knee.
    ///
    /// To control the compression transition softness on
    /// threshold point
    pub fn knee(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Knee"), self.remote, 0.0..1.0)
    }
    /// Output Gain
    ///
    /// To control the gain after compression
    pub fn gain_out(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("GainOut"), self.remote, -24.0..24.0)
    }
    /// Auto Make Up Option
    ///
    /// apply an output gain automatically
    /// computed to compensate the compression
    pub fn make_up(&self) -> BoolParameter<'a, true, true> {
        BoolParameter::new(self.param("MakeUp"), self.remote)
    }
}

/// Gate detailed parameters/settings
///
/// Only works on Voicemeeter Potato
pub struct StripGate<'a> {
    remote: &'a VoicemeeterRemote,
    strip_index: ZIndex,
}

impl<'a> StripGate<'a> {
    fn new(remote: &'a VoicemeeterRemote, strip_index: ZIndex) -> Self {
        Self {
            remote,
            strip_index,
        }
    }

    /// Get the identifier for a gate parameter on this strip: `Strip[i].gate.{dot}`
    pub fn param(&self, dot: impl ToString) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(format!("{STRIP}[{}].Gate.{}", self.strip_index, dot.to_string()).into())
    }

    /// Threshold
    ///
    /// If input gain is below this level the gate is
    /// closing, above this level the gate is opening.
    pub fn threshold(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Threshold"), self.remote, -60.0..-10.0)
    }
    /// Damping Max
    ///
    /// Allows limiting the gain reduction when
    /// the gate is closing. Per default OFF = -inf, the gate
    /// completely remove the signal when closing.
    pub fn damping(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Damping"), self.remote, -60.0..-10.0)
    }
    /// Band Pass Sidechain (hz)
    ///
    /// This parameters allows to define a Band Pass frequency (1,5 octave)
    /// in the sidechain (input signal
    /// controlling the gate). Then the gate will react on specific
    /// frequency range only.
    pub fn bp_sidechain(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("BPSidechain"), self.remote, 100.0..=4000.0)
    }
    /// Attack Time (ms)
    ///
    /// Define how long it takes to open the gate (gain increasing time).
    pub fn attack(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Attack"), self.remote, 0.0..=1000.0)
    }
    /// Hold Time (ms)
    ///
    /// Define the minimal time the gate stays
    /// opened anyway (whatever the input gain).
    pub fn hold(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Hold"), self.remote, 0.0..=5000.0)
    }
    /// Release Time (ms)
    ///
    /// Define how long it takes to close the gate
    /// (gain decreasing time).
    pub fn release(&self) -> FloatParameter<'a, true, true> {
        FloatParameter::new(self.param("Release"), self.remote, 0.0..=5000.0)
    }
}
