use std::{
    ffi::{CString, NulError},
    os::raw::c_long,
    ptr,
};

use crate::{
    bindings::VBVMR_CBCOMMAND, interface::callback::data::RawCallbackData, CallbackCommand,
    VoicemeeterRemote,
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

// Thanks to sgrif, http://blog.sagetheprogrammer.com/neat-rust-tricks-passing-rust-closures-to-c
fn register_audio_callback<'cb, F>(
    remote: &VoicemeeterRemote,
    mode: &crate::AudioCallbackMode,
    application: *mut std::os::raw::c_char,
    callback: F,
) -> Result<*mut F, AudioCallbackRegisterError>
where
    F: FnMut(CallbackCommand<'cb>, i32) -> c_long + 'static,
{
    // This leaks
    let data = Box::into_raw(Box::new(callback));
    println!("callback {:p}", data);
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

#[must_use = "This structure contains the raw pointer to the closure environment, if this is not returned you will leak memory"]
pub struct CallbackGuard<'a, F> {
    guard: *mut F,
    lt: std::marker::PhantomData<&'a ()>,
}

impl VoicemeeterRemote {
    #[tracing::instrument(skip(application_name, callback), fields(application_name, mode))]
    pub fn audio_callback_register<'a, 'cb, F>(
        &'a self,
        mode: crate::AudioCallbackMode,
        application_name: impl AsRef<str>,
        callback: F,
    ) -> Result<CallbackGuard<F>, AudioCallbackRegisterError>
    where
        F: FnMut(CallbackCommand<'cb>, i32) -> c_long + 'static,
    {
        // TODO: SAFETY!!!
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
