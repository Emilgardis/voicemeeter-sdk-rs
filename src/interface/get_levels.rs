use std::ptr;

use crate::types::LevelType;

use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    // TODO: one thread only
    pub fn get_level(&self, level_type: LevelType, channel: i32) -> Result<f32, GetLevelError> {
        let mut f = std::f32::NAN;
        let res = unsafe { self.raw.VBVMR_GetLevel(level_type as i32, channel, &mut f) };
        match res {
            0 => Ok(f),
            -1 => Err(GetLevelError::CannotGetClient),
            -2 => Err(GetLevelError::NoServer),
            -3 => Err(GetLevelError::NoLevel),
            -4 => Err(GetLevelError::OutOfRange),
            s => Err(GetLevelError::Other(s)),
        }
    }

    // TODO: one thread only
    pub fn get_midi_message(&self) -> Result<Vec<u8>, GetMidiMessageError> {
        let mut v = vec![0; 1024];
        let len = self.get_midi_message_buff(&mut v)?;
        v.truncate(len);
        Ok(v)
    }

    // TODO: one thread only
    #[inline]
    pub fn get_midi_message_buff(&self, buffer: &mut [u8]) -> Result<usize, GetMidiMessageError> {
        let res = unsafe {
            self.raw
                .VBVMR_GetMidiMessage(ptr::addr_of_mut!(buffer[0]), buffer.len() as _)
        };
        match res {
            res if res >= 0 => Ok(res as usize),
            -1 => Err(GetMidiMessageError::CannotGetClient),
            -2 => Err(GetMidiMessageError::NoServer),
            v @ -5 | v @ -6 => Err(GetMidiMessageError::NoMidiData(v)),
            s => Err(GetMidiMessageError::Other(s)),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum GetLevelError {
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    #[error("no server")]
    NoServer,
    #[error("no level available")]
    NoLevel,
    #[error("out of range")]
    OutOfRange,
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum GetMidiMessageError {
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    #[error("no server")]
    NoServer,
    #[error("no level available")]
    NoMidiData(i32),
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}
