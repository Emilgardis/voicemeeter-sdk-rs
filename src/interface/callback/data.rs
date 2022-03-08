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
        let mut array = [(); N].map(|_| <_>::default());
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
        let mut array = [(); N].map(|_| <_>::default());
        for i in 0..N {
            let ptr = data[idx.start + i];
            array[i] = unsafe { std::slice::from_raw_parts(ptr, self.samples_per_frame()) };
        }
        DeviceBuffer::Buffer(array)
    }
}

/// Buffer for main mode.
#[derive(Debug)]
pub struct BufferMainData<'a> {
    data: (&'a [*mut f32], &'a [*mut f32]),
    read_buffer: Vec<&'a [f32]>,
    write_buffer: Vec<&'a mut [f32]>,
    samples_per_frame: usize,
    program: VoicemeeterApplication,
}

impl<'a> BufferDataExt<'a> for BufferMainData<'a> {
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

impl<'a> BufferMainData<'a> {
    //#[tracing::instrument(skip_all, name = "BufferMainData::new")]
    pub(crate) fn new(
        program: VoicemeeterApplication,
        data: &'a AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        Self {
            data: data.read_write_buffer(),
            samples_per_frame,
            read_buffer: Vec::with_capacity(8),
            write_buffer: Vec::with_capacity(8),
            program,
        }
    }

    /// Get all buffers
    pub fn get_all_buffers<'b>(
        &'a mut self,
    ) -> (main::ReadDevices<'a, 'b>, main::WriteDevices<'a, 'b>) {
        (main::ReadDevices::new(self), main::WriteDevices::new(self))
    }

    //#[tracing::instrument(skip(self), name = "BufferMainData::read_write_buffer_on_device")]
    /// Get the read and write buffers for a specific [device](Device).
    ///
    /// # Notes
    ///
    /// The output may be empty if the device does not have an output in the buffer. The second slice will then be empty.
    /// If there is no input buffer for this device, the result would be [None](Option::None).
    #[allow(clippy::type_complexity)]
    pub fn read_write_buffer_on_device<'b>(
        &'b mut self,
        channel: &Device,
    ) -> Option<(&'b [&'a [f32]], &'b mut [&'a mut [f32]])> {
        // FIXME: Find a way to not clear everytime.
        self.read_buffer.clear();
        self.write_buffer.clear();
        let idx = channel.main(&self.program);
        //println!("channel: program: {}, {channel:?}, idx: {idx:?}", &self.program);
        // There should not be any device without a read but a write
        let (r_idx, w_idx) = (idx.0?, idx.1);
        tracing::trace!("getting buffers: {:?}, {:?}", r_idx, w_idx);
        let (read, write) = self.data;
        for i in 0..r_idx.size {
            // by contract from voicemeeter, the ranges will be contigous
            let read = unsafe {
                std::slice::from_raw_parts(read[r_idx.start + i], self.samples_per_frame)
            };
            self.read_buffer.push(read);

            if let Some(ref w_idx) = w_idx {
                // by contract from voicemeeter, the ranges will be contigous
                let write = unsafe {
                    std::slice::from_raw_parts_mut(write[w_idx.start + i], self.samples_per_frame)
                };
                self.write_buffer.push(write);
            }
            // tracing::trace!(
            //     "read from {}, to {}. resulting in {} elems",
            //     r_idx.start,
            //     r_idx.size,
            //     read.len()
            // );
        }
        Some((&self.read_buffer, &mut self.write_buffer))
    }
}

/// Buffer for output mode.
#[derive(Debug)]
pub struct BufferOutData<'a> {
    data: (&'a [*mut f32], &'a [*mut f32]),
    read_buffer: Vec<&'a [f32]>,
    write_buffer: Vec<&'a mut [f32]>,
    samples_per_frame: usize,
    program: VoicemeeterApplication,
}

