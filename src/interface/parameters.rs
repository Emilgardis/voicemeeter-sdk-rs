//! Functions for setting and getting parameter values.
//!
//! Note that if your application exits quickly after setting a parameter, voicemeeter may not update.
//! If you do this, you should maybe insert a small sleep before dropping the remote.'
// FIXME: file above as an issue upstream, is this an issue?
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Bound, RangeBounds};

use crate::types::{BusMode, ParameterNameRef, VoicemeeterApplication, ZIndex};
use crate::VoicemeeterRemote;

mod errors;

pub mod bus;
pub mod get_parameters;
pub mod option;
pub mod set_parameters;
pub mod strip;

pub use bus::*;
pub use errors::*;
pub use option::*;
pub use strip::*;

use self::get_parameters::GetParameterError;
use self::set_parameters::SetParameterError;

impl VoicemeeterRemote {
    /// Abstraction for voicemeeter parameters,
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use voicemeeter::VoicemeeterRemote;
    ///
    /// # let remote: VoicemeeterRemote = todo!();
    /// println!(
    ///     "Strip 0: {}",
    ///     remote.parameters().strip(0)?.device()?.name().get()?
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parameters(&self) -> Parameters {
        Parameters { remote: self }
    }
}

/// A float parameter
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
            _pd: <_>::default(),
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

/// Parameter abstraction
///
/// # Examples
///
/// ```rust,no_run
/// # use voicemeeter::VoicemeeterRemote;
/// # let remote: VoicemeeterRemote = todo!();
/// println!(
///     "Strip 0: {}",
///     remote.parameters().strip(0)?.device()?.name().get()?
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Parameters<'a> {
    remote: &'a VoicemeeterRemote,
}

// TODO: Add `recorder`, `patch` and `vban`
impl<'a> Parameters<'a> {
    /// Parameters of a strip.
    ///
    /// A strip is a input, "physical" or "virtual"
    pub fn strip(&self, index: impl Into<ZIndex>) -> Result<Strip<'a>, OutOfRangeError> {
        let index = index.into();
        Ok(match (self.remote.program, index.0) {
            (VoicemeeterApplication::Voicemeeter, 0..=2) => Strip::new(self.remote, index),
            (VoicemeeterApplication::VoicemeeterBanana, 0..=4) => Strip::new(self.remote, index),
            (VoicemeeterApplication::VoicemeeterPotato, 0..=7)
            | (VoicemeeterApplication::PotatoX64Bits, 0..=7) => Strip::new(self.remote, index),
            _ => return Err(OutOfRangeError { name: STRIP, index }),
        })
    }

    /// Parameters of a bus.
    ///
    /// A bus is a output.
    pub fn bus(&self, index: impl Into<ZIndex>) -> Result<Bus<'a>, OutOfRangeError> {
        let index = index.into();
        Ok(match (self.remote.program, index.0) {
            (VoicemeeterApplication::Voicemeeter, 0..=1) => Bus::new(self.remote, index),
            (VoicemeeterApplication::VoicemeeterBanana, 0..=4) => Bus::new(self.remote, index),
            (VoicemeeterApplication::VoicemeeterPotato, 0..=7)
            | (VoicemeeterApplication::PotatoX64Bits, 0..=7) => Bus::new(self.remote, index),
            _ => return Err(OutOfRangeError { name: BUS, index }),
        })
    }

    /// Options for Voicemeeter
    pub fn option(&self) -> VoicemeeterOption<'a> {
        VoicemeeterOption::new(self.remote)
    }
}
