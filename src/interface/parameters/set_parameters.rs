//! Functions and data types for setting parameter values.
//!
//! # Functions
//!
//! * [`set_parameter_float`](VoicemeeterRemote::set_parameter_float)
//! * [`set_parameter_string`](VoicemeeterRemote::set_parameter_string)
//! * [`set_parameters`](VoicemeeterRemote::set_parameters)
use std::ffi::CString;

use crate::types::ParameterNameRef;

use crate::VoicemeeterRemote;

impl VoicemeeterRemote {
    /// Set the float value of a parameter. See also [`VoicemeeterRemote::parameters()`] to do this with functions.
    #[tracing::instrument(skip(self))]
    pub fn set_parameter_float(
        &self,
        param: &ParameterNameRef,
        value: f32,
    ) -> Result<(), SetParameterError> {
        let param = CString::new(param.as_ref()).unwrap();
        tracing::debug!("setting float parameter");
        let res = unsafe {
            self.raw
                .VBVMR_SetParameterFloat(param.as_ptr() as *mut _, value)
        };
        match res {
            0 => Ok(()),
            -1 => Err(SetParameterError::CannotGetClient),
            -2 => Err(SetParameterError::NoServer),
            -3 => Err(SetParameterError::UnknownParameter(
                param.to_string_lossy().into_owned(),
            )), // NOTE: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            s => Err(SetParameterError::Other(s)),
        }
    }

    /// Set the string value of a parameter. See also [`VoicemeeterRemote::parameters()`] to do this with functions.
    #[tracing::instrument(skip(self))]
    pub fn set_parameter_string(
        &self,
        param: &ParameterNameRef,
        value: &str,
    ) -> Result<(), SetParameterError> {
        let param = CString::new(param.as_ref()).unwrap();
        let value = CString::new(value).unwrap();
        tracing::debug!("setting string parameter");
        let res = unsafe {
            self.raw
                .VBVMR_SetParameterStringA(param.as_ptr() as *mut _, value.as_ptr() as *mut _)
        };
        match res {
            0 => Ok(()),
            -1 => Err(SetParameterError::CannotGetClient),
            -2 => Err(SetParameterError::NoServer),
            -3 => Err(SetParameterError::UnknownParameter(
                param.to_string_lossy().into_owned(),
            )), // NOTE: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            s => Err(SetParameterError::Other(s)),
        }
    }

    // TODO: Example script.
    /// Set parameters using a script. Similar to macro button scripts.
    pub fn set_parameters(&self, script: &str) -> Result<(), SetParametersError> {
        let script = CString::new(script).unwrap();
        let res = unsafe { self.raw.VBVMR_SetParameters(script.as_ptr() as *mut _) };

        match res {
            l if l > 0 => Err(SetParametersError::ScriptError(l as usize)),
            0 => Ok(()),
            -1 => Err(SetParameterError::CannotGetClient.into()),
            -2 => Err(SetParameterError::NoServer.into()),
            s => Err(SetParameterError::Other(s).into()),
        }
    }
}

/// Errors that can happen when setting parameters.
#[derive(Debug, thiserror::Error, Clone)]
#[non_exhaustive]
pub enum SetParametersError {
    /// Script error
    #[error("script error on line: {0}")]
    ScriptError(usize),
    /// An error occured when setting parameters.
    #[error(transparent)]
    SetParameterError(#[from] SetParameterError),
}

/// Errors that can happen when setting a parameter.
#[derive(Debug, thiserror::Error, Clone)]
#[non_exhaustive]
pub enum SetParameterError {
    // TODO: is this correct? docs say "error (unexpected)""
    /// Cannot get client.
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    /// No server.
    #[error("no server")]
    NoServer,
    /// Unknown parameter.
    #[error("unknown parameter: {0}")]
    UnknownParameter(String),
    /// An unknown error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}
