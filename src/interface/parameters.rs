//! Functions for setting and getting parameter values.
//!
//! Note that if your application exits quickly after setting a parameter, voicemeeter may not update.
//! If you do this, you should maybe insert a small sleep before dropping the remote.'
//!
//! # Functions
//!
//! * [`parameters`](VoicemeeterRemote::parameters)

// FIXME: file above as an issue upstream, is this an issue?
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Bound, RangeBounds};

use crate::types::{BusMode, Device, ParameterNameRef, VoicemeeterApplication, ZIndex};
use crate::VoicemeeterRemote;

mod errors;

pub mod bus;
pub mod eq;
pub mod fx;
pub mod get_parameters;
pub mod option;
pub mod recorder;
pub mod set_parameters;
pub mod strip;
pub mod vban;

pub use bus::*;
pub use eq::*;
pub use errors::*;
pub use fx::*;
pub use option::*;
pub use recorder::*;
pub use strip::*;
pub use vban::*;

use self::get_parameters::GetParameterError;
use self::set_parameters::SetParameterError;

pub(crate) static BUS: &str = "Bus";
pub(crate) static FX: &str = "Recorder";
pub(crate) static RECORDER: &str = "Recorder";
pub(crate) static STRIP: &str = "Strip";
pub(crate) static VOICEMEETER_OPTION: &str = "Option";
pub(crate) static VBAN: &str = "vban";

impl VoicemeeterRemote {
    /// Get access to [parameters](Parameters) of the application.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use voicemeeter::VoicemeeterRemote;
    ///
    /// # let remote: VoicemeeterRemote = todo!();
    /// println!("Strip 1: {}", remote.parameters().strip(0)?.label().get()?);
    ///
    /// println!(
    ///     "Bus 0 (A1) Device: {}",
    ///     remote.parameters().bus(0)?.device().name().get()?
    /// );
    ///
    /// // Enable A1 on strip 1
    /// remote.parameters().strip(0)?.a1().set(true)?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parameters(&self) -> Parameters {
        Parameters { remote: self }
    }
}

/// A float parameter
#[must_use = "set or get the value of the parameter"]
pub struct FloatParameter<'a, const WRITE: bool = true, const READ: bool = true> {
    /// The name of the parameter, fully qualified
    pub name: Cow<'a, ParameterNameRef>,

    remote: &'a VoicemeeterRemote,
    _range: Option<(Bound<f32>, Bound<f32>)>,
}

impl<'a, const WRITE: bool, const READ: bool> FloatParameter<'a, WRITE, READ> {
    fn new<R>(name: Cow<'a, ParameterNameRef>, remote: &'a VoicemeeterRemote, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        Self {
            name,
            remote,
            _range: Some((range.start_bound().cloned(), range.end_bound().cloned())),
        }
    }
    fn new_unranged(name: Cow<'a, ParameterNameRef>, remote: &'a VoicemeeterRemote) -> Self {
        Self {
            name,
            remote,
            _range: None,
        }
    }
}

impl<'a, const READ: bool> FloatParameter<'a, true, READ> {
    /// Set the value of this parameter
    pub fn set(&self, val: f32) -> Result<(), SetParameterError> {
        self.remote.set_parameter_float(&self.name, val)
    }
}

impl<'a, const WRITE: bool> FloatParameter<'a, WRITE, true> {
    /// Get the value of this parameter
    pub fn get(&self) -> Result<f32, GetParameterError> {
        self.remote.get_parameter_float(&self.name)
    }
}

/// A string parameter
#[must_use = "set or get the value of the parameter"]
pub struct StringParameter<'a, const WRITE: bool = true, const READ: bool = true> {
    /// The name of the parameter, fully qualified
    pub name: Cow<'a, ParameterNameRef>,

    remote: &'a VoicemeeterRemote,
}

impl<'a, const WRITE: bool, const READ: bool> StringParameter<'a, WRITE, READ> {
    fn new(name: Cow<'a, ParameterNameRef>, remote: &'a VoicemeeterRemote) -> Self {
        Self { name, remote }
    }
}

