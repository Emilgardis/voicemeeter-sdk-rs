//! Bus parameters
use super::*;

/// Parameters for a bus.
///
/// A bus is a output.
///
/// # Example
///
/// ```rust,no_run
/// use voicemeeter::VoicemeeterRemote;
///
/// // Get the client.
/// let remote = VoicemeeterRemote::new()?;
///
/// // Get the label of bus A1 (index 0)
/// println!("{}", remote.parameters().bus(0)?.label().get()?);
///
/// // Mute bus A4 (index 5)
/// remote.parameters().bus(2)?.mute().set(true)?;
///
/// // Ensure the change is registered.
/// remote.is_parameters_dirty()?;
///
/// // We need to sleep here because otherwise changes won't be registered,
/// // in a long running program this is not needed.
/// std::thread::sleep(std::time::Duration::from_millis(50));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Bus<'a> {
    remote: &'a VoicemeeterRemote,
    strip_index: ZIndex,
}

impl<'a> Bus<'a> {
    #[doc(hidden)]
    pub fn new(remote: &'a VoicemeeterRemote, strip_index: ZIndex) -> Self {
        Bus {
            remote,
            strip_index,
        }
    }

    fn param(&self, dot: impl ToString) -> Cow<'static, ParameterNameRef> {
        // TODO: Should this maybe allow custom names?
        Cow::Owned(format!("{BUS}[{}].{}", self.strip_index, dot.to_string()).into())
    }

    /// Label
    pub fn label(&self) -> StringParameter {
        StringParameter::new(self.param("Label"), self.remote)
    }

    /// Mono Button
    pub fn mono(&self) -> IntParameter {
        IntParameter::new(self.param("Mono"), self.remote, 0..=2)
    }

    /// Mute Button
    pub fn mute(&self) -> BoolParameter {
        BoolParameter::new(self.param("Mute"), self.remote)
    }

    /// EQ Button
    pub fn eq_on(&self) -> BoolParameter {
        BoolParameter::new(self.param("EQ.on"), self.remote)
    }

    /// EQ Memory Slot
    pub fn eq_ab(&self) -> BoolParameter {
        BoolParameter::new(self.param("EQ.AHB"), self.remote)
    }

    /// Gain slider
    pub fn gain(&self) -> IntParameter {
        IntParameter::new(self.param("gain"), self.remote, -60..=12)
    }

    /// Bus mode Normal
    pub fn mode(&self) -> BusModeParameter {
        BusModeParameter::new(self.remote, self.strip_index)
    }

    /// EQ on channel
    pub fn eq(&self, channel: usize) -> EqChannelParameter {
        EqChannelParameter::new(self.remote, self.strip_index, channel)
    }
    /// Fade to
    pub fn fade_to(&self) -> TupleParameter<'_, i32, usize> {
        TupleParameter::new(self.param("FadeTo"), self.remote)
    }
    /// Fade by
    pub fn fade_by(&self) -> TupleParameter<'_, i32, usize> {
        TupleParameter::new(self.param("FadeBy"), self.remote)
    }
    /// BUS SEL Button
    pub fn sel(&self) -> BoolParameter {
        BoolParameter::new(self.param("Sel"), self.remote)
    }
    /// Reverb return
    pub fn return_reverb(&self) -> IntParameter {
        IntParameter::new(self.param("ReturnReverb"), self.remote, 0..=10)
    }
    /// Delay return
    pub fn return_delay(&self) -> IntParameter {
        IntParameter::new(self.param("ReturnDelay"), self.remote, 0..=10)
    }
    /// Fx1 Return
    pub fn return_fx1(&self) -> IntParameter {
        IntParameter::new(self.param("ReturnFx1"), self.remote, 0..=10)
    }
    /// Fx2 Return
    pub fn return_fx2(&self) -> IntParameter {
        IntParameter::new(self.param("ReturnFx2"), self.remote, 0..=10)
    }
    /// Monitor
    pub fn monitor(&self) -> BoolParameter {
        BoolParameter::new(self.param("Monitor"), self.remote)
    }
    /// Audio Device information
    pub fn device(&self) -> BusDevice {
        BusDevice::new(self.remote, self.strip_index)
    }
}

