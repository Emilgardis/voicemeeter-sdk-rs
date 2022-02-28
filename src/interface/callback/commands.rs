use std::ptr::NonNull;

use crate::{
    bindings::VBVMR_CBCOMMAND,
    types::{Channel, VoicemeeterApplication},
};

use super::data::{AudioBuffer, AudioInfo, RawCallbackData};

macro_rules! implement {
    (@audio_info $($name:ident),* $(,)?) => {
        $(
            impl HasAudioInfo for $name<'_> {
                fn get(&self) -> &AudioInfo {
                    self.info
                }
                fn get_mut(&mut self) -> &mut AudioInfo {
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

pub trait HasAudioInfo {
    fn get(&self) -> &AudioInfo;
    fn get_mut(&mut self) -> &mut AudioInfo;
}

pub trait HasAudioBuffer {
    fn get(&self) -> &AudioBuffer;
    fn get_mut(&mut self) -> &mut AudioBuffer;
}

#[derive(Debug)]
pub struct Starting<'a> {
    pub info: &'a mut AudioInfo,
}

impl<'a> Starting<'a> {
    pub fn new(info: &'a mut AudioInfo) -> Self {
        Self { info }
    }
}

#[derive(Debug)]
pub struct Ending<'a> {
    pub info: &'a mut AudioInfo,
}

impl<'a> Ending<'a> {
    pub fn new(info: &'a mut AudioInfo) -> Self {
        Self { info }
    }
}

#[derive(Debug)]
pub struct Change<'a> {
    pub info: &'a mut AudioInfo,
}

impl<'a> Change<'a> {
    pub fn new(info: &'a mut AudioInfo) -> Self {
        Self { info }
    }
}

#[derive(Debug)]
pub struct BufferIn<'a> {
    pub buffer: BufferInData<'a>,
    pub sr: usize,
    pub nbs: usize,
    pub nbi: usize,
    pub nbo: usize,
}

#[derive(Debug)]
pub struct BufferInData<'a> {
    data: (&'a [*mut f32], &'a [*mut f32]),
    read_buffer: Vec<&'a [f32]>,
    write_buffer: Vec<&'a mut [f32]>,
    samples_per_frame: usize,
    program: VoicemeeterApplication,
}

impl<'a> BufferIn<'a> {
    pub fn new(program: &VoicemeeterApplication, buffer: &'a mut AudioBuffer) -> Self {
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
    pub fn new(
        program: &VoicemeeterApplication,
        data: &'a mut AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        Self {
            data: data.read_write_buffer(),
            samples_per_frame,
            read_buffer: Vec::with_capacity(data.audiobuffer_nbs as usize),
            write_buffer: Vec::with_capacity(data.audiobuffer_nbs as usize),
            program: *program,
        }
    }

    // FIXME: These should be an iterator, maybe.
    #[tracing::instrument(skip(self))]
    pub fn read_write_buffer_on_channel<'b>(
        &'b mut self,
        channel: &Channel,
    ) -> Option<(&'b mut [&'a [f32]], &'b mut [&'a mut [f32]])> {
        tracing::trace!("clearing read and write buffer");
        self.read_buffer.clear();
        self.write_buffer.clear();
        let idx = channel.input(&self.program)?;
        let (read, write) = self.data;
        // FIXME: assert that the range is contiguous
        for i in 0..self.samples_per_frame {
            let read = unsafe { std::slice::from_raw_parts(read[idx.start], idx.size) };
            let write = unsafe { std::slice::from_raw_parts_mut(write[idx.start], idx.size) };
            // tracing::trace!(
            //     "read from {}, to {}. resulting in {} elems",
            //     idx.start,
            //     idx.size,
            //     read.len()
            // );
            self.read_buffer.push(read);
            self.write_buffer.push(write);
        }
        Some((&mut self.read_buffer, &mut self.write_buffer))
    }
}

#[derive(Debug)]
pub struct BufferOut<'a> {
    pub sr: usize,
    pub buffer: BufferOutData<'a>,
    pub nbs: usize,
    pub nbi: usize,
    pub nbo: usize,
}

#[derive(Debug)]
pub struct BufferOutData<'a> {
    data: (&'a [*mut f32], &'a [*mut f32]),
    read_buffer: Vec<&'a [f32]>,
    write_buffer: Vec<&'a mut [f32]>,
    samples_per_frame: usize,
    program: VoicemeeterApplication,
}

