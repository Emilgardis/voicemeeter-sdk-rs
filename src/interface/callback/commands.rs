//! Callback command types.
//!
//! These are returned to the callback function.
use crate::{bindings::VBVMR_CBCOMMAND, types::VoicemeeterApplication};

use super::data::{
    AudioBuffer, AudioInfo, BufferInData, BufferMainData, BufferOutData, RawCallbackData,
};

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

/// Data for output mode.
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

/// Data for main mode.
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

/// Callback command passed to the [audio callback](crate::VoicemeeterRemote::audio_callback_register).
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