/// Parameters for bus mode
pub struct BusModeParameter<'a> {
    remote: &'a VoicemeeterRemote,
    strip_index: ZIndex,
}

impl<'a> BusModeParameter<'a> {
    fn new(remote: &'a VoicemeeterRemote, strip_index: ZIndex) -> Self {
        Self {
            remote,
            strip_index,
        }
    }

    fn param(&self, dot: impl ToString) -> Cow<'static, ParameterNameRef> {
        // TODO: Should this maybe allow custom names?
        Cow::Owned(format!("{BUS}[{}].mode.{}", self.strip_index, dot.to_string()).into())
    }

    /// Get the current bus mode
    pub fn get(&self) -> Result<Option<BusMode>, GetParameterError> {
        Ok(Some(if self.is_normal()? {
            BusMode::Normal
        } else if self.is_amix()? {
            BusMode::Amix
        } else if self.is_bmix()? {
            BusMode::Bmix
        } else if self.is_repeat()? {
            BusMode::Repeat
        } else if self.is_composite()? {
            BusMode::Composite
        } else if self.is_tv_mix()? {
            BusMode::TvMix
        } else if self.is_up_mix21()? {
            BusMode::UpMix21
        } else if self.is_up_mix41()? {
            BusMode::UpMix41
        } else if self.is_up_mix61()? {
            BusMode::UpMix61
        } else if self.is_center_only()? {
            BusMode::CenterOnly
        } else if self.is_lfe_only()? {
            BusMode::LfeOnly
        } else if self.is_rear_only()? {
            BusMode::RearOnly
        } else {
            return Ok(None);
        }))
    }
    /// Set the bus mode
    pub fn set(&self, mode: BusMode) -> Result<(), SetParameterError> {
        match mode {
            BusMode::Normal => self.set_normal(true),
            BusMode::Amix => self.set_amix(true),
            BusMode::Bmix => self.set_bmix(true),
            BusMode::Repeat => self.set_repeat(true),
            BusMode::Composite => self.set_composite(true),
            BusMode::TvMix => self.set_tv_mix(true),
            BusMode::UpMix21 => self.set_up_mix21(true),
            BusMode::UpMix41 => self.set_up_mix41(true),
            BusMode::UpMix61 => self.set_up_mix61(true),
            BusMode::CenterOnly => self.set_center_only(true),
            BusMode::LfeOnly => self.set_lfe_only(true),
            BusMode::RearOnly => self.set_rear_only(true),
        }
    }

    /// Returns `true` if the bus mode is [`Normal`](BusMode::Normal)
    pub fn is_normal(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("normal"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`Amix`](BusMode::Amix)
    pub fn is_amix(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("Amix"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`Bmix`](BusMode::Bmix)
    pub fn is_bmix(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("Bmix"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`Repeat`](BusMode::Repeat)
    pub fn is_repeat(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("Repeat"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`Composite`](BusMode::Composite)
    pub fn is_composite(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("Composite"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`TvMix`](BusMode::TvMix)
    pub fn is_tv_mix(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("TVMix"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`UpMix21`](BusMode::UpMix21)
    pub fn is_up_mix21(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("UpMix21"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`UpMix41`](BusMode::UpMix41)
    pub fn is_up_mix41(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("UpMix41"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`UpMix61`](BusMode::UpMix61)
    pub fn is_up_mix61(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("UpMix61"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`CenterOnly`](BusMode::CenterOnly)
    pub fn is_center_only(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("CenterOnly"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`LfeOnly`](BusMode::LfeOnly)
    pub fn is_lfe_only(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("LFEOnly"), self.remote).get()
    }
    /// Returns `true` if the bus mode is [`RearOnly`](BusMode::RearOnly)
    pub fn is_rear_only(&self) -> Result<bool, GetParameterError> {
        BoolParameter::<'_, false, true>::new(self.param("RearOnly"), self.remote).get()
    }

    /// Set the bus mode for [`Normal`](BusMode::Normal)
    pub fn set_normal(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("normal"), self.remote).set(val)
    }
    /// Set the bus mode for [`Amix`](BusMode::Amix)
    pub fn set_amix(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("Amix"), self.remote).set(val)
    }
    /// Set the bus mode for [`Bmix`](BusMode::Bmix)
    pub fn set_bmix(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("Bmix"), self.remote).set(val)
    }
    /// Set the bus mode for [`Repeat`](BusMode::Repeat)
    pub fn set_repeat(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("Repeat"), self.remote).set(val)
    }
    /// Set the bus mode for [`Composite`](BusMode::Composite)
    pub fn set_composite(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("Composite"), self.remote).set(val)
    }
    /// Set the bus mode for [`TvMix`](BusMode::TvMix)
    pub fn set_tv_mix(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("TVMix"), self.remote).set(val)
    }
    /// Set the bus mode for [`UpMix21`](BusMode::UpMix21)
    pub fn set_up_mix21(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("UpMix21"), self.remote).set(val)
    }
    /// Set the bus mode for [`UpMix41`](BusMode::UpMix41)
    pub fn set_up_mix41(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("UpMix41"), self.remote).set(val)
    }
    /// Set the bus mode for [`UpMix61`](BusMode::UpMix61)
    pub fn set_up_mix61(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("UpMix61"), self.remote).set(val)
    }
    /// Set the bus mode for [`CenterOnly`](BusMode::CenterOnly)
    pub fn set_center_only(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("CenterOnly"), self.remote).set(val)
    }
    /// Set the bus mode for [`LfeOnly`](BusMode::LfeOnly)
    pub fn set_lfe_only(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("LFEOnly"), self.remote).set(val)
    }
    /// Set the bus mode for [`RearOnly`](BusMode::RearOnly)
    pub fn set_rear_only(&self, val: bool) -> Result<(), SetParameterError> {
        BoolParameter::<'_, true, false>::new(self.param("RearOnly"), self.remote).set(val)
    }
}

