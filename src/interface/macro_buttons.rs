use crate::{bindings::VBVMR_MACROBUTTON_MODE, types::LogicalButton};

use super::VoicemeeterRemote;

pub struct Status(bool);

impl std::fmt::Display for Status {
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

    pub fn get_macrobutton_state(
        &self,
        button: impl Into<LogicalButton>,
    ) -> Result<Status, GetMacroButtonStatusError> {
        let mut f = 0.0f32;
        let button = button.into();
        let res = unsafe { self.raw.VBVMR_MacroButton_GetStatus(button.0 .0, &mut f, 0) };
        match res {
            0 => Ok(Status(f == 1.)),
            -1 => Err(GetMacroButtonStatusError::CannotGetClient),
            -2 => Err(GetMacroButtonStatusError::NoServer),
            -3 => Err(GetMacroButtonStatusError::UnknownParameter(button)), // FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe?
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
        let mut f = 0.0f32;
        let button = button.into();
        let bitmode = if displayed_state_only {
            VBVMR_MACROBUTTON_MODE::STATEONLY
        } else {
            VBVMR_MACROBUTTON_MODE::DEFAULT
        };
        let res = unsafe {
            self.raw
                .VBVMR_MacroButton_SetStatus(button.0 .0, (state as u32) as f32, bitmode.0)
        };
        match res {
            0 => Ok(()),
            -1 => Err(SetMacroButtonStatusError::CannotGetClient),
            -2 => Err(SetMacroButtonStatusError::NoServer),
            -3 => Err(SetMacroButtonStatusError::UnknownParameter(button)), // FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            -5 => Err(SetMacroButtonStatusError::StructureMismatch(button, 1)),
            s => Err(SetMacroButtonStatusError::Other(s)),
        }
    }

    pub fn get_macrobutton_trigger_state(
        &self,
        button: impl Into<LogicalButton>,
    ) -> Result<Status, GetMacroButtonStatusError> {
        let mut f = 0.0f32;
        let button = button.into();
        let res = unsafe { self.raw.VBVMR_MacroButton_GetStatus(button.0 .0, &mut f, 3) };
        match res {
            0 => Ok(Status(f == 1.)),
            -1 => Err(GetMacroButtonStatusError::CannotGetClient),
            -2 => Err(GetMacroButtonStatusError::NoServer),
            -3 => Err(GetMacroButtonStatusError::UnknownParameter(button)), // FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            -5 => Err(GetMacroButtonStatusError::StructureMismatch(button, 3)),
            s => Err(GetMacroButtonStatusError::Other(s)),
        }
    }

    pub fn set_macrobutton_trigger_state(
        &self,
        button: impl Into<LogicalButton>,
        state: bool,
    ) -> Result<(), SetMacroButtonStatusError> {
        let button = button.into();
        let res = unsafe {
            self.raw
                .VBVMR_MacroButton_SetStatus(button.0 .0, (state as u32) as f32, 3)
        };
        match res {
            0 => Ok(()),
            -1 => Err(SetMacroButtonStatusError::CannotGetClient),
            -2 => Err(SetMacroButtonStatusError::NoServer),
            -3 => Err(SetMacroButtonStatusError::UnknownParameter(button)), // FIXME: Lossless always (assuming vmr doesn't modify :) ), unsafe?
            -5 => Err(SetMacroButtonStatusError::StructureMismatch(button, 3)),
            s => Err(SetMacroButtonStatusError::Other(s)),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum IsMacroButtonDirtyError {
    // TODO: is this correct? docs say "error (unexpected)""
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    #[error("no server")]
    NoServer,
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum GetMacroButtonStatusError {
    // TODO: is this correct? docs say "error"
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    #[error("no server")]
    NoServer,
    #[error("unknown button: {}", 0.0)]
    UnknownParameter(LogicalButton),
    #[error("tried to parse parameter {0:?} as a {1} but it is not")]
    StructureMismatch(LogicalButton, i32),
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum SetMacroButtonStatusError {
    // TODO: is this correct? docs say "error"
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    #[error("no server")]
    NoServer,
    #[error("unknown button: {}", 0.0)]
    UnknownParameter(LogicalButton),
    #[error("tried to parse parameter {0:?} as a {1} but it is not")]
    StructureMismatch(LogicalButton, i32),
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
}
