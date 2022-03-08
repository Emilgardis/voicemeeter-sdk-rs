//! Level and midi related functions
use std::ptr;

pub use crate::types::{Device, LevelType};

use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    // TODO: one thread only
    /// Get the level of a channel on a device
    pub fn get_level(
        &self,
        level_type: LevelType,
        device: Device,
        channel: usize,
    ) -> Result<Option<f32>, GetLevelError> {
        let mut f = std::f32::NAN;
        let dev_num =
            if let Some(dev_num) = device.as_level_device_num(&self.program, level_type, channel) {
                dev_num as i32
            } else {
                return Ok(None);
            };
        let res = unsafe { self.raw.VBVMR_GetLevel(level_type as i32, dev_num, &mut f) };
        match res {
            0 => Ok(Some(f)),
            -1 => Err(GetLevelError::CannotGetClient),
            -2 => Err(GetLevelError::NoServer),
            -3 => Err(GetLevelError::NoLevel),
            -4 => Err(GetLevelError::OutOfRange),
            s => Err(GetLevelError::Other(s)),
        }
    }

    // TODO: one thread only
    /// Get a midi message.
    pub fn get_midi_message(&self) -> Result<Vec<u8>, GetMidiMessageError> {
        let mut v = vec![0; 1024];
        let len = self.get_midi_message_buff(&mut v)?;
        v.truncate(len);
        Ok(v)
    }

    // TODO: one thread only
    /// Get a midi message with a set buffer.
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

/// Errors that can happen when querying levels from Voicemeeter.
#[derive(Debug, thiserror::Error, Clone)]
pub enum GetLevelError {
    /// Cannot get client.
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    /// No server found.
    #[error("no server")]
    NoServer,
    /// No level found.
    #[error("no level available")]
    NoLevel,
    /// Level is out of range.
    #[error("out of range")]
    OutOfRange,
    /// An unexpected error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

/// Errors that can happen when querying midi messages from Voicemeeter.
#[derive(Debug, thiserror::Error, Clone)]
pub enum GetMidiMessageError {
    /// Cannot get client.
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    /// No server found.
    #[error("no server")]
    NoServer,
    /// No midi data found.
    #[error("no level available")]
    NoMidiData(i32),
    /// An unexpected error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}
