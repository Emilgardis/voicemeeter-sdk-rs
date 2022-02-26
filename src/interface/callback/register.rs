use std::{
    ffi::{CString, NulError},
    os::raw::c_long,
};

use crate::{
    bindings::{VBVMR_AUDIOCALLBACK, VBVMR_CBCOMMAND},
    interface::callback::data::RawCallbackData,
    CallbackCommand, VoicemeeterRemote,
};

/******************************************************************************/
/*                             VB-AUDIO CALLBACK                              */
/******************************************************************************/
/* 4x Functions to process all voicemeeter audio input and output channels    */
/*                                                                            */
/* VBVMR_AudioCallbackRegister	 :to register your audio callback(s)          */
/* VBVMR_AudioCallbackStart	     :to start the audio stream                   */
/* VBVMR_AudioCallbackStop    	 :to stop the audio stream                    */
/* VBVMR_AudioCallbackUnregister :to unregister / Release callback(s)         */
/******************************************************************************/

impl VoicemeeterRemote {
    pub fn audio_callback_register_main(
        &self,
        application_name: impl AsRef<str>,
        mut callback: impl FnMut(CallbackCommand<'_>, i32) -> c_long,
    ) -> Result<(), AudioCallbackRegisterError> {
        // TODO: SAFETY!!!
        #[allow(unused_mut)]
        let mut application = CString::new(application_name.as_ref())?;
        let mut callback_transformed = |t, b: *mut std::ffi::c_void, nnn| {
            debug_assert!(!b.is_null());
            let ptr = RawCallbackData::from_ptr(b);
            callback(
                unsafe { CallbackCommand::new_unchecked(VBVMR_CBCOMMAND(t), ptr) },
                nnn,
            )
        };
        let (user_data, callback) = crate::ffi::split::split_closure(&mut callback_transformed);
        let res = unsafe {
            self.raw.VBVMR_AudioCallbackRegister(
                VBVMR_AUDIOCALLBACK::MAIN.0,
                Some(callback),
                user_data,
                application.as_ptr() as *mut _,
            )
        };
        match res {
            0 => Ok(()),
            -1 => Err(AudioCallbackRegisterError::NoServer),
            1 => Err(AudioCallbackRegisterError::AlreadyRegistered(application)),
            s => Err(AudioCallbackRegisterError::Unexpected(s)),
        }
    }

    pub fn audio_callback_register_input(
        &self,
        application_name: impl AsRef<str>,
        mut callback: impl FnMut(CallbackCommand<'_>, i32) -> c_long,
    ) -> Result<(), AudioCallbackRegisterError> {
        // TODO: SAFETY!!!
        #[allow(unused_mut)]
        let mut application = CString::new(application_name.as_ref())?;
        let mut callback_transformed = |t, b: *mut std::ffi::c_void, nnn| {
            debug_assert!(!b.is_null());
            let ptr = RawCallbackData::from_ptr(b);
            callback(
                unsafe { CallbackCommand::new_unchecked(VBVMR_CBCOMMAND(t), ptr) },
                nnn,
            )
        };
        let (user_data, callback) = crate::ffi::split::split_closure(&mut callback_transformed);
        let res = unsafe {
            self.raw.VBVMR_AudioCallbackRegister(
                VBVMR_AUDIOCALLBACK::INPUT.0,
                Some(callback),
                user_data,
                application.as_ptr() as *mut _,
            )
        };
        match res {
            0 => Ok(()),
            -1 => Err(AudioCallbackRegisterError::NoServer),
            1 => Err(AudioCallbackRegisterError::AlreadyRegistered(application)),
            s => Err(AudioCallbackRegisterError::Unexpected(s)),
        }
    }

    pub fn audio_callback_register_output(
        &self,
        application_name: impl AsRef<str>,
        mut callback: impl FnMut(CallbackCommand<'_>, i32) -> c_long,
    ) -> Result<(), AudioCallbackRegisterError> {
        // TODO: SAFETY!!!
        #[allow(unused_mut)]
        let mut application = CString::new(application_name.as_ref())?;
        let mut callback_transformed = |t, b: *mut std::ffi::c_void, nnn| {
            debug_assert!(!b.is_null());
            let ptr = RawCallbackData::from_ptr(b);
            callback(
                unsafe { CallbackCommand::new_unchecked(VBVMR_CBCOMMAND(t), ptr) },
                nnn,
            )
        };
        let (user_data, callback) = crate::ffi::split::split_closure(&mut callback_transformed);
        let res = unsafe {
            self.raw.VBVMR_AudioCallbackRegister(
                VBVMR_AUDIOCALLBACK::OUTPUT.0,
                Some(callback),
                user_data,
                application.as_ptr() as *mut _,
            )
        };
        match res {
            0 => Ok(()),
            -1 => Err(AudioCallbackRegisterError::NoServer),
            1 => Err(AudioCallbackRegisterError::AlreadyRegistered(application)),
            s => Err(AudioCallbackRegisterError::Unexpected(s)),
        }
    }

    pub fn audio_callback_unregister(&self) -> Result<(), AudioCallbackUnregisterError> {
        let res = unsafe { self.raw.VBVMR_AudioCallbackUnregister() };
        match res {
            0 => Ok(()),
            -1 => Err(AudioCallbackUnregisterError::NoServer),
            1 => Err(AudioCallbackUnregisterError::AlreadyUnregistered),
            s => Err(AudioCallbackUnregisterError::Unexpected(s)),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum AudioCallbackRegisterError {
    // TODO: is this correct?
    #[error("no server")]
    NoServer,
    #[error("an application `{}` is already registered for this callback type", _0.to_string_lossy())]
    AlreadyRegistered(CString),
    #[error("could not make application name into a c-string")]
    NulError(#[from] NulError),
    #[error("an unexpected error occurred")]
    Unexpected(i32),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum AudioCallbackUnregisterError {
    #[error("no server")]
    NoServer,
    #[error("callback already unregistered")]
    AlreadyUnregistered,
    #[error("an unexpected error occurred")]
    Unexpected(i32),
}
