//! Underlying data types for callbacks

use std::ptr::NonNull;

use crate::types::{VoicemeeterApplication, Device};

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

/// Buffer for main mode.
#[derive(Debug)]
pub struct BufferMainData<'a> {
    data: (&'a [*mut f32], &'a [*mut f32]),
    read_buffer: Vec<&'a [f32]>,
    write_buffer: Vec<&'a mut [f32]>,
    samples_per_frame: usize,
    program: VoicemeeterApplication,
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

    /// Get the data inside the buffer as slices of pointers
    pub fn data<'b>(&'b self) -> (&'a [*mut f32], &'a [*mut f32]) {
        self.data
    }

    // FIXME: There should be a way to get distinct r/w buffers, right now you can not easily get Strip1 read and OutputA1 write for example.

    // FIXME: These should be an iterator, maybe.
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
        // FIXME: assert that the range is contiguous
        for i in 0..r_idx.size {
            let read = unsafe {
                std::slice::from_raw_parts(read[r_idx.start + i], self.samples_per_frame)
            };
            self.read_buffer.push(read);

            if let Some(ref w_idx) = w_idx {
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


impl<'a> BufferOutData<'a> {
    //#[tracing::instrument(skip_all, name = "BufferOutData::new")]
    pub(crate) fn new(
        program: VoicemeeterApplication,
        data: &'a AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        let rw = data.read_write_buffer();
        unsafe {
            tracing::trace!(
                "hmm, {:?}",
                &std::mem::transmute_copy::<_, &[&mut f32]>(&rw.0)
            );
        }
        Self {
            data: rw,
            samples_per_frame,
            read_buffer: Vec::with_capacity(8),
            write_buffer: Vec::with_capacity(8),
            program,
        }
    }

    // FIXME: These should be an iterator, maybe.
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
        // FIXME: assert that the range is contiguous
        for i in 0..idx.size {
            let read =
                unsafe { std::slice::from_raw_parts(read[idx.start + i], self.samples_per_frame) };
            self.read_buffer.push(read);

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

    // FIXME: These should be an iterator, maybe.
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
            let read =
                unsafe { std::slice::from_raw_parts(read[idx.start + i], self.samples_per_frame) };
            self.read_buffer.push(read);

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