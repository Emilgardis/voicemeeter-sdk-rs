//! Functions and data types for setting parameter values.
use std::ffi::CString;

use crate::types::ParameterNameRef;

use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    // TODO: Provide abstraction, e.g ParameterThing::Struct.blabla()
    /// Set the float value of a parameter.
    pub fn set_parameter_float(
        &self,
        param: &ParameterNameRef,
        value: f32,
    ) -> Result<(), SetParameterError> {
        let param = CString::new(param.as_ref()).unwrap();
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
    // TODO: Provide abstraction, e.g ParameterThing::Struct.blabla()
    /// Set the string value of a parameter.
    pub fn set_parameter_string(
        &self,
        param: impl AsRef<str>,
        value: &str,
    ) -> Result<(), SetParameterError> {
        let param = CString::new(param.as_ref()).unwrap();
        let value = CString::new(value).unwrap();
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
