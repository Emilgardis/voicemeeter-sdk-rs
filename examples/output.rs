use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use voicemeeter::types::Device;
use voicemeeter::{AudioCallbackMode, CallbackCommand, DeviceBuffer, VoicemeeterRemote};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();
    // Setup a hook for catching ctrl+c to properly stop the program.
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Get the client.
    let remote = VoicemeeterRemote::new()?;

    let mut frame = 0;
    let mut sample_rate = None;
    let mut sine_r_phase = 0.;
    // A simple sine generator
    let mut sine_r = |sr: f32| {
        sine_r_phase = (sine_r_phase + 440.0 * 1.0 / sr).fract();
        std::f32::consts::TAU * sine_r_phase
    };

    // This is the callback command that will be called by voicemeeter on every audio frame,
    // start, stop and change.
    // This callback can capture data from its scope
    let callback = |command: CallbackCommand, _nnn: i32| -> i32 {
        match command {
            CallbackCommand::Starting(info) => {
                sample_rate = Some(info.info.samplerate);
                println!("starting!\n{info:?}")
            }
            CallbackCommand::Ending(_) => println!("ending!"),
            CallbackCommand::Change(info) => println!("application change requested!\n{info:?}"),
            // Output mode modifies the
            voicemeeter::CallbackCommand::BufferOut(data) => {
                frame += 1;
                // The `get_buffers` method gives the read and write buffers in a tuple.
                let (read, mut write) = data.buffer.get_buffers();
                // Apply a function on all channels  of `OutputA1`.
                write.output_a1.apply_all_samples(
                    &read.output_a1,
                    |ch: usize, r: &f32, w: &mut f32| {
                        // if right
                        if ch == 0 {
                            *w = sine_r(sample_rate.unwrap() as f32);
                        // otherwise
                        } else {
                            *w = *r;
                        }
                    },
                );
                // Apply another function on all channels of `OutputA2`.
                write.output_a2.apply(
                    &read.output_a2,
                    |_ch: usize, r: &[f32], w: &mut [f32]| {
                        w.copy_from_slice(r)
                    },
                );
                // the buffer write type has a convenience method to copy data for specified devices.
                write.copy_device_from(
                    &read,
                    [
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
            // The MAIN command acts like a main i/o hub for all audio.
            // Below example will show how to use devices without `get_all_buffers`
            voicemeeter::CallbackCommand::BufferMain(mut data) => {
                // The data returned by voicemeeter is a slice of frames per "channel"
                // containing another slice with `data.nbs` samples.
                // Each device has a number of channels
                // (e.g left, right, center, etc. typically 8 channels)
                for device in remote.program.devices() {
                    // The `read_write_buffer_on_device` method on the buffer will return
                    // a slice of all channels for the given device.
                    let (buffer_in, buffer_out): (&[&[f32]], &mut [&mut [f32]]) = match (
                        data.buffer.read.device(device),
                        data.buffer.write.device_mut(device),
                    ) {
                        (DeviceBuffer::Buffer(r), DeviceBuffer::Buffer(w)) => (r, w),
                        _ => continue,
                    };
                    // If the input is as large as the output
                    // (which should always be true)
                    if buffer_out.len() == buffer_in.len() {
                        for (read, write) in buffer_in.iter().zip(buffer_out.iter_mut()) {
                            // Write the input to the output
                            write.copy_from_slice(read);
                        }
                    }
                }

                // Instead of the above, the equivalent with convenience functions would be
                let (read, mut write) = data.buffer.get_buffers();
                write.copy_device_from(&read, remote.program.devices());
            }
            _ => (),
        }
        0
    };
    // Register the callback
    let guard = remote.audio_callback_register(
        // The mode can be multiple modes
        AudioCallbackMode::MAIN | AudioCallbackMode::OUTPUT,
        "my_app",
        callback,
    )?;
    // It is good practice to wait a bit here before starting the callback,
    // otherwise you may experience some crackling. Another reason
    // for crackle is not quick enough execution, try running in release mode for optimizations.
    std::thread::sleep(std::time::Duration::from_millis(500));

    remote.audio_callback_start()?;

    while running.load(Ordering::SeqCst) {
        std::hint::spin_loop()
    }

    remote.audio_callback_unregister(guard)?;
    println!("total frames: {frame}");
    Ok(())
}
