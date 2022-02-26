use crate::bindings::VBVMR_CBCOMMAND;

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
        $(
            impl HasAudioBuffer for $name<'_> {
                fn get(&self) -> &AudioBuffer {
                    self.buffer
                }
                fn get_mut(&mut self) -> &mut AudioBuffer {
                    self.buffer
                }
            }
        )*
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
    pub buffer: &'a mut AudioBuffer,
}

impl<'a> BufferIn<'a> {
    pub fn new(buffer: &'a mut AudioBuffer) -> Self {
        Self { buffer }
    }
}

#[derive(Debug)]
pub struct BufferOut<'a> {
    pub buffer: &'a mut AudioBuffer,
}

impl<'a> BufferOut<'a> {
    pub fn new(buffer: &'a mut AudioBuffer) -> Self {
        Self { buffer }
    }
}

#[derive(Debug)]
pub struct BufferMain<'a> {
    pub buffer: &'a mut AudioBuffer,
}

impl<'a> BufferMain<'a> {
    pub fn new(buffer: &'a mut AudioBuffer) -> Self {
        Self { buffer }
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
    pub(crate) unsafe fn new_unchecked(command: VBVMR_CBCOMMAND, ptr: RawCallbackData) -> Self {
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
                Self::BufferOut(BufferOut::new(unsafe { ptr.as_audio_buffer() }))
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