impl<'a> BufferDataExt<'a> for BufferOutData<'a> {
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
        data: &'a AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        Self {
            data: data.read_write_buffer(),
            samples_per_frame,
            read_buffer: Vec::with_capacity(8),
            write_buffer: Vec::with_capacity(8),
            program,
        }
    }

    /// Get all buffers
    pub fn get_all_buffers<'b>(
        &'a mut self,
    ) -> (output::ReadDevices<'a, 'b>, output::WriteDevices<'a, 'b>) {
        (
            output::ReadDevices::new(self),
            output::WriteDevices::new(self),
        )
    }

    //#[tracing::instrument(skip(self), name = "BufferOutData::read_write_buffer_on_device")]
    /// Get the read and write buffers for a specific [device](Device).
    #[allow(clippy::type_complexity)]
    pub fn read_write_buffer_on_device<'b>(
        &'b mut self,
        channel: &Device,
    ) -> Option<(&'b [&'a [f32]], &'b mut [&'a mut [f32]])> {
        self.read_buffer.clear();
        self.write_buffer.clear();
        let idx = channel.output(&self.program)?;
        // There should not be any channels without a read but a write
        let (read, write) = self.data;
        for i in 0..idx.size {
            // by contract from voicemeeter, the ranges will be contigous
            let read =
                unsafe { std::slice::from_raw_parts(read[idx.start + i], self.samples_per_frame) };
            self.read_buffer.push(read);

            // by contract from voicemeeter, the ranges will be contigous
            let write = unsafe {
                std::slice::from_raw_parts_mut(write[idx.start + i], self.samples_per_frame)
            };
            self.write_buffer.push(write);
            // tracing::trace!(
            //     "read from {}, to {}. resulting in {} elems",
            //     r_idx.start,
            //     r_idx.size,
            //     read.len()
            // );
        }
        Some((&self.read_buffer, &mut self.write_buffer))
    }
}

/// Buffer for input mode.
#[derive(Debug)]
pub struct BufferInData<'a> {
    data: (&'a [*mut f32], &'a [*mut f32]),
    read_buffer: Vec<&'a [f32]>,
    write_buffer: Vec<&'a mut [f32]>,
    samples_per_frame: usize,
    program: VoicemeeterApplication,
}

impl<'a> BufferDataExt<'a> for BufferInData<'a> {
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
        data: &'a AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        Self {
            data: data.read_write_buffer(),
            samples_per_frame,
            read_buffer: Vec::with_capacity(8),
            write_buffer: Vec::with_capacity(8),
            program,
        }
    }

    /// Get all buffers
    pub fn get_all_buffers<'b>(
        &'a mut self,
    ) -> (input::ReadDevices<'a, 'b>, input::WriteDevices<'a, 'b>) {
        (
            input::ReadDevices::new(self),
            input::WriteDevices::new(self),
        )
    }

    //#[tracing::instrument(skip(self), name = "BufferInData::read_write_buffer_on_channel")]
    /// Get the read and write buffers for a specific [device](Device).
    #[allow(clippy::type_complexity)]
    pub fn read_write_buffer_on_device<'b>(
        &'b mut self,
        channel: &Device,
    ) -> Option<(&'b [&'a [f32]], &'b mut [&'a mut [f32]])> {
        self.read_buffer.clear();
        self.write_buffer.clear();
        let idx = channel.input(&self.program)?;
        // There should not be any channels without a read but a write
        let (read, write) = self.data;
        // FIXME: assert that the range is contiguous
        for i in 0..idx.size {
            // by contract from voicemeeter, the ranges will be contigous
            let read =
                unsafe { std::slice::from_raw_parts(read[idx.start + i], self.samples_per_frame) };
            self.read_buffer.push(read);

            // by contract from voicemeeter, the ranges will be contigous
            let write = unsafe {
                std::slice::from_raw_parts_mut(write[idx.start + i], self.samples_per_frame)
            };
            self.write_buffer.push(write);
            // tracing::trace!(
            //     "read from {}, to {}. resulting in {} elems",
            //     r_idx.start,
            //     r_idx.size,
            //     read.len()
            // );
        }
        Some((&self.read_buffer, &mut self.write_buffer))
    }
}