impl<'a, const READ: bool> StringParameter<'a, true, READ> {
    /// Set the value of this parameter
    pub fn set(&self, val: &str) -> Result<(), SetParameterError> {
        self.remote.set_parameter_string(&self.name, val)
    }
}

impl<'a, const WRITE: bool> StringParameter<'a, WRITE, true> {
    /// Get the value of this parameter
    pub fn get(&self) -> Result<String, GetParameterError> {
        self.remote.get_parameter_string(&self.name)
    }
}

/// A tuple parameter
#[must_use = "set or get the value of the parameter"]
pub struct TupleParameter<'a, A, B, const WRITE: bool = true, const READ: bool = true> {
    /// The name of the parameter, fully qualified
    pub name: Cow<'a, ParameterNameRef>,

    remote: &'a VoicemeeterRemote,
    _pd: std::marker::PhantomData<(A, B)>,
}

impl<'a, A, B, const WRITE: bool, const READ: bool> TupleParameter<'a, A, B, WRITE, READ> {
    fn new(name: Cow<'a, ParameterNameRef>, remote: &'a VoicemeeterRemote) -> Self {
        Self {
            name,
            remote,
            _pd: Default::default(),
        }
    }
}

impl<'a, A, B, const READ: bool> TupleParameter<'a, A, B, true, READ>
where
    A: Debug,
    B: Debug,
{
    /// Set the value of this parameter
    pub fn set(&self, val_a: A, val_b: B) -> Result<(), SetParameterError> {
        self.remote
            .set_parameter_string(&self.name, &format!("({:?}, {:?})", val_a, val_b))
    }
}

/// A boolean parameter
#[must_use = "set or get the value of the parameter"]
pub struct BoolParameter<'a, const WRITE: bool = true, const READ: bool = true> {
    /// The name of the parameter, fully qualified
    pub name: Cow<'a, ParameterNameRef>,

    remote: &'a VoicemeeterRemote,
}

impl<'a, const WRITE: bool, const READ: bool> BoolParameter<'a, WRITE, READ> {
    fn new(name: Cow<'a, ParameterNameRef>, remote: &'a VoicemeeterRemote) -> Self {
        Self { name, remote }
    }
}

impl<'a, const READ: bool> BoolParameter<'a, true, READ> {
    /// Set the value of this parameter
    pub fn set(&self, val: bool) -> Result<(), SetParameterError> {
        self.remote
            .set_parameter_float(&self.name, if val { 1.0 } else { 0.0 })
    }
}

impl<'a, const WRITE: bool> BoolParameter<'a, WRITE, true> {
    /// Get the value of this parameter
    pub fn get(&self) -> Result<bool, GetParameterError> {
        Ok(self.remote.get_parameter_float(&self.name)? == 1.0)
    }
}

/// A integer parameter
#[must_use = "set or get the value of the parameter"]
pub struct IntParameter<'a, const WRITE: bool = true, const READ: bool = true> {
    /// The name of the parameter, fully qualified
    pub name: Cow<'a, ParameterNameRef>,

    remote: &'a VoicemeeterRemote,
    _range: Option<(Bound<i32>, Bound<i32>)>,
}

impl<'a, const WRITE: bool, const READ: bool> IntParameter<'a, WRITE, READ> {
    fn new<R>(name: Cow<'a, ParameterNameRef>, remote: &'a VoicemeeterRemote, range: R) -> Self
    where
        R: RangeBounds<i32>,
    {
        Self {
            name,
            remote,
            _range: Some((range.start_bound().cloned(), range.end_bound().cloned())),
        }
    }
    fn new_unranged(name: Cow<'a, ParameterNameRef>, remote: &'a VoicemeeterRemote) -> Self {
        Self {
            name,
            remote,
            _range: None,
        }
    }
}

impl<'a, const READ: bool> IntParameter<'a, true, READ> {
    /// Set the value of this parameter
    pub fn set(&self, val: i32) -> Result<(), SetParameterError> {
        self.remote.set_parameter_float(&self.name, val as f32)
    }
}

