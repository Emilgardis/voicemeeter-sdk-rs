//! Start stop functions for voicemeeter audio callback
use crate::VoicemeeterRemote;

impl VoicemeeterRemote {
    /// Audio callback start.
    pub fn audio_callback_start(&self) -> Result<(), AudioCallbackStartError> {
        let res = unsafe { self.raw.VBVMR_AudioCallbackStart() };
        match res {
            0 => Ok(()),
            -1 => Err(AudioCallbackStartError::NoServer),
            1 => Err(AudioCallbackStartError::NoCallbackRegistered),
            s => Err(AudioCallbackStartError::Unexpected(s)),
        }
    }

    /// Audio callback stop.
    pub fn audio_callback_stop(&self) -> Result<(), AudioCallbackStopError> {
        let res = unsafe { self.raw.VBVMR_AudioCallbackStop() };
        match res {
            0 => Ok(()),
            -1 => Err(AudioCallbackStopError::NoServer),
            1 => Err(AudioCallbackStopError::NoCallbackRegistered),
            s => Err(AudioCallbackStopError::Unexpected(s)),
        }
    }
}

/// Errors that can occur while registering the audio callback.
#[derive(Debug, Clone, thiserror::Error)]
pub enum AudioCallbackStartError {
    // TODO: is this correct?
    /// No server.
    #[error("no server")]
    NoServer,
    /// No callback registered.
    #[error("no callback registered")]
    NoCallbackRegistered,
    /// An unknown error code occured.
    #[error("an unexpected error occurred: error code {0}")]
    Unexpected(i32),
}

/// Errors that can occur while unregistering the audio callback.
#[derive(Debug, Clone, thiserror::Error)]
pub enum AudioCallbackStopError {
    // TODO: is this correct?
    /// No server.
    #[error("no server")]
    NoServer,
    /// No callback registered.
    #[error("no callback registered")]
    NoCallbackRegistered,
    /// An unknown error code occured.
    #[error("an unexpected error occurred: error code {0}")]
    Unexpected(i32),
}
