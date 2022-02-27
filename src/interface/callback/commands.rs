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
pub struct BufferInData<'a>(&'a mut AudioBuffer);

impl std::ops::Deref for BufferInData<'_> {
    type Target = AudioBuffer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BufferInData<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> BufferIn<'a> {
    pub fn new(buffer: &'a mut AudioBuffer) -> Self {
        Self {
            sr: buffer.audiobuffer_sr as usize,
            nbs: buffer.audiobuffer_nbs as usize,
            nbi: buffer.audiobuffer_nbi as usize,
            nbo: buffer.audiobuffer_nbo as usize,
            buffer: BufferInData(buffer),
        }
    }
}

// impl<'a> BufferInData<'a> {
//     #[inline]
//     pub fn read_write_buffer(
//         &mut self,
//         program: &VoicemeeterApplication,
//     ) -> (&[*mut f32], &[*mut f32]) {
//         match program {
//             VoicemeeterApplication::Voicemeeter => self.read_write_buffer_with_len::<12, 12>(),
//             VoicemeeterApplication::VoicemeeterBanana => {
//                 self.read_write_buffer_with_len::<22, 22>()
//             }
//             VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
//                 self.read_write_buffer_with_len::<34, 34>()
//             }
//             VoicemeeterApplication::Other => {
//                 // TODO: Find the first non-null ptr and return to that?
//                 self.read_write_buffer_with_len::<34, 34>()
//             }
//         }
//     }

//     pub fn read_buffer(&mut self, program: &VoicemeeterApplication) -> &[*mut f32] {
//         self.read_write_buffer(program).0
//     }
//     pub fn write_buffer(&mut self, program: &VoicemeeterApplication) -> &[*mut f32] {
//         self.read_write_buffer(program).1
//     }

//     pub fn read_output(&'a mut self, program: &VoicemeeterApplication) -> &[*mut f32] {
//         self.read_buffer(program)
//     }
// }

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
    data: (&'a [*const f32], &'a [*mut f32]),
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
        Self {
            data: data.read_write_buffer(data.audiobuffer_nbi as usize, data.audiobuffer_nbo as usize),
            samples_per_frame,
            read_buffer: Vec::with_capacity(data.audiobuffer_nbs as usize),
            write_buffer: Vec::with_capacity(data.audiobuffer_nbs as usize),
            program: *program,
        }
    }

    // FIXME: These should be an iterator, maybe.
    #[inline]
    pub fn read_write_buffer_on_channel(
        &'a mut self,
        channel: Channel,
    ) -> Option<(&'a mut [&'a [f32]], &'a mut [&'a mut [f32]])> {
        self.read_buffer.clear();
        self.write_buffer.clear();
        let idx = channel.output(&self.program)?;
        let (read, write) = self.data;
        // FIXME: assert that the range is contiguous
        for i in 0..idx.size {
            let read = unsafe { std::slice::from_raw_parts(read[idx.start], idx.size) };
            let write = unsafe { std::slice::from_raw_parts_mut(write[idx.start], idx.size) };
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
pub struct BufferMainData<'a>(&'a mut AudioBuffer);

impl std::ops::Deref for BufferMainData<'_> {
    type Target = AudioBuffer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BufferMainData<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> BufferMain<'a> {
    pub fn new(buffer: &'a mut AudioBuffer) -> Self {
        Self {
            sr: buffer.audiobuffer_sr as usize,
            nbs: buffer.audiobuffer_nbs as usize,
            nbi: buffer.audiobuffer_nbi as usize,
            nbo: buffer.audiobuffer_nbo as usize,
            buffer: BufferMainData(buffer),
        }
    }
}

// impl<'a> BufferMainData<'a> {
//     #[inline]
//     pub fn read_write_buffer(
//         &mut self,
//         program: &VoicemeeterApplication,
//     ) -> (&[*mut f32], &[*mut f32]) {
//         // FIXME: This is already captured in nbi/nbo
//         match program {
//             VoicemeeterApplication::Voicemeeter => self.read_write_buffer_with_len::<28, 16>(),
//             VoicemeeterApplication::VoicemeeterBanana => {
//                 self.read_write_buffer_with_len::<62, 40>()
//             }
//             VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
//                 self.read_write_buffer_with_len::<98, 64>()
//             }
//             VoicemeeterApplication::Other => {
//                 // TODO: Find the first non-null ptr and return to that?
//                 self.read_write_buffer_with_len::<98, 64>()
//             }
//         }
//     }
//     pub fn read_buffer(&mut self, program: &VoicemeeterApplication) -> &[*mut f32] {
//         self.read_write_buffer(program).0
//     }

//     pub fn write_buffer(&mut self, program: &VoicemeeterApplication) -> &[*mut f32] {
//         self.read_write_buffer(program).1
//     }

//     // pub fn read_output(&'a mut self, program: &VoicemeeterApplication) -> &[f32] {
//     //     let buf = self.read_buffer(program);
//     //     match program {
//     //         VoicemeeterApplication::Voicemeeter => &buf[12..=27],
//     //         VoicemeeterApplication::VoicemeeterBanana => &buf[22..=61],
//     //         VoicemeeterApplication::VoicemeeterPotato | VoicemeeterApplication::PotatoX64Bits => {
//     //             &buf[34..64]
//     //         }
//     //         VoicemeeterApplication::Other => {
//     //             // TODO: Find the first non-null ptr and return to that?
//     //             buf
//     //         }
//     //     }
//     // }
// }

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
    pub(crate) unsafe fn new_unchecked(program: &VoicemeeterApplication, command: VBVMR_CBCOMMAND, ptr: RawCallbackData) -> Self {
        match command {
            VBVMR_CBCOMMAND::STARTING => {
                Self::Starting(Starting::new(unsafe { ptr.as_audio_info() }))
            }
            VBVMR_CBCOMMAND::ENDING => Self::Ending(Ending::new(unsafe { ptr.as_audio_info() })),
            VBVMR_CBCOMMAND::CHANGE => Self::Change(Change::new(unsafe { ptr.as_audio_info() })),
            VBVMR_CBCOMMAND::BUFFER_IN => {
                Self::BufferIn(BufferIn::new(unsafe { ptr.as_audio_buffer() }))
            }
            VBVMR_CBCOMMAND::BUFFER_OUT => {
                Self::BufferOut(BufferOut::new(program, unsafe { ptr.as_audio_buffer() }))
            }
            VBVMR_CBCOMMAND::BUFFER_MAIN => {
                Self::BufferMain(BufferMain::new(unsafe { ptr.as_audio_buffer() }))
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
