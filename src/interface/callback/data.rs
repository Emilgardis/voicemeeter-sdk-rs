pub type AudioInfo = crate::bindings::VBVMR_T_AUDIOINFO;
pub type AudioBuffer = crate::bindings::VBVMR_T_AUDIOBUFFER;

#[repr(transparent)]
pub struct RawCallbackData(std::ptr::NonNull<std::ffi::c_void>);

impl std::fmt::Debug for RawCallbackData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}", self.0)
    }
}
impl RawCallbackData {
    pub fn from_ptr(ptr: *mut std::ffi::c_void) -> Self {
        RawCallbackData(std::ptr::NonNull::new(ptr).unwrap())
    }

    pub unsafe fn as_audio_info<'a>(&self) -> &'a mut AudioInfo {
        unsafe { self.0.cast().as_mut() }
    }

    pub unsafe fn as_audio_buffer<'a>(&self) -> &'a mut AudioBuffer {
        unsafe { self.0.cast().as_mut() }
    }
}
