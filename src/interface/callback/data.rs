pub type AudioInfo = crate::bindings::VBVMR_T_AUDIOINFO;
pub type AudioBuffer = crate::bindings::VBVMR_T_AUDIOBUFFER;

impl AudioBuffer {
    pub(crate) fn read_write_buffer<'a>(
        &'a self,
        nbi: usize,
        nbo: usize,
    ) -> (&'a [*const f32], &'a [*mut f32]) {
        for (idx, ptr) in self.audiobuffer_r.iter().enumerate().take(nbi) {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        for (idx, ptr) in self.audiobuffer_w.iter().enumerate().take(nbo) {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        (
            &unsafe { &std::mem::transmute_copy::<_, &[*const f32; 128]>(&self.audiobuffer_w)[..nbi]},
            &self.audiobuffer_w[..nbo],
        )
    }
    pub unsafe fn read_buffer_with_len<'a, const R: usize>(&self) -> &'a [*mut f32; R] {
        for (idx, ptr) in self.audiobuffer_r.iter().enumerate().take(R) {
            debug_assert!(!ptr.is_null(), "ptr: {:?} was null at idx: {}", ptr, idx);
        }
        unsafe { std::mem::transmute_copy(&self.audiobuffer_r) }
    }

    pub unsafe fn write_buffer_with_len<'a, const W: usize>(&self) -> &'a [*mut f32; W] {
        for (idx, ptr) in self.audiobuffer_w.iter().enumerate().take(W) {
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
