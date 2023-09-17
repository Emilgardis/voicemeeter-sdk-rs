//! The interface for Voicemeeter remote.
//!
//! See the methods on [`VoicemeeterRemote`] for how to use the interface
use crate::{types::VoicemeeterApplication, LoadError};

use self::{
    communication_login_logout::LoginError, general_information::GetVoicemeeterInformationError,
};

pub mod callback;
pub mod communication_login_logout;
pub mod device;
pub mod general_information;
pub mod get_levels;
pub mod macro_buttons;
pub mod parameters;

/// Interface for voicemeeter.
#[derive(Clone)]
#[cfg(feature = "interface")] // for doc_cfg
pub struct VoicemeeterRemote {
    raw: &'static crate::bindings::VoicemeeterRemoteRaw,
    logout_handle: Option<std::sync::Arc<bool>>,
    /// The type of the running Voicemeeter instance.
    pub program: VoicemeeterApplication,
}

pub(crate) static LOGOUT_HANDLE: std::sync::OnceLock<
    std::sync::Mutex<Option<std::sync::Arc<bool>>>,
> = std::sync::OnceLock::new();

impl std::fmt::Debug for VoicemeeterRemote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.program {
            VoicemeeterApplication::None => write!(f, "[no voicemeeter program running]"),
            p => write!(f, "[{}]", p),
        }
    }
}

impl VoicemeeterRemote {
    /// Creates a new [`VoicemeeterRemote`] instance that is logged in with the client.
    ///
    /// # Notes
    ///
    /// If you create [`VoicemeeterRemote`] instances, when the last instance remaining is dropped,
    /// you will automatically be logged out of the API, meaning you cannot login again.
    ///
    /// This is done to prevent leaving the API logged in when your program is closed,
    /// causing a visual bug in the voicemeeter application.
    #[tracing::instrument]
    pub fn new() -> Result<Self, InitializationError> {
        let raw = crate::get_voicemeeter_raw()?;
        let mut s = VoicemeeterRemote::from_raw(raw);
        if s.logout_handle.is_none() {
            return Err(InitializationError::AlreadyLoggedOut);
        }
        match s.login() {
            Ok(_) => {}
            Err(LoginError::AlreadyLoggedIn(_)) => {}
            e => {
                e?;
            }
        };
        s.update_program()?;
        Ok(s)
    }

    fn from_raw(raw: &'static crate::VoicemeeterRemoteRaw) -> VoicemeeterRemote {
        Self {
            raw,
            program: VoicemeeterApplication::Other,
            logout_handle: LOGOUT_HANDLE
                .get_or_init(|| std::sync::Mutex::new(Some(std::sync::Arc::new(false))))
                .lock()
                .unwrap()
                .clone(),
        }
    }

    /// Update the current program type.
    pub fn update_program(&mut self) -> Result<(), GetVoicemeeterInformationError> {
        match self.get_voicemeeter_type() {
            Ok(t) => self.program = t,
            Err(GetVoicemeeterInformationError::NoServer) => {
                self.program = VoicemeeterApplication::None
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

impl Drop for VoicemeeterRemote {
    fn drop(&mut self) {
        // This logout only happens if this is the only voicemeeter handle that exists.
        let _ = self._logout();
    }
}

/// Errors that can occur when initializing the Voicemeeter remote DLL.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InitializationError {
    /// Error while loading the DLL.
    #[error("could not load the client")]
    LoadError(#[from] LoadError),
    /// Error when logging in.
    #[error("could not login")]
    LoginError(#[from] LoginError),
    /// Application has already logged out.
    #[error("application has already logged out, so cannot login again")]
    AlreadyLoggedOut,
    /// Error when getting the Voicemeeter application type.
    #[error("could not get voicemeeter type")]
    InformationError(#[from] GetVoicemeeterInformationError),
}
