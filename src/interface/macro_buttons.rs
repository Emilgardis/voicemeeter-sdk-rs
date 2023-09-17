//! Functions and data types for macro buttons.
//!
//! # Functions
//!
//! * [`is_macrobutton_dirty`](VoicemeeterRemote::is_macrobutton_dirty)
//! * [`get_macrobutton_state`](VoicemeeterRemote::get_macrobutton_state)
//! * [`set_macrobutton_state`](VoicemeeterRemote::set_macrobutton_state)
//! * [`get_macrobutton_trigger_state`](VoicemeeterRemote::get_macrobutton_trigger_state)
//! * [`set_macrobutton_trigger_state`](VoicemeeterRemote::set_macrobutton_trigger_state)
use crate::{bindings::VBVMR_MACROBUTTON_MODE, types::LogicalButton};

use super::VoicemeeterRemote;

/// Status of a macro button.
#[repr(transparent)]
pub struct MacroButtonStatus(pub bool);

impl std::ops::Deref for MacroButtonStatus {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for MacroButtonStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            f.write_str("🟢")
        } else {
            f.write_str("🔴")
        }
    }
}

impl VoicemeeterRemote {
    // FIXME: Only call from one thread, limit it
    /// Check if macro buttons have changed
    ///
    /// Call this function periodically to check if the macro buttons have changed, typically every 10ms.
    ///
    /// # Security
    ///
    /// This method must only be called from one thread.
    pub fn is_macrobutton_dirty(&self) -> Result<bool, IsMacroButtonDirtyError> {
        let res = unsafe { self.raw.VBVMR_MacroButton_IsDirty() };
        match res {
            0 => Ok(false),
            s if s > 0 => Ok(true),
            -1 => Err(IsMacroButtonDirtyError::CannotGetClient),
            -2 => Err(IsMacroButtonDirtyError::NoServer),
            s => Err(IsMacroButtonDirtyError::Other(s)),
        }
    }

    /// Get a specific macro buttons status.
    pub fn get_macrobutton_state(
        &self,
        button: impl Into<LogicalButton>,
    ) -> Result<MacroButtonStatus, GetMacroButtonStatusError> {
        let mut f = 0.0f32;
        let button = button.into();
        let res = unsafe { self.raw.VBVMR_MacroButton_GetStatus(button.0.0, &mut f, 0) };
        match res {
            0 => Ok(MacroButtonStatus(f == 1.)),
            -1 => Err(GetMacroButtonStatusError::CannotGetClient),
            -2 => Err(GetMacroButtonStatusError::NoServer),
            -3 => Err(GetMacroButtonStatusError::UnknownParameter(button)), /* FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe? */
            -5 => Err(GetMacroButtonStatusError::StructureMismatch(button, 1)),
            s => Err(GetMacroButtonStatusError::Other(s)),
        }
    }

    /// Set a macro buttons state.
    ///
    /// use `displayed_state_only` to only set the displayed state of the macro button, but not trigger it's associated requests.
    pub fn set_macrobutton_state(
        &self,
        button: impl Into<LogicalButton>,
        state: bool,
        displayed_state_only: bool,
    ) -> Result<(), SetMacroButtonStatusError> {
        let _f = 0.0f32;
        let button = button.into();
        let bitmode = if displayed_state_only {
            VBVMR_MACROBUTTON_MODE::STATEONLY
        } else {
            VBVMR_MACROBUTTON_MODE::DEFAULT
        };
        let res = unsafe {
            self.raw
                .VBVMR_MacroButton_SetStatus(button.0.0, (state as u32) as f32, bitmode.0)
        };
        match res {
            0 => Ok(()),
            -1 => Err(SetMacroButtonStatusError::CannotGetClient),
            -2 => Err(SetMacroButtonStatusError::NoServer),
            -3 => Err(SetMacroButtonStatusError::UnknownParameter(button)), /* FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe? */
            -5 => Err(SetMacroButtonStatusError::StructureMismatch(button, 1)),
            s => Err(SetMacroButtonStatusError::Other(s)),
        }
    }

    /// Get the trigger state of the macrobutton.
    pub fn get_macrobutton_trigger_state(
        &self,
        button: impl Into<LogicalButton>,
    ) -> Result<MacroButtonStatus, GetMacroButtonStatusError> {
        let mut f = 0.0f32;
        let button = button.into();
        let res = unsafe { self.raw.VBVMR_MacroButton_GetStatus(button.0.0, &mut f, 3) };
        match res {
            0 => Ok(MacroButtonStatus(f == 1.)),
            -1 => Err(GetMacroButtonStatusError::CannotGetClient),
            -2 => Err(GetMacroButtonStatusError::NoServer),
            -3 => Err(GetMacroButtonStatusError::UnknownParameter(button)), /* FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe? */
            -5 => Err(GetMacroButtonStatusError::StructureMismatch(button, 3)),
            s => Err(GetMacroButtonStatusError::Other(s)),
        }
    }

    /// Set the trigger state of the macrobutton.
    pub fn set_macrobutton_trigger_state(
        &self,
        button: impl Into<LogicalButton>,
        state: bool,
    ) -> Result<(), SetMacroButtonStatusError> {
        let button = button.into();
        let res = unsafe {
            self.raw
                .VBVMR_MacroButton_SetStatus(button.0.0, (state as u32) as f32, 3)
        };
        match res {
            0 => Ok(()),
            -1 => Err(SetMacroButtonStatusError::CannotGetClient),
            -2 => Err(SetMacroButtonStatusError::NoServer),
            -3 => Err(SetMacroButtonStatusError::UnknownParameter(button)), /* FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe? */
            -5 => Err(SetMacroButtonStatusError::StructureMismatch(button, 3)),
            s => Err(SetMacroButtonStatusError::Other(s)),
        }
    }
}

/// Errors that can happen when querying macro button "dirty" flag.
#[derive(Debug, thiserror::Error, Clone)]
#[non_exhaustive]
pub enum IsMacroButtonDirtyError {
    // TODO: is this correct? docs say "error (unexpected)""
    /// Cannot get client.
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    /// No server.
    #[error("no server")]
    NoServer,
    /// An unknown error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

/// Errors that can happen when getting macrobutton status.
#[derive(Debug, thiserror::Error, Clone)]
#[non_exhaustive]
pub enum GetMacroButtonStatusError {
    // TODO: is this correct? docs say "error"
    /// Cannot get client.
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    /// No server.
    #[error("no server")]
    NoServer,
    /// Unknown parameter/button.
    #[error("unknown button: {}", 0.0)]
    UnknownParameter(LogicalButton),
    /// Structure mismatch.
    #[error("tried to parse parameter {0:?} as a {1} but it is not")]
    StructureMismatch(LogicalButton, i32),
    /// An unknown error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

/// Errors that can happen when setting macrobutton status.
#[derive(Debug, thiserror::Error, Clone)]
#[non_exhaustive]
pub enum SetMacroButtonStatusError {
    // TODO: is this correct? docs say "error"
    /// Cannot get client.
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    /// No server.
    #[error("no server")]
    NoServer,
    /// Unknown parameter/button.
    #[error("unknown button: {}", 0.0)]
    UnknownParameter(LogicalButton),
    /// Structure mismatch.
    #[error("tried to parse parameter {0:?} as a {1} but it is not")]
    StructureMismatch(LogicalButton, i32),
    /// An unknown error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}