impl<'a, const WRITE: bool> IntParameter<'a, WRITE, true> {
    /// Get the value of this parameter
    pub fn get(&self) -> Result<i32, GetParameterError> {
        Ok(self.remote.get_parameter_float(&self.name)? as i32)
    }
}

/// Strip index helper
pub trait StripIndex {
    /// Get the strip index
    fn into_strip_index(self, program: &VoicemeeterApplication) -> Result<ZIndex, ParameterError>;
}

impl StripIndex for i32 {
    fn into_strip_index(self, _: &VoicemeeterApplication) -> Result<ZIndex, ParameterError> {
        Ok(ZIndex(self))
    }
}

impl StripIndex for usize {
    fn into_strip_index(self, _: &VoicemeeterApplication) -> Result<ZIndex, ParameterError> {
        Ok(ZIndex(self as _))
    }
}

impl StripIndex for ZIndex {
    fn into_strip_index(self, _: &VoicemeeterApplication) -> Result<ZIndex, ParameterError> {
        Ok(self)
    }
}

impl StripIndex for Device {
    fn into_strip_index(self, program: &VoicemeeterApplication) -> Result<ZIndex, ParameterError> {
        if !self.is_strip() {
            return Err(InvalidTypeError::ExpectedStrip {
                device: format!("{:?}", self),
            }
            .into());
        }
        self.as_strip_index(program).ok_or_else(|| {
            DeviceError {
                program: *program,
                device: self,
            }
            .into()
        })
    }
}

/// Bus index helper
pub trait BusIndex {
    /// Get the bus index
    fn into_bus_index(self, program: &VoicemeeterApplication) -> Result<ZIndex, ParameterError>;
}

impl BusIndex for i32 {
    fn into_bus_index(self, _: &VoicemeeterApplication) -> Result<ZIndex, ParameterError> {
        Ok(ZIndex(self))
    }
}

impl BusIndex for usize {
    fn into_bus_index(self, _: &VoicemeeterApplication) -> Result<ZIndex, ParameterError> {
        Ok(ZIndex(self as _))
    }
}

impl BusIndex for ZIndex {
    fn into_bus_index(self, _: &VoicemeeterApplication) -> Result<ZIndex, ParameterError> {
        Ok(self)
    }
}

impl BusIndex for Device {
    fn into_bus_index(self, program: &VoicemeeterApplication) -> Result<ZIndex, ParameterError> {
        if !self.is_bus() {
            return Err(InvalidTypeError::ExpectedBus {
                device: format!("{:?}", self),
            }
            .into());
        }
        self.as_bus_index(program)
            .ok_or_else(|| {
                DeviceError {
                    program: *program,
                    device: self,
                }
                .into()
            })
            .map(|i| i.0)
    }
}

/// Parameter abstraction
///
/// # Examples
///
/// ```rust,no_run
/// # use voicemeeter::VoicemeeterRemote;
/// # let remote: VoicemeeterRemote = todo!();
/// println!(
///     "Strip 1: {}",
///     remote.parameters().strip(0)?.device()?.name().get()?
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Parameters<'a> {
    remote: &'a VoicemeeterRemote,
}

