//! Underlying data types for callbacks

mod buffer_abstraction;

pub use buffer_abstraction::{input, main, output, DeviceBuffer};

use std::ptr::NonNull;

use crate::types::{ChannelIndex, Device, VoicemeeterApplication};

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
    //#[tracing::instrument(level = "trace", skip_all,name = "RawCallbackData::as_audio_info")]
    /// Get the audio information from the raw callback data.
    ///
    /// # Safety
    ///
    /// 1. This should not be called without ensuring that the pointer is in "scope" and that it is an [`AudioInfo`]
    /// 2. All other conditions of [NonNull::as_mut] has to be met as well
    pub unsafe fn as_audio_info<'a>(&self) -> &'a mut AudioInfo {
        unsafe { self.0.cast().as_mut() }
    }
    //#[tracing::instrument(level = "trace", skip_all,name = "RawCallbackData::as_audio_buffer")]
    /// Get the audio information from the raw callback data.
    ///
    /// # Safety
    ///
    /// 1. This should not be called without ensuring that the pointer is in "scope" and that it is an [`AudioBuffer`]
    /// 2. All other conditions of [NonNull::as_mut] has to be met as well
    pub unsafe fn as_audio_buffer<'a>(&self) -> &'a mut AudioBuffer {
        unsafe { self.0.cast().as_mut() }
    }
}

trait BufferDataExt<'a> {
    /// Get the data inside the buffer as slices of pointers
    fn data<'b>(&'b self) -> (&'a [*mut f32], &'a [*mut f32]);
    /// Given a device, return a channel index
    fn channel_index_write(&self, device: &Device) -> Option<ChannelIndex>;
    fn channel_index_read(&self, device: &Device) -> Option<ChannelIndex>;
    fn samples_per_frame(&self) -> usize;
    /// Get the device write buffer for a device at idx.start..idx.start+N.
    ///
    /// This is unsafe because it can create mutable references arbitrarily.
    #[inline]
    unsafe fn device_write<'b, const N: usize>(
        &'a self,
        device: &Device,
    ) -> DeviceBuffer<[&'b mut [f32]; N]> {
        let idx = if let Some(idx) = self.channel_index_write(device) {
            idx
        } else {
            return DeviceBuffer::None;
        };
        assert_eq!(N, idx.size);
        let data = self.data().1;
        // Each channel inside idx.start..(idx..start+idx.size) will be a *mut f32 that points to an array of `nbs` f32s.
        let mut array = [(); N].map(|_| Default::default());
        for i in 0..N {
            let ptr = data[idx.start + i];
            array[i] = unsafe { std::slice::from_raw_parts_mut(ptr, self.samples_per_frame()) };
        }
        DeviceBuffer::Buffer(array)
    }
    /// Get the device read buffer for a device at idx.start..idx.start+N.
    ///
    /// This is unsafe because it can create mutable references arbitrarily.
    #[inline]
    unsafe fn device_read<'b, const N: usize>(
        &'a self,
        device: &Device,
    ) -> DeviceBuffer<[&'b [f32]; N]> {
        let idx = if let Some(idx) = self.channel_index_read(device) {
            idx
        } else {
            return DeviceBuffer::None;
        };
        assert_eq!(N, idx.size, "on device: {device:?}");
        let data = self.data().0;
        // Each channel inside idx.start..(idx..start+idx.size) will be a *mut f32 that points to an array of `nbs` f32s.
        let mut array = [(); N].map(|_| Default::default());
        for i in 0..N {
            let ptr = data[idx.start + i];
            array[i] = unsafe { std::slice::from_raw_parts(ptr, self.samples_per_frame()) };
        }
        DeviceBuffer::Buffer(array)
    }
}

/// Buffer for main mode.
///
/// Retrieved via [`CallbackCommand::BufferMain`](crate::interface::callback::CallbackCommand::BufferMain)
pub struct BufferMainData<'a> {
    /// Read
    pub read: main::ReadDevices<'a, 'a>,
    /// Write
    pub write: main::WriteDevices<'a, 'a>,
}

pub(crate) struct Main<'a> {
    program: VoicemeeterApplication,
    samples_per_frame: usize,
    data: (&'a [*mut f32], &'a [*mut f32]),
}

impl<'a> BufferDataExt<'a> for Main<'a> {
    #[inline]
    fn data<'b>(&'b self) -> (&'a [*mut f32], &'a [*mut f32]) {
        self.data
    }

    #[inline]
    fn channel_index_read(&self, device: &Device) -> Option<ChannelIndex> {
        device.main(&self.program).0
    }

    #[inline]
    fn channel_index_write(&self, device: &Device) -> Option<ChannelIndex> {
        device.main(&self.program).1
    }

    #[inline]
    fn samples_per_frame(&self) -> usize {
        self.samples_per_frame
    }
}

