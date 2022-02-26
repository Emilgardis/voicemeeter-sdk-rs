use crate::VoicemeeterRemote;

impl VoicemeeterRemote {
    pub fn audio_callback_start(&self) -> Result<(), AudioCallbackStartError> {
        let res = unsafe { self.raw.VBVMR_AudioCallbackStart() };
        match res {
            0 => Ok(()),
            -1 => Err(AudioCallbackStartError::NoServer),
            1 => Err(AudioCallbackStartError::NoCallbackRegistered),
            s => Err(AudioCallbackStartError::Unexpected(s)),
        }
    }

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

#[derive(Debug, Clone, thiserror::Error)]
pub enum AudioCallbackStartError {
    // TODO: is this correct?
    #[error("no server")]
    NoServer,
    #[error("no callback registered")]
    NoCallbackRegistered,
    #[error("an unexpected error occurred")]
    Unexpected(i32),
}
#[derive(Debug, Clone, thiserror::Error)]
pub enum AudioCallbackStopError {
    // TODO: is this correct?
    #[error("no server")]
    NoServer,
    #[error("no callback registered")]
    NoCallbackRegistered,
    #[error("an unexpected error occurred")]
    Unexpected(i32),
}
