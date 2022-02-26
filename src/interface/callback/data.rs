pub type AudioInfo = crate::bindings::VBVMR_T_AUDIOINFO;
pub type AudioBuffer = crate::bindings::VBVMR_T_AUDIOBUFFER;

impl AudioBuffer {
    pub fn read_write_buffer_with_len<'a, const R: usize, const W: usize>(
        &self,
    ) -> (&'a [f32], &'a mut [f32]) {
        for (idx, ptr) in self.audiobuffer_r.iter().enumerate().take(R) {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        for (idx, ptr) in self.audiobuffer_w.iter().enumerate().take(W) {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        (
            unsafe { std::mem::transmute_copy(&self.audiobuffer_r) },
            unsafe { std::mem::transmute_copy(&self.audiobuffer_w) },
        )
    }

    pub fn read_buffer_with_len<'a, const N: usize>(&self) -> &'a mut [f32; N] {
        for (idx, ptr) in self.audiobuffer_r.iter().enumerate().take(N) {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        unsafe { std::mem::transmute_copy(&self.audiobuffer_r) }
    }

    pub fn write_buffer_with_len<'a, const N: usize>(&mut self) -> &'a mut [f32; N] {
        for (idx, ptr) in self.audiobuffer_w.iter().enumerate().take(N) {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        unsafe { std::mem::transmute_copy(&self.audiobuffer_w) }
    }
}

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
