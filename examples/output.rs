use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use voicemeeter::types::Device;
use voicemeeter::{AudioCallbackMode, CallbackCommand, VoicemeeterRemote};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // This is the callback command that will be called by voicemeeter on every audio frame,
    // start, stop and change.
    // This callback can capture data from its scope
    let callback = |command: CallbackCommand, _nnn: i32| -> i32 {
        match command {
            CallbackCommand::Starting(info) => println!("starting!\n{info:?}"),
            CallbackCommand::Ending(_) => println!("ending!"),
            CallbackCommand::Change(info) => println!("application change requested!\n{info:?}"),
            // Output mode modifies the 
            voicemeeter::CallbackCommand::BufferOut(mut data) => {
                frame += 1;
                // The `get_all_buffers` method returns all possible devices for the current application.
                let (read, mut write) = data.buffer.get_all_buffers();
                // Below, we check that output_a1 is available (should always be true) and then apply a function on it.
                if let (Some(read_a1), Some(ref mut write_a1)) = (read.output_a1, &mut write.output_a1) {
                    // `output_a1` gives us an array with 8 slices, each slice is a channel with `data.nbs` samples.
                    for (read_a1_channel, write_a1_channel) in
                        read_a1.iter().zip(write_a1.iter_mut())
                    {
                        write_a1_channel
                            .iter_mut()
                            .enumerate()
                            .map(|(i, f)| *f = read_a1_channel[i] * 0.5) // lower each sample by a half
                            .for_each(drop);
                    }
                }

                // the buffer write type has a convenience method to copy data for specified devices.
                write.copy_device_from(
                    &read,
                    [
                        //Device::OutputA1,
                        Device::OutputA2,
                        Device::OutputA3,
                        Device::OutputA4,
                        Device::OutputA5,
                        Device::VirtualOutputB1,
                        Device::VirtualOutputB2,
                        Device::VirtualOutputB3,
                    ],
                )
            }
            // MAIN mode, this command is used for every audio frame and acts like a main i/o hub for the audio.
            // Below example will show how to use devices without `get_all_buffers`
            voicemeeter::CallbackCommand::BufferMain(mut data) => {
                // The data returned by voicemeeter is a slice of frames per "channel"
                // containing another slice with `data.nbs` samples.
                // Each device has a number of channels
                // (e.g left, right, center, etc. typically 8 channels)
                for device in [Device::OutputA1, Device::OutputA2] {
                    // The `read_write_buffer_on_device` method on the buffer will return
                    // a slice of all channels for the given device.
                    let (buffer_in, buffer_out): (&[&[f32]], &mut [&mut [f32]]) =
                        match data.buffer.read_write_buffer_on_device(&device) {
                            Some(b) => b,
                            None => continue,
                        };
                    // If the input is as large as the output
                    // (which is always true for OutputA1 and OutputA2),
                    if buffer_out.len() == buffer_in.len() {
                        for (read, write) in buffer_in.iter().zip(buffer_out.iter_mut()) {
                            // Write the input to the output
                            write.copy_from_slice(read);
                        }
                    }
                }
                // Instead of the above, the equivalent with convenience functions would be
                let (read, mut write) = data.buffer.get_all_buffers();
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
