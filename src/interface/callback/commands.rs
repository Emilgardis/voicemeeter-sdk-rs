//! Callback command types.
//!
//! These are returned to the callback function.
use crate::{
    bindings::VBVMR_CBCOMMAND,
    types::{Device, VoicemeeterApplication},
};

use super::data::{AudioBuffer, AudioInfo, RawCallbackData};

macro_rules! implement {
    (@audio_info $($name:ident),* $(,)?) => {
        $(
            impl HasAudioInfo for $name<'_> {
                fn get(&self) -> &AudioInfo {
                    self.info
                }
            }
        )*
    };
    (@audio_buffer $($name:ident),* $(,)?) => {
        // $(
        //     impl HasAudioBuffer for $name<'_> {
        //         fn get(&self) -> &AudioBuffer {
        //             &self.buffer
        //         }
        //         fn get_mut(&mut self) -> &mut AudioBuffer {
        //             &mut self.buffer
        //         }
        //     }
        // )*
    };
}

implement! { @audio_info
    Starting,
    Ending,
    Change,
}

implement! { @audio_buffer
    BufferIn,
    BufferOut,
    BufferMain,
}

/// Callback command with audio info. Used to abstract away the command type in client code
pub trait HasAudioInfo {
    /// Get the audio info.
    fn get(&self) -> &AudioInfo;
}

/// Callback command with audio buffer. Used to abstract away the command type in client code
pub trait HasAudioBuffer {
    /// Get the audio buffer.
    fn get(&self) -> &AudioBuffer;
}

// FIXME: add .mds for these docs

/// Starting command.
#[derive(Debug)]
pub struct Starting<'a> {
    /// Audio info
    pub info: &'a AudioInfo,
}

impl<'a> Starting<'a> {
    /// Create a new `Starting` command.
    //#[tracing::instrument(skip_all, name = "Starting::new")]
    pub(crate) fn new(info: &'a AudioInfo) -> Self {
        Self { info }
    }
}

/// Ending command.
#[derive(Debug)]
pub struct Ending<'a> {
    /// Audio info
    pub info: &'a AudioInfo,
}

impl<'a> Ending<'a> {
    /// Create a new `Ending` command.
    //#[tracing::instrument(skip_all, name = "Ending::new")]
    pub(crate) fn new(info: &'a AudioInfo) -> Self {
        Self { info }
    }
}

/// Change command.
#[derive(Debug)]
pub struct Change<'a> {
    /// Audio info
    pub info: &'a AudioInfo,
}

impl<'a> Change<'a> {
    /// Create a new `Change` command.
    //#[tracing::instrument(skip_all, name = "Change::new")]
    pub(crate) fn new(info: &'a AudioInfo) -> Self {
        Self { info }
    }
}

/// Data for input mode.
#[derive(Debug)]
pub struct BufferIn<'a> {
    /// Buffer data for input mode
    pub buffer: BufferInData<'a>,
    /// Sample rate
    pub sr: usize,
    /// Number of samples per frame
    pub nbs: usize,
    /// Total number of inputs in buffer.
    pub nbi: usize,
    /// Total number of outputs in buffer.
    pub nbo: usize,
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