// TODO: `patch`
impl<'a> Parameters<'a> {
    /// Parameters of a [strip](Strip).
    ///
    /// A strip is a input that can be physical or virtual
    ///
    ///
    /// # Availability
    ///
    /// On each Voicemeeter application, there are different amounts of strips
    ///
    /// Application | Strips | Physical | Virtual
    /// :--- | :--- | :--- | :---
    /// Voicemeeter | total: `3` | total: `2` _(starting on strip #0)_ | total: `1` _(starting on strip #2)_
    /// Voicemeeter Banana | total: `5` | total: `3` _(starting on strip #0)_ | total: `2` _(starting on strip #3)_
    /// Voicemeeter Potato | total: `8` | total: `5` _(starting on strip #0)_ | total: `3` _(starting on strip #5)_
    pub fn strip(&self, index: impl StripIndex) -> Result<Strip<'a>, ParameterError> {
        let index = index.into_strip_index(&self.remote.program)?;
        Ok(match (self.remote.program, index.0) {
            (VoicemeeterApplication::Voicemeeter, 0..=2) => Strip::new(self.remote, index),
            (VoicemeeterApplication::VoicemeeterBanana, 0..=4) => Strip::new(self.remote, index),
            (VoicemeeterApplication::VoicemeeterPotato, 0..=7)
            | (VoicemeeterApplication::PotatoX64Bits, 0..=7) => Strip::new(self.remote, index),
            _ => {
                return Err(Into::into(OutOfRangeError {
                    name: STRIP.to_owned(),
                    index,
                    program: self.remote.program,
                }));
            }
        })
    }

    /// Parameters of a [bus](Bus).
    ///
    /// A bus is a output. In the interface, these are called `A1`, `A2`, `A3`, `B1`, etc.
    ///
    /// # Availability
    ///
    /// On each Voicemeeter application, there are different amounts of busses
    ///
    /// Application | Busses | Physical | Virtual
    /// :--- | :--- | :--- | :---
    /// Voicemeeter | total: `2` | total: `2` _(starting on bus #0)_ | total: `0`
    /// Voicemeeter Banana | total: `5` | total: `3` _(starting on bus #0)_ | total: `2` _(starting on bus #3)_
    /// Voicemeeter Potato | total: `8` | total: `5` _(starting on bus #0)_ | total: `3` _(starting on bus #5)_
    pub fn bus(&self, index: impl BusIndex) -> Result<Bus<'a>, ParameterError> {
        let index = index.into_bus_index(&self.remote.program)?;
        Ok(match (self.remote.program, index.0) {
            (VoicemeeterApplication::Voicemeeter, 0..=1) => Bus::new(self.remote, index),
            (VoicemeeterApplication::VoicemeeterBanana, 0..=4) => Bus::new(self.remote, index),
            (VoicemeeterApplication::VoicemeeterPotato, 0..=7)
            | (VoicemeeterApplication::PotatoX64Bits, 0..=7) => Bus::new(self.remote, index),
            _ => {
                return Err(Into::into(OutOfRangeError {
                    name: BUS.to_owned(),
                    index,
                    program: self.remote.program,
                }));
            }
        })
    }

    /// Options for Voicemeeter
    pub fn option(&self) -> VoicemeeterOption<'a> {
        VoicemeeterOption::new(self.remote)
    }

    /// Voicemeeter recorder with playback
    pub fn recorder(&self) -> Result<VoicemeeterRecorder<'a>, ParameterError> {
        const VALID: &[VoicemeeterApplication] = &[
            VoicemeeterApplication::VoicemeeterBanana,
            VoicemeeterApplication::VoicemeeterPotato,
            VoicemeeterApplication::PotatoX64Bits,
        ];
        if !VALID.contains(&self.remote.program) {
            Err(ParameterError::Version(InvalidVoicemeeterVersion {
                expected: VALID,
                found: self.remote.program,
                parameter: RECORDER.to_owned(),
            }))
        } else {
            Ok(VoicemeeterRecorder::new(self.remote))
        }
    }

    /// Voicemeeter FX
    pub fn fx(&self) -> Result<VoicemeeterFx<'a>, ParameterError> {
        const VALID: &[VoicemeeterApplication] = &[
            VoicemeeterApplication::VoicemeeterPotato,
            VoicemeeterApplication::PotatoX64Bits,
        ];
        if !VALID.contains(&self.remote.program) {
            Err(ParameterError::Version(InvalidVoicemeeterVersion {
                expected: VALID,
                found: self.remote.program,
                parameter: FX.to_owned(),
            }))
        } else {
            Ok(VoicemeeterFx::new(self.remote))
        }
    }

    /// Voicemeeter VBAN
    pub fn vban(&self) -> VoicemeeterVban<'a> {
        VoicemeeterVban::new(self.remote)
    }
}
