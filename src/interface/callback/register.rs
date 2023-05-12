//! Functions for callbacks in Voicemeeter.
use std::{
    ffi::{CString, NulError},
    os::raw::c_long,
    ptr,
};

use crate::{
    bindings::VBVMR_CBCOMMAND, interface::callback::data::RawCallbackData, CallbackCommand,
    VoicemeeterRemote,
};

/***************************************************************************** */
/* VB-AUDIO CALLBACK */
/***************************************************************************** */
/* 4x Functions to process all voicemeeter audio input and output channels */
/*  */
/* VBVMR_AudioCallbackRegister	 :to register your audio callback(s) */
/* VBVMR_AudioCallbackStart	     :to start the audio stream */
/* VBVMR_AudioCallbackStop    	 :to stop the audio stream */
/* VBVMR_AudioCallbackUnregister :to unregister / Release callback(s) */
/***************************************************************************** */

// Thanks to sgrif, http://blog.sagetheprogrammer.com/neat-rust-tricks-passing-rust-closures-to-c
fn register_audio_callback<'cb, F>(
    remote: &VoicemeeterRemote,
    mode: &crate::AudioCallbackMode,
    application: *mut std::os::raw::c_char,
    callback: F,
) -> Result<*mut F, AudioCallbackRegisterError>
where
    F: FnMut(CallbackCommand<'cb>, i32) -> c_long,
{
    // This leaks
    let data = Box::into_raw(Box::new(callback));
    tracing::debug!("callback {:p}", data);
    let res = unsafe {
        remote.raw.VBVMR_AudioCallbackRegister(
            mode.0,
            Some(call_closure::<F>),
            data as *mut _,
            application,
        )
    };
    tracing::debug!("registered application");
    match res {
        0 => Ok(data),
        -1 => Err(AudioCallbackRegisterError::NoServer),
        1 => Err(AudioCallbackRegisterError::AlreadyRegistered(unsafe {
            CString::from_raw(application)
        })),
        s => Err(AudioCallbackRegisterError::Unexpected(s)),
    }
}

unsafe extern "C" fn call_closure<'cb, F>(
    user_data: *mut std::os::raw::c_void,
    command: c_long,
    buffer: *mut std::os::raw::c_void,
    nnn: c_long,
) -> c_long
where
    F: FnMut(CallbackCommand<'cb>, i32) -> c_long,
{
    let callback_ptr = user_data as *mut F;
    let callback = unsafe { &mut *callback_ptr };
    let ptr = RawCallbackData::from_ptr(buffer);
    callback(
        unsafe {
            CallbackCommand::new_unchecked(
                crate::types::VoicemeeterApplication::PotatoX64Bits,
                VBVMR_CBCOMMAND(command),
                ptr,
            )
        },
        nnn,
    )
}

/// Guard type for the callback. If this is dropped the callback data will be leaked and newer dropped.
#[must_use = "This structure contains the raw pointer to the closure environment, if this is not returned you will leak memory"]
pub struct CallbackGuard<'a, F> {
    guard: *mut F,
    lt: std::marker::PhantomData<&'a ()>,
}

impl VoicemeeterRemote {
    /// Register a callback for audio.
    ///
    /// The callback is a function that will be called when the audio stream is started, changed or stopped and when data is sent to it.
    ///
    /// The callback takes two arguments, the command sent from voicemeeter and a currently unused [i32] parameter for additional data.
    ///
    /// The [mode](crate::AudioCallbackMode) determines what buffers are returned.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    #[doc = include_str!("../../../examples/simple.rs")]
    /// ```
    ///
    /// ## Complete example
    /// ```rust,no_run
    #[doc = include_str!("../../../examples/output.rs")]
    /// ```
    ///
    #[tracing::instrument(skip(application_name, callback), fields(application_name, mode))]
    pub fn audio_callback_register<'a, 'cb, 'g, F>(
        &'a self,
        mode: crate::AudioCallbackMode,
        application_name: impl AsRef<str>,
        callback: F,
    ) -> Result<CallbackGuard<'g, F>, AudioCallbackRegisterError>
    where
        F: FnMut(CallbackCommand<'cb>, i32) -> c_long + 'g,
    {
        let application_name = application_name.as_ref();
        tracing::Span::current().record("application_name", &application_name);
        //let ctx_span = tracing::trace_span!("voicemeeter_callback");
        //ctx_span.record("application_name", &application_name).record("mode", &mode.0).follows_from(tracing::Span::current());
        assert!(application_name.len() <= 64);
        let mut application = [0u8; 64];
        application[0..application_name.len()].copy_from_slice(application_name.as_bytes());
        let ptr = ptr::addr_of!(self.program);
        tracing::info!("a: {ptr:p}");

        let g = register_audio_callback(
            self,
            &mode,
            ptr::addr_of_mut!(application) as *mut _,
            callback,
        )?;

        Ok(CallbackGuard {
            guard: g,
            lt: Default::default(),
        })
    }

    /// Unregister a callback. This implicitly calls [`VoicemeeterRemote::audio_callback_stop`].
    pub fn audio_callback_unregister<F>(
        &self,
        guard: CallbackGuard<'_, F>,
    ) -> Result<(), AudioCallbackUnregisterError> {
        let res = unsafe { self.raw.VBVMR_AudioCallbackUnregister() };
        match res {
            0 => {
                unsafe { Box::from_raw(guard.guard) };
                Ok(())
            }
            -1 => Err(AudioCallbackUnregisterError::NoServer),
            1 => Err(AudioCallbackUnregisterError::AlreadyUnregistered),
            s => Err(AudioCallbackUnregisterError::Unexpected(s)),
        }
    }

    /// Unregister a callback without dropping the callback, thus leaking data. This implicitly calls [`VoicemeeterRemote::audio_callback_stop`].
    pub fn audio_callback_unregister_leak<F>(&self) -> Result<(), AudioCallbackUnregisterError> {
        let res = unsafe { self.raw.VBVMR_AudioCallbackUnregister() };
        match res {
            0 => Ok(()),
            -1 => Err(AudioCallbackUnregisterError::NoServer),
            1 => Err(AudioCallbackUnregisterError::AlreadyUnregistered),
            s => Err(AudioCallbackUnregisterError::Unexpected(s)),
        }
    }
}

/// Errors that can occur while registering an audio callback.
#[derive(Debug, Clone, thiserror::Error)]
pub enum AudioCallbackRegisterError {
    // TODO: is this correct?
    /// No server.
    #[error("no server")]
    NoServer,
    /// Application is already registered.
    #[error("an application `{}` is already registered", _0.to_string_lossy())]
    AlreadyRegistered(CString),
    /// Could not make a c-string. This is a bug.
    #[error("could not make application name into a c-string")]
    NulError(#[from] NulError),
    /// An unknown error code occured.
    #[error("unexpected error occurred: error code {0}")]
    Unexpected(i32),
}

/// Errors that can occur while unregistering the audio callback.
#[derive(Debug, Clone, thiserror::Error)]
pub enum AudioCallbackUnregisterError {
    /// No server.
    #[error("no server")]
    NoServer,
    /// Application is already unregistered.
    #[error("callback already unregistered")]
    AlreadyUnregistered,
    /// An unknown error code occured.
    #[error("an unexpected error occurred: error code {0}")]
    Unexpected(i32),
}
