//! Parameter related functions and types
use std::{
    ffi::{CStr, CString, NulError},
    os::raw::c_char,
    ptr,
};

use crate::types::ParameterNameRef;

use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    // TODO: Only call from one thread, limit it
    /// Check if parameters have changed
    ///
    /// Call this function periodically to check if parameters have changed, typically every 10ms.
    ///
    /// # Security
    ///
    /// This method must only be called from one thread.
    pub fn is_parameters_dirty(&self) -> Result<bool, IsParametersDirtyError> {
        let res = unsafe { self.raw.VBVMR_IsParametersDirty() };
        match res {
            0 => Ok(false),
            1 => Ok(true),
            -1 => Err(IsParametersDirtyError::CannotGetClient),
            -2 => Err(IsParametersDirtyError::NoServer),
            s => Err(IsParametersDirtyError::Other(s)),
        }
    }
    // TODO: Provide abstraction, e.g ParameterThing::Struct.blabla()
    /// Get the float value of a parameter.
    pub fn get_parameter_float(&self, param: &ParameterNameRef) -> Result<f32, GetParameterError> {
        let mut f = 0.0f32;
        let param = CString::new(param.as_ref())?;
        let res = unsafe {
            self.raw
                .VBVMR_GetParameterFloat(param.as_ptr() as *mut _, &mut f)
        };
        match res {
            0 => Ok(f),
            -1 => Err(GetParameterError::CannotGetClient),
            -2 => Err(GetParameterError::NoServer),
            -3 => Err(GetParameterError::UnknownParameter(
                param.to_string_lossy().into_owned(),
            )), // NOTE: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            -5 => Err(GetParameterError::StructureMismatch(
                param.to_string_lossy().into_owned(),
                "float",
            )),
            s => Err(GetParameterError::Other(s)),
        }
    }
    // TODO: Provide abstraction, e.g ParameterThing::Struct.blabla()
    /// Get the string value of a parameter.
    pub fn get_parameter_string(
        &self,
        param: impl AsRef<str>,
    ) -> Result<String, GetParameterError> {
        let param = CString::new(param.as_ref()).unwrap();
        let mut output = [0 as c_char; 512];
        let res = unsafe {
            self.raw
                .VBVMR_GetParameterStringA(param.as_ptr() as *mut _, ptr::addr_of_mut!(output[0]))
        };
        match res {
            0 => {
                let output = unsafe { CStr::from_ptr(ptr::addr_of!(output[0])) }
                    .to_string_lossy()
                    .into_owned();
                Ok(output)
            }
            -1 => Err(GetParameterError::CannotGetClient),
            -2 => Err(GetParameterError::NoServer),
            -3 => Err(GetParameterError::UnknownParameter(
                param.to_string_lossy().into_owned(),
            )), // NOTE: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            -5 => Err(GetParameterError::StructureMismatch(
                param.to_string_lossy().into_owned(),
                "float",
            )),
            s => Err(GetParameterError::Other(s)),
        }
    }
}

/// Errors that can happen when getting a parameter.
#[derive(Debug, thiserror::Error, Clone)]
pub enum GetParameterError {
    /// Could not make a c-compatible string. This is a bug.
    #[error("could not make into a c-string")]
    NulError(#[from] NulError),
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
    /// Structure mismatch.
    #[error("tried to parse parameter {0:?} as a {1} but it is not")]
    StructureMismatch(String, &'static str),
    /// An unknown error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

/// Errors that can happen when querying parameter "dirty" flag.
#[derive(Debug, thiserror::Error, Clone)]
pub enum IsParametersDirtyError {
    // TODO: is this correct? docs say "error (unexpected)""
    /// Cannot get client.
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    /// No server.
    #[error("no server")]
    NoServer,
    /// An unknown error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}
