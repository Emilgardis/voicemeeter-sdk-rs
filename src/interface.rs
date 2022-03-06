//! The interface for Voicemeeter remote.-
use crate::{types::VoicemeeterApplication, LoadError};

use self::{
    communication_login_logout::LoginError, general_information::GetVoicemeeterInformationError,
};

pub mod callback;
pub mod communication_login_logout;
pub mod device;
pub mod general_information;
pub mod get_levels;
pub mod get_parameters;
pub mod macro_buttons;
pub mod set_parameters;

/// Interface for voicemeeter.
#[derive(Clone)]
pub struct VoicemeeterRemote {
    raw: &'static crate::bindings::VoicemeeterRemoteRaw,
    program: VoicemeeterApplication,
}

impl std::fmt::Debug for VoicemeeterRemote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.program)
    }
}

impl VoicemeeterRemote {
    /// Creates a new [`VoicemeeterRemote`] instance that is logged in with the client.
    #[tracing::instrument]
    pub fn new() -> Result<Self, InitializationError> {
        let raw = crate::get_voicemeeter_raw()?;
        let mut s = VoicemeeterRemote::from_raw(raw);
        s.login()?;
        s.program = s.get_voicemeeter_type()?;
        Ok(s)
    }

    fn from_raw(raw: &'static crate::VoicemeeterRemoteRaw) -> VoicemeeterRemote {
        Self {
            raw,
            program: VoicemeeterApplication::Other,
        }
    }
}

impl Drop for VoicemeeterRemote {
    fn drop(&mut self) {
        let _ = self._logout();
    }
}

/// Errors that can occur when initializing the Voicemeeter remote DLL.
#[derive(Debug, thiserror::Error)]
pub enum InitializationError {
    /// Error while loading the DLL.
    #[error("could not load the client")]
    LoadError(#[from] LoadError),
    /// Error when logging in.
    #[error("could not login")]
    LoginError(#[from] LoginError),
    /// Error when getting the Voicemeeter application type.
    #[error("could not get voicemeeter type")]
    InformationError(#[from] GetVoicemeeterInformationError),
}
