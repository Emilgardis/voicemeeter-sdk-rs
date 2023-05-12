#![cfg(nightly)]
#![allow(clippy::missing_safety_doc, clippy::let_and_return, unused)]
#![feature(array_methods)]
//! Module for setting up miri interface. This is not intended to be used directly, but you could if wanted/needed.
use voicemeeter::{
    bindings::VBVMR_CBCOMMAND, interface::callback::data::RawCallbackData, types::Device, *,
};

/// Returns an audio_info, leaks!
pub unsafe fn audio_info() -> *mut std::os::raw::c_void {
    let ptr = Box::into_raw(Box::new(bindings::tagVBVMR_AUDIOINFO {
        samplerate: 44100,
        nbSamplePerFrame: 512,
    })) as _;
    #[cfg(miri)]
    miri_static_root(ptr as *const u8);
    ptr
}

/// Returns an audio_buffer, leaks!
pub unsafe fn audio_buffer(input: i32, output: i32) -> *mut std::os::raw::c_void {
    let ptr = Box::into_raw(Box::new(bindings::tagVBVMR_AUDIOBUFFER {
        audiobuffer_sr: 44100,
        audiobuffer_nbs: 512,
        audiobuffer_nbi: input,
        audiobuffer_nbo: output,
        audiobuffer_r: std::array::from_fn(|i| {
            if i < input as usize {
                make_channel()
            } else {
                std::ptr::null_mut()
            }
        }),
        audiobuffer_w: std::array::from_fn(|i| {
            if i < output as usize {
                make_channel()
            } else {
                std::ptr::null_mut()
            }
        }),
    })) as _;
    #[cfg(miri)]
    miri_static_root(ptr as *const u8);
    ptr
}

/// Returns an audio buffer of size 512 for a channel. Data is not random
pub unsafe fn make_channel() -> *mut f32 {
    let ptr = Box::leak(Box::new([0.0f32; 512])).as_mut_ptr();
    #[cfg(miri)]
    miri_static_root(ptr as *const u8);
    ptr
}

#[test]
fn test_audio_info() {
    unsafe {
        let info = audio_info();
        let info = info as *mut bindings::tagVBVMR_AUDIOINFO;
        assert_eq!(info.as_ref().unwrap().samplerate, 44100);
        assert_eq!(info.as_ref().unwrap().nbSamplePerFrame, 512);
    }
}

#[test]
fn test_audio_buffer() {
    unsafe {
        let buffer = audio_buffer(64, 64);
        let buffer = buffer as *mut bindings::tagVBVMR_AUDIOINFO;
        assert_eq!(buffer.as_ref().unwrap().samplerate, 44100);
        assert_eq!(buffer.as_ref().unwrap().nbSamplePerFrame, 512);
    }
}

#[test]
fn callback_stuff() {
    unsafe {
        let buffer = audio_buffer(98, 64);
        let command = CallbackCommand::new_unchecked(
            types::VoicemeeterApplication::PotatoX64Bits,
            VBVMR_CBCOMMAND::BUFFER_MAIN,
            RawCallbackData::from_ptr(buffer),
        );
        match command {
            CallbackCommand::BufferMain(data) => {
                let (read, mut write) = data.buffer.get_buffers();
                // Apply a function on all channels  of `OutputA1`.
                write.output_a1.apply_all_samples(
                    &read.output_a1,
                    |ch: usize, r: &f32, w: &mut f32| {
                        *w = *r;
                    },
                );
                // Apply another function on all channels of `OutputA2`.
                write
                    .output_a2
                    .apply(&read.output_a2, |_ch: usize, r: &[f32], w: &mut [f32]| {
                        w.copy_from_slice(r)
                    });
                // the buffer write type has a convenience method to
                // copy data for specified devices.
                write.copy_device_from(
                    &read,
                    &[
                        //Device::OutputA1,
                        //Device::OutputA2,
                        Device::OutputA3,
                        Device::OutputA4,
                        Device::OutputA5,
                        Device::VirtualOutputB1,
                        Device::VirtualOutputB2,
                        Device::VirtualOutputB3,
                    ],
                );
            }
            _ => panic!("should not be hit"),
        }
    }
}

#[cfg(miri)]
extern "Rust" {
    /// Miri-provided extern function to mark the block `ptr` points to as a "root"
    /// for some static memory. This memory and everything reachable by it is not
    /// considered leaking even if it still exists when the program terminates.
    ///
    /// `ptr` has to point to the beginning of an allocated block.
    fn miri_static_root(ptr: *const u8);

    /// Miri-provided extern function to obtain a backtrace of the current call stack.
    /// This returns a boxed slice of pointers - each pointer is an opaque value
    /// that is only useful when passed to `miri_resolve_frame`
    /// The `flags` argument must be `0`.
    fn miri_get_backtrace(flags: u64) -> Box<[*mut ()]>;

}
