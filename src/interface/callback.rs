//! Callback function
//!
//! See [`VoicemeeterRemote::audio_callback_register`](crate::VoicemeeterRemote::audio_callback_register) for how to register and use callbacks.
pub mod commands;
pub mod data;
pub mod register;
pub mod start_stop;

pub use commands::{
    BufferIn, BufferMain, BufferOut, CallbackCommand, HasAudioBuffer, HasAudioInfo,
};
pub use data::{BufferInData, BufferMainData, BufferOutData};
