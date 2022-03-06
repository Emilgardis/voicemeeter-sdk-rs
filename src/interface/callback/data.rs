//! Underlying data types for callbacks

use std::ptr::NonNull;

/// Audio information
pub type AudioInfo = crate::bindings::VBVMR_T_AUDIOINFO;
/// Audio buffers
pub type AudioBuffer = crate::bindings::VBVMR_T_AUDIOBUFFER;

impl AudioBuffer {
    //#[tracing::instrument(level = "debug", skip(self))]
    pub(crate) fn read_write_buffer(&self) -> (&[*mut f32], &[*mut f32]) {
        let first_ptr_r = self.audiobuffer_r[0];
        let first_ptr_w = self.audiobuffer_w[0];
        for (idx, ptr) in self
            .audiobuffer_r
            .iter()
            .enumerate()
            .take(self.audiobuffer_nbi as usize)
        {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        for (idx, ptr) in self
            .audiobuffer_w
            .iter()
            .enumerate()
            .take(self.audiobuffer_nbo as usize)
        {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        //tracing::trace!("read_write_buffer: {:?}", self);

        let k = (
            &self.audiobuffer_r[..self.audiobuffer_nbi as usize],
            &self.audiobuffer_w[..self.audiobuffer_nbo as usize],
        );
        // sanity check
        debug_assert_eq!(first_ptr_r, k.0[0]);
        debug_assert_eq!(first_ptr_w, k.1[0]);
        k
    }
}

#[repr(transparent)]
/// Raw callback data
pub struct RawCallbackData(NonNull<std::ffi::c_void>);

impl std::fmt::Debug for RawCallbackData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}", self.0)
    }
}
// All of these tracing::instrument need to be skipped or we crash.
impl RawCallbackData {
    //#[tracing::instrument(level = "trace", skip_all,name = "RawCallbackData::from_ptr")]'
    /// Create a new `RawCallbackData` from a raw pointer.
    pub fn from_ptr(ptr: *mut std::ffi::c_void) -> Self {
        RawCallbackData(NonNull::new(ptr).unwrap())
    }
    // TODO: Safety
    //#[tracing::instrument(level = "trace", skip_all,name = "RawCallbackData::as_audio_info")]
    /// Get the audio information from the raw callback data.
    ///
    /// # Safety
    ///
    /// 1. This should not be called without ensuring that the pointer is in "scope" and that it is an [`AudioInfo`]
    /// 2. All other conditions of [NonNull::as_ref] has to be met as well
    pub unsafe fn as_audio_info<'a>(&self) -> &'a AudioInfo {
        unsafe { self.0.cast().as_ref() }
    }
    // TODO: Safety
    //#[tracing::instrument(level = "trace", skip_all,name = "RawCallbackData::as_audio_buffer")]
    /// Get the audio information from the raw callback data.
    ///
    /// # Safety
    ///
    /// 1. This should not be called without ensuring that the pointer is in "scope" and that it is an [`AudioBuffer`]
    /// 2. All other conditions of [NonNull::as_ref] has to be met as well
    pub unsafe fn as_audio_buffer<'a>(&self) -> &'a AudioBuffer {
        unsafe { self.0.cast().as_ref() }
    }
}
