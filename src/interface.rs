use crate::LoadError;

use self::communication_login_logout::LoginError;

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
}

impl VoicemeeterRemote {
    /// Creates a new [`VoicemeeterRemote`] instance that is logged in with the client.
    pub fn new() -> Result<Self, InitializationError> {
        let raw = crate::get_voicemeeter_raw()?;
        let mut s = VoicemeeterRemote::from_raw(raw);
        s.login()?;
        Ok(s)
    }

    fn from_raw(raw: &'static crate::VoicemeeterRemoteRaw) -> VoicemeeterRemote {
        Self { raw }
    }
}

impl Drop for VoicemeeterRemote {
    fn drop(&mut self) {
        self._logout();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InitializationError {
    #[error("could not load the client")]
    LoadError(#[from] LoadError),
    #[error("could not login")]
    LoginError(#[from] LoginError),
}
