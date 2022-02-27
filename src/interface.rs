use crate::{LoadError, types::VoicemeeterApplication};

use self::{communication_login_logout::LoginError, general_information::GetVoicemeeterInformationError};

pub mod callback;
pub mod communication_login_logout;
pub mod device;
pub mod general_information;
pub mod get_levels;
pub mod get_parameters;
pub mod macro_buttons;
pub mod set_parameters;

#[derive(Clone)]
pub struct VoicemeeterRemote {
    raw: &'static crate::bindings::VoicemeeterRemoteRaw,
    program: VoicemeeterApplication,
}

impl VoicemeeterRemote {
    /// Creates a new [`VoicemeeterRemote`] instance that is logged in with the client.
    pub fn new() -> Result<Self, InitializationError> {
        let raw = crate::get_voicemeeter_raw()?;
        let mut s = VoicemeeterRemote::from_raw(raw);
        s.login()?;
        s.program = s.get_voicemeeter_type()?;
        Ok(s)
    }

    fn from_raw(raw: &'static crate::VoicemeeterRemoteRaw) -> VoicemeeterRemote {
        Self { raw, program: VoicemeeterApplication::Other }
    }
}

impl Drop for VoicemeeterRemote {
    fn drop(&mut self) {
        let _ = self._logout();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InitializationError {
    #[error("could not load the client")]
    LoadError(#[from] LoadError),
    #[error("could not login")]
    LoginError(#[from] LoginError),
    #[error("could not get voicemeeter type")]
    InformationError(#[from] GetVoicemeeterInformationError),
}
