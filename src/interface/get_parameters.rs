use std::{
    ffi::{CStr, CString, NulError},
    os::raw::c_char,
    ptr,
};



use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    // FIXME: Only call from one thread, limit it
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
    // FIXME: Prefer using abstraction [linkme]
    pub fn get_parameter_float(&self, param: impl AsRef<str>) -> Result<f32, GetParameterError> {
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
            )), // FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            -5 => Err(GetParameterError::StructureMismatch(
                param.to_string_lossy().into_owned(),
                "float",
            )),
            s => Err(GetParameterError::Other(s)),
        }
    }
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
            )), // FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            -5 => Err(GetParameterError::StructureMismatch(
                param.to_string_lossy().into_owned(),
                "float",
            )),
            s => Err(GetParameterError::Other(s)),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum GetParameterError {
    #[error("could not make into a c-string")]
    NulError(#[from] NulError),
    // TODO: is this correct? docs say "error (unexpected)""
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    #[error("no server")]
    NoServer,
    #[error("unknown parameter: {0}")]
    UnknownParameter(String),
    #[error("tried to parse parameter {0:?} as a {1} but it is not")]
    StructureMismatch(String, &'static str),
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum IsParametersDirtyError {
    // TODO: is this correct? docs say "error (unexpected)""
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    #[error("no server")]
    NoServer,
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}
