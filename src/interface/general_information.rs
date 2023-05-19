//! General information about the running Voicemeeter instance.
//!
//! # Functions
//!
//! * [`get_voicemeeter_type`](VoicemeeterRemote::get_voicemeeter_type)
//! * [`get_voicemeeter_version`](VoicemeeterRemote::get_voicemeeter_version)
use crate::types::VoicemeeterApplication;

use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    /// Get the application type of the running Voicemeeter instance.
    pub fn get_voicemeeter_type(
        &self,
    ) -> Result<VoicemeeterApplication, GetVoicemeeterInformationError> {
        let mut t = 0i32;
        let res = unsafe { self.raw.VBVMR_GetVoicemeeterType(&mut t) };
        match res {
            0 => Ok(VoicemeeterApplication::from(t)),
            -1 => Err(GetVoicemeeterInformationError::CannotGetClient),
            -2 => Err(GetVoicemeeterInformationError::NoServer),
            s => Err(GetVoicemeeterInformationError::Other(s)),
        }
    }
    /// Get the version of the running Voicemeeter instance.
    pub fn get_voicemeeter_version(
        &self,
    ) -> Result<VoicemeeterVersion, GetVoicemeeterInformationError> {
        let mut t = 0i32;
        let res = unsafe { self.raw.VBVMR_GetVoicemeeterVersion(&mut t) };
        match res {
            0 => {
                let a: [u8; 4] = t.to_be_bytes();
                Ok(VoicemeeterVersion(a[0], a[1], a[2], a[3]))
            }
            -1 => Err(GetVoicemeeterInformationError::CannotGetClient),
            -2 => Err(GetVoicemeeterInformationError::NoServer),
            s => Err(GetVoicemeeterInformationError::Other(s)),
        }
    }
}

/// Version of the Voicemeeter instance.
#[derive(Debug, Clone)]
pub struct VoicemeeterVersion(pub u8, pub u8, pub u8, pub u8);

impl std::fmt::Display for VoicemeeterVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

/// Errors that can happen when querying information from Voicemeeter.
#[derive(Debug, thiserror::Error, Clone)]
pub enum GetVoicemeeterInformationError {
    /// Cannot get client.
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    /// No server found.
    #[error("no server")]
    NoServer,
    /// An unexpected error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
    /// Got an unexcepted response.
    #[error("got an unexpected response")]
    InvalidResponse(String),
}