//#[tracing::instrument(skip_all, name = "BufferMainData::new")]
impl<'a> BufferMainData<'a> {
    pub(crate) fn new<'b: 'a>(
        program: VoicemeeterApplication,
        data: &'b AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        let mut data = Main {
            data: data.read_write_buffer(),
            samples_per_frame,
            program,
        };
        unsafe {
            Self {
                read: main::ReadDevices::new(&mut data),
                write: main::WriteDevices::new(&mut data),
            }
        }
    }

    /// Convenience function to get the read and write buffers
    ///
    /// ```rust,no_run
    /// # use voicemeeter::interface::callback::BufferMain; let data: BufferMain = unimplemented!();
    /// let (read, mut write) = data.buffer.get_buffers();
    /// ```
    pub fn get_buffers(self) -> (main::ReadDevices<'a, 'a>, main::WriteDevices<'a, 'a>) {
        (self.read, self.write)
    }
}

/// Buffer for output mode.
///
/// Retrieved via [`CallbackCommand::BufferOut`](crate::interface::callback::CallbackCommand::BufferOut)
pub struct BufferOutData<'a> {
    /// Read
    pub read: output::ReadDevices<'a, 'a>,
    /// Write
    pub write: output::WriteDevices<'a, 'a>,
}
pub(crate) struct Output<'a> {
    program: VoicemeeterApplication,
    samples_per_frame: usize,
    data: (&'a [*mut f32], &'a [*mut f32]),
}

impl<'a> BufferDataExt<'a> for Output<'a> {
    #[inline]
    fn data<'b>(&'b self) -> (&'a [*mut f32], &'a [*mut f32]) {
        self.data
    }
    #[inline]
    fn channel_index_read(&self, device: &Device) -> Option<ChannelIndex> {
        device.output(&self.program)
    }

    #[inline]
    fn channel_index_write(&self, device: &Device) -> Option<ChannelIndex> {
        device.output(&self.program)
    }

    #[inline]
    fn samples_per_frame(&self) -> usize {
        self.samples_per_frame
    }
}

impl<'a> BufferOutData<'a> {
    //#[tracing::instrument(skip_all, name = "BufferOutData::new")]
    pub(crate) fn new(
        program: VoicemeeterApplication,
        data: &'a mut AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        let mut data = Output {
            data: data.read_write_buffer(),
            samples_per_frame,
            program,
        };
        unsafe {
            Self {
                read: output::ReadDevices::new(&mut data),
                write: output::WriteDevices::new(&mut data),
            }
        }
    }

    /// Convenience function to get the read and write buffers
    ///
    /// ```rust,no_run
    /// # use voicemeeter::interface::callback::BufferOut; let data: BufferOut = unimplemented!();
    /// let (read, mut write) = data.buffer.get_buffers();
    /// ```
    pub fn get_buffers(self) -> (output::ReadDevices<'a, 'a>, output::WriteDevices<'a, 'a>) {
        (self.read, self.write)
    }
}

/// Buffer for input mode.
///
/// Retrieved via [`CallbackCommand::BufferIn`](crate::interface::callback::CallbackCommand::BufferIn)
pub struct BufferInData<'a> {
    /// Read
    pub read: input::ReadDevices<'a, 'a>,
    /// Write
    pub write: input::WriteDevices<'a, 'a>,
}
pub(crate) struct Input<'a> {
    program: VoicemeeterApplication,
    samples_per_frame: usize,
    data: (&'a [*mut f32], &'a [*mut f32]),
}

impl<'a> BufferDataExt<'a> for Input<'a> {
    #[inline]
    fn data<'b>(&'b self) -> (&'a [*mut f32], &'a [*mut f32]) {
        self.data
    }
    #[inline]
    fn channel_index_read(&self, device: &Device) -> Option<ChannelIndex> {
        device.input(&self.program)
    }

    #[inline]
    fn channel_index_write(&self, device: &Device) -> Option<ChannelIndex> {
        device.input(&self.program)
    }

    #[inline]
    fn samples_per_frame(&self) -> usize {
        self.samples_per_frame
    }
}

impl<'a> BufferInData<'a> {
    //#[tracing::instrument(skip_all, name = "BufferInData::new")]
    pub(crate) fn new(
        program: VoicemeeterApplication,
        data: &'a mut AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        let mut data = Input {
            data: data.read_write_buffer(),
            samples_per_frame,
            program,
        };
        unsafe {
            Self {
                read: input::ReadDevices::new(&mut data),
                write: input::WriteDevices::new(&mut data),
            }
        }
    }

    /// Convenience function to get the read and write buffers
    ///
    /// ```rust,no_run
    /// # use voicemeeter::interface::callback::BufferIn; let data: BufferIn = unimplemented!();
    /// let (read, mut write) = data.buffer.get_buffers();
    /// ```
    pub fn get_buffers(self) -> (input::ReadDevices<'a, 'a>, input::WriteDevices<'a, 'a>) {
        (self.read, self.write)
    }
}