/// Parameter for EQ on a specific channel
pub struct EqChannelParameter<'a> {
    remote: &'a VoicemeeterRemote,
    strip_index: ZIndex,
    channel: usize,
}

impl<'a> EqChannelParameter<'a> {
    fn new(remote: &'a VoicemeeterRemote, strip_index: ZIndex, channel: usize) -> Self {
        Self {
            remote,
            strip_index,
            channel,
        }
    }

    fn param(&self, cell: usize, dot: impl ToString) -> Cow<'static, ParameterNameRef> {
        Cow::Owned(
            format!(
                "{STRIP}[{}].EQ.channel[{}].cell[{}].{}",
                self.strip_index,
                self.channel,
                cell,
                dot.to_string()
            )
            .into(),
        )
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
    pub fn gain(&self, cell: usize) -> IntParameter {
        // TODO: Enum Parameter
        IntParameter::new(self.param(cell, "gain"), self.remote, -12..=12)
    }
    /// Quality of the EQ filter.
    pub fn q(&self, cell: usize) -> IntParameter {
        // TODO: Enum Parameter
        IntParameter::new(self.param(cell, "q"), self.remote, 1..=100)
    }
}

/// Bus device parameters
pub struct BusDevice<'a> {
    remote: &'a VoicemeeterRemote,
    strip_index: ZIndex,
}

impl<'a> BusDevice<'a> {
    #[doc(hidden)]
    pub fn new(remote: &'a VoicemeeterRemote, strip_index: ZIndex) -> Self {
        Self {
            remote,
            strip_index,
        }
    }

    fn param(&self, dot: impl ToString) -> Cow<'static, ParameterNameRef> {
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
