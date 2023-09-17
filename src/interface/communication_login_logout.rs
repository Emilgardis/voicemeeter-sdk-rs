//! Communication with and to Voicemeeter
//!
//! # Functions
//! * [`initial_status`](VoicemeeterRemote::initial_status)
//! * `login` is implicitly called when creating your first [`VoicemeeterRemote`] instance.
//! * [`logout`](VoicemeeterRemote::logout)
//! * [`run_voicemeeter`](VoicemeeterRemote::run_voicemeeter)
use crate::types::VoicemeeterApplication;

use super::VoicemeeterRemote;

static HAS_LOGGED_IN: std::sync::OnceLock<VoicemeeterStatus> = std::sync::OnceLock::new();

impl VoicemeeterRemote {
    /// Get the status of the running Voicemeeter instance when we first logged in
    pub fn initial_status() -> VoicemeeterStatus {
        HAS_LOGGED_IN.get().unwrap().clone()
    }
    pub(crate) fn login(&mut self) -> Result<VoicemeeterStatus, LoginError> {
        if let Some(res) = HAS_LOGGED_IN.get() {
            return Err(LoginError::AlreadyLoggedIn(res.clone()));
        }
        let res = unsafe { self.raw.VBVMR_Login() };
        let res = match res {
            0 => Ok(VoicemeeterStatus::Launched),
            1 => Ok(VoicemeeterStatus::NotLaunched),
            -2 => Err(LoginError::LoginFailed),
            s => Err(LoginError::Unexpected(s)),
        }?;
        tracing::debug!("logged in with status {:?}", res);
        Ok(HAS_LOGGED_IN.get_or_init(|| res).clone())
    }
    /// Logout from the voicemeeter instance. This should only be called when you never need another VoiceMeeter remote again.
    ///
    /// # Notes
    ///
    /// [`VoicemeeterRemote::new`] will automatically login if needed.
    pub fn logout(self) -> Result<(), LogoutError> {
        drop(self);
        if super::LOGOUT_HANDLE
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .is_some()
        {
            Err(LogoutError::OtherRemotesExists)
        } else {
            Ok(())
        }
    }

    pub(crate) fn _logout(&mut self) -> Result<(), LogoutError> {
        let _ = self.logout_handle.take();
        // TODO: use Option::take_if ?
        let Some(mut a) = super::LOGOUT_HANDLE.get().unwrap().lock().unwrap().take() else {
            return Ok(());
        };
        if let Some(logged_out) = std::sync::Arc::get_mut(&mut a) {
            if *logged_out {
                return Ok(());
            }
            tracing::debug!("logging out");
            let res = unsafe { self.raw.VBVMR_Logout() };
            match res {
                0 => {
                    *logged_out = true;
                    Ok(())
                }
                s => Err(LogoutError::Unexpected(s)),
            }
        } else {
            super::LOGOUT_HANDLE
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .replace(a);
            Err(LogoutError::OtherRemotesExists)
        }
    }

    /// Invoke Voicemeeter to open and be visible on previous location.
    pub fn run_voicemeeter(
        &self,
        r#type: VoicemeeterApplication,
    ) -> Result<(), RunVoicemeeterError> {
        let res = unsafe { self.raw.VBVMR_RunVoicemeeter(r#type as i32) };
        match res {
            0 => Ok(()),
            -1 => Err(RunVoicemeeterError::NotInstalled),
            -2 => Err(RunVoicemeeterError::UnknownType),
            s => Err(RunVoicemeeterError::Other(s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The status of the Voicemeeter instance.
pub enum VoicemeeterStatus {
    /// Voicemeeter is launched.
    Launched,
    /// Voicemeeter is not launched.
    NotLaunched,
}

/// Errors that can happen when logging in.
#[derive(Debug, thiserror::Error, Clone)]
#[non_exhaustive]
pub enum LoginError {
    /// Application has already logged in
    #[error("application has already logged in")]
    AlreadyLoggedIn(VoicemeeterStatus),
    /// The login failed.
    #[error("unexpected login (logout was expected before)")]
    LoginFailed,
    /// An unexpected error occured.
    #[error("cannot get client (unexpected): {0}")]
    Unexpected(i32),
}

/// Errors that can happen when loging out.
#[derive(Debug, thiserror::Error, Clone)]
#[non_exhaustive]
pub enum LogoutError {
    /// Couldn't logout due to other [VoicemeeterRemote]s existing in this program
    #[error("couldn't logout due to other `VoicemeeterRemote`s existing in this program")]
    OtherRemotesExists,
    /// An unexpected error occured.
    #[error("cannot get client (unexpected): {0}")]
    Unexpected(i32),
}
/// Errors that can happen when [opening](VoicemeeterRemote::run_voicemeeter) voicemeeter.
#[derive(Debug, thiserror::Error, Clone)]
#[non_exhaustive]
pub enum RunVoicemeeterError {
    /// Voicemeeter is not installed.
    #[error("voicemeeter is not installed")]
    NotInstalled,
    /// Unknown voicemeeter type.
    #[error("unknown type")]
    UnknownType,
    /// An unexpected error occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

impl LoginError {
    /// Returns `true` if the login error is [`LoginFailed`].
    ///
    /// [`LoginFailed`]: LoginError::LoginFailed
    pub fn is_login_failed(&self) -> bool {
        matches!(self, Self::LoginFailed)
    }

    /// Returns `true` if the login error is [`Unexpected`].
    ///
    /// [`Unexpected`]: LoginError::Unexpected
    pub fn is_unexpected(&self) -> bool {
        matches!(self, Self::Unexpected(..))
    }
}
