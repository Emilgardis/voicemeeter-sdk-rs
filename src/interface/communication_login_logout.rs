//! Communication with and to Voicemeeter
//!
//! # Functions
//!
//! * [`logout`](VoicemeeterRemote::logout)
//! * [`run_voicemeeter`](VoicemeeterRemote::run_voicemeeter)
use crate::types::VoicemeeterApplication;

use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    pub(crate) fn login(&mut self) -> Result<VoicemeeterStatus, LoginError> {
        let res = unsafe { self.raw.VBVMR_Login() };
        match res {
            0 => Ok(VoicemeeterStatus::Launched),
            1 => Ok(VoicemeeterStatus::NotLaunched),
            -2 => Err(LoginError::LoginFailed),
            s => Err(LoginError::Unexpected(s)),
        }
    }
    /// Logout from the voicemeeter instance.
    ///
    /// # Notes
    ///
    /// [`VoicemeeterRemote::new`] will automatically login.
    pub fn logout(mut self) -> Result<(), LogoutError> {
        self._logout()?;
        Ok(())
    }

    pub(crate) fn _logout(&mut self) -> Result<(), LogoutError> {
        let res = unsafe { self.raw.VBVMR_Logout() };
        match res {
            0 => Ok(()),
            s => Err(LogoutError::Unexpected(s)),
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

/// Errors that can happen when loging in.
#[derive(Debug, thiserror::Error, Clone)]
pub enum LoginError {
    /// The login failed.
    #[error("unexpected login (logout was expected before)")]
    LoginFailed,
    /// An unexpected error occured.
    #[error("cannot get client (unexpected): {0}")]
    Unexpected(i32),
}

/// Errors that can happen when loging out.
#[derive(Debug, thiserror::Error, Clone)]
pub enum LogoutError {
    /// An unexpected error occured.
    #[error("cannot get client (unexpected): {0}")]
    Unexpected(i32),
}
/// Errors that can happen when [opening](VoicemeeterRemote::run_voicemeeter) voicemeeter.
#[derive(Debug, thiserror::Error, Clone)]
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