impl<'a> BufferIn<'a> {
    //#[tracing::instrument(skip_all, name = "BufferIn::new")]
    pub(crate) fn new(program: VoicemeeterApplication, buffer: &'a AudioBuffer) -> Self {
        Self {
            sr: buffer.audiobuffer_sr as usize,
            nbs: buffer.audiobuffer_nbs as usize,
            nbi: buffer.audiobuffer_nbi as usize,
            nbo: buffer.audiobuffer_nbo as usize,
            buffer: BufferInData::new(program, buffer, buffer.audiobuffer_nbs as usize),
        }
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

    // FIXME: These should be an iterator, maybe.
    //#[tracing::instrument(skip(self), name = "BufferInData::read_write_buffer_on_channel")]
    #[allow(clippy::type_complexity)]
    /// Get the read and write buffers for a specific [device](Device).
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

/// Data for output mode.
#[derive(Debug)]
pub struct BufferOut<'a> {
    /// Sample rate
    pub sr: usize,
    /// Buffer data for output mode
    pub buffer: BufferOutData<'a>,
    /// Number of samples per frame
    pub nbs: usize,
    /// Total number of inputs in buffer.
    pub nbi: usize,
    /// Total number of outputs in buffer.
    pub nbo: usize,
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

impl<'a> BufferOut<'a> {
    //#[tracing::instrument(skip_all, name = "BufferOut::new")]
    pub(crate) fn new(program: VoicemeeterApplication, buffer: &'a AudioBuffer) -> Self {
        Self {
            sr: buffer.audiobuffer_sr as usize,
            nbs: buffer.audiobuffer_nbs as usize,
            nbi: buffer.audiobuffer_nbi as usize,
            nbo: buffer.audiobuffer_nbo as usize,
            buffer: BufferOutData::new(program, buffer, buffer.audiobuffer_nbs as usize),
        }
    }
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
    #[allow(clippy::type_complexity)]
    /// Get the read and write buffers for a specific [device](Device).
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

/// Data for main mode.
#[derive(Debug)]
pub struct BufferMain<'a> {
    /// Buffer data for main mode
    pub buffer: BufferMainData<'a>,
    /// Sample rate
    pub sr: usize,
    /// Number of samples per frame
    pub nbs: usize,
    /// Total number of inputs in buffer.
    pub nbi: usize,
    /// Total number of outputs in buffer.
    pub nbo: usize,
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

impl<'a> BufferMain<'a> {
    //#[tracing::instrument(skip_all, name = "BufferMain::new")]
    pub(crate) fn new(program: VoicemeeterApplication, buffer: &'a AudioBuffer) -> Self {
        Self {
            sr: buffer.audiobuffer_sr as usize,
            nbs: buffer.audiobuffer_nbs as usize,
            nbi: buffer.audiobuffer_nbi as usize,
            nbo: buffer.audiobuffer_nbo as usize,
            buffer: BufferMainData::new(program, buffer, buffer.audiobuffer_nbs as usize),
        }
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

    /// Get the data inside the buffer as slices of pointers
    pub fn data<'b>(&'b self) -> (&'a [*mut f32], &'a [*mut f32]) {
        self.data
    }

    // FIXME: There should be a way to get distinct r/w buffers, right now you can not easily get Strip1 read and OutputA1 write for example.

    // FIXME: These should be an iterator, maybe.
    //#[tracing::instrument(skip(self), name = "BufferMainData::read_write_buffer_on_device")]
    #[allow(clippy::type_complexity)]
    /// Get the read and write buffers for a specific [device](Device).
    ///
    /// # Notes
    ///
    /// The output may be empty if the device does not have an output in the buffer. The second slice will then be empty.
    /// If there is no input buffer for this device, the result would be [None](Option::None).
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

/// Callback command passed to the [audio callback](crate::VoicemeeterRemote::audio_callback_register).
#[derive(Debug)]
#[repr(i32)]
#[cfg(feature = "interface")] // for doc_cfg
pub enum CallbackCommand<'a> {
    /// Starting command
    Starting(Starting<'a>),
    /// Ending command
    Ending(Ending<'a>),
    /// Change command
    Change(Change<'a>),
    /// BufferIn command
    BufferIn(BufferIn<'a>),
    /// BufferOut command
    BufferOut(BufferOut<'a>),
    /// BufferMain command
    BufferMain(BufferMain<'a>),
    /// Other inknown command
    Other(VBVMR_CBCOMMAND, RawCallbackData),
}

impl<'a> CallbackCommand<'a> {
    // TODO: adding a field here makes the program segfault
    //#[tracing::instrument(skip_all, name = "CallbackCommand::new_unchecked")]
    pub(crate) unsafe fn new_unchecked(
        program: VoicemeeterApplication,
        command: VBVMR_CBCOMMAND,
        ptr: RawCallbackData,
    ) -> Self {
        match command {
            VBVMR_CBCOMMAND::STARTING => {
                Self::Starting(Starting::new(unsafe { ptr.as_audio_info() }))
            }
            VBVMR_CBCOMMAND::ENDING => Self::Ending(Ending::new(unsafe { ptr.as_audio_info() })),
            VBVMR_CBCOMMAND::CHANGE => Self::Change(Change::new(unsafe { ptr.as_audio_info() })),
            VBVMR_CBCOMMAND::BUFFER_IN => {
                Self::BufferIn(BufferIn::new(program, unsafe { ptr.as_audio_buffer() }))
            }
            VBVMR_CBCOMMAND::BUFFER_OUT => {
                Self::BufferOut(BufferOut::new(program, unsafe { ptr.as_audio_buffer() }))
            }
            VBVMR_CBCOMMAND::BUFFER_MAIN => {
                Self::BufferMain(BufferMain::new(program, unsafe { ptr.as_audio_buffer() }))
            }
            i => Self::Other(i, ptr),
        }
    }

    /// Get the command "name"
    pub fn name(&self) -> Option<&'static str> {
        Some(match self {
            Self::Starting(_) => "Starting",
            Self::Ending(_) => "Ending",
            Self::Change(_) => "Change",
            Self::BufferIn(_) => "BufferIn",
            Self::BufferOut(_) => "BufferOut",
            Self::BufferMain(_) => "BufferMain",
            Self::Other(_, _) => return None,
        })
    }
}