impl<'a> BufferOut<'a> {
    pub fn new(program: &VoicemeeterApplication, buffer: &'a mut AudioBuffer) -> Self {
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
    pub fn new(
        program: &VoicemeeterApplication,
        data: &'a mut AudioBuffer,
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
            read_buffer: Vec::with_capacity(data.audiobuffer_nbs as usize),
            write_buffer: Vec::with_capacity(data.audiobuffer_nbs as usize),
            program: *program,
        }
    }

    // FIXME: These should be an iterator, maybe.
    #[tracing::instrument(skip(self))]
    pub fn read_write_buffer_on_channel<'b>(
        &'b mut self,
        channel: &Channel,
    ) -> Option<(&'b mut [&'a [f32]], &'b mut [&'a mut [f32]])> {
        tracing::trace!("clearing read and write buffer");
        self.read_buffer.clear();
        self.write_buffer.clear();
        let idx = channel.output(&self.program)?;
        let (read, write) = self.data;
        // FIXME: assert that the range is contiguous
        for i in 0..self.samples_per_frame {
            let read = unsafe { std::slice::from_raw_parts(read[idx.start], idx.size) };
            let write = unsafe { std::slice::from_raw_parts_mut(write[idx.start], idx.size) };
            // tracing::trace!(
            //     "read from {}, to {}. resulting in {} elems",
            //     idx.start,
            //     idx.size,
            //     read.len()
            // );
            self.read_buffer.push(read);
            self.write_buffer.push(write);
        }
        Some((&mut self.read_buffer, &mut self.write_buffer))
    }
}

#[derive(Debug)]
pub struct BufferMain<'a> {
    pub buffer: BufferMainData<'a>,
    pub sr: usize,
    pub nbs: usize,
    pub nbi: usize,
    pub nbo: usize,
}

#[derive(Debug)]
pub struct BufferMainData<'a> {
    data: (&'a [*mut f32], &'a [*mut f32]),
    read_buffer: Vec<&'a [f32]>,
    write_buffer: Vec<&'a mut [f32]>,
    samples_per_frame: usize,
    program: VoicemeeterApplication,
}

impl<'a> BufferMain<'a> {
    pub fn new(program: &VoicemeeterApplication, buffer: &'a mut AudioBuffer) -> Self {
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
    pub fn new(
        program: &VoicemeeterApplication,
        data: &'a mut AudioBuffer,
        samples_per_frame: usize,
    ) -> Self {
        Self {
            data: data.read_write_buffer(),
            samples_per_frame,
            read_buffer: Vec::with_capacity(data.audiobuffer_nbs as usize),
            write_buffer: Vec::with_capacity(data.audiobuffer_nbs as usize),
            program: *program,
        }
    }

    pub unsafe fn data<'b>(&'b self) -> (&'a [*mut f32], &'a [*mut f32]) {
        self.data
    }

    // FIXME: These should be an iterator, maybe.
    #[tracing::instrument(skip(self))]
    pub fn read_write_buffer_on_channel<'b>(
        &'b mut self,
        channel: &Channel,
    ) -> Option<(&'b mut [&'a [f32]], &'b mut [&'a mut [f32]])> {
        self.read_buffer.clear();
        self.write_buffer.clear();
        let idx = channel.main(&self.program);
        // There should not be any channels without a read but a write
        let (r_idx, w_idx) = (idx.0?, idx.1);
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
        Some((&mut self.read_buffer, &mut self.write_buffer))
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum CallbackCommand<'a> {
    Starting(Starting<'a>),
    Ending(Ending<'a>),
    Change(Change<'a>),
    BufferIn(BufferIn<'a>),
    BufferOut(BufferOut<'a>),
    BufferMain(BufferMain<'a>),
    Other(VBVMR_CBCOMMAND, RawCallbackData),
}

impl<'a> CallbackCommand<'a> {
    pub(crate) unsafe fn new_unchecked(
        program: &VoicemeeterApplication,
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
    pub fn name(&self) -> &'static str {
        match self {
            Self::Starting(_) => "Starting",
            Self::Ending(_) => "Ending",
            Self::Change(_) => "Change",
            Self::BufferIn(_) => "BufferIn",
            Self::BufferOut(_) => "BufferOut",
            Self::BufferMain(_) => "BufferMain",
            Self::Other(_, _) => "unknown",
        }
    }
}
