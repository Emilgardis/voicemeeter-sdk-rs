use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use voicemeeter::types::Device;
use voicemeeter::{CallbackCommand, VoicemeeterRemote};

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
    // This is the callback command that will be called by voicemeeter on every audio frame, start, stop and change.
    let mut frame = 0;
    // This callback can capture data from its scope
    let callback = |command: CallbackCommand, _nnn: i32| -> i32 {
        match command {
            CallbackCommand::Starting(info) => println!("starting!\n{info:?}"),
            CallbackCommand::Ending(_) => println!("ending!"),
            CallbackCommand::Change(info) => println!("application change requested!\n{info:?}"),
            // In MAIN mode, this command is used for every audio frame.
            voicemeeter::CallbackCommand::BufferMain(mut data) => {
                frame += 1;
                // The data returned by voicemeeter is a slice of frames per "channel" containing another slice with `data.nbs` samples.
                // each device has a number of channels (e.g left, right, center, etc. typically 8 channels)
                for device in [Device::OutputA1, Device::OutputA2] {
                    // The `read_write_buffer_on_device` method on the buffer will return a slice of all channels for the given device.
                    let (buffer_in, buffer_out): (&[&[f32]], &mut [&mut [f32]]) =
                        match data.buffer.read_write_buffer_on_device(&device) {
                            Some(b) => b,
                            None => continue,
                        };
                    // If the input is as large as the output (which is always true for OutputA1 and OutputA2),
                    // write the input to the output
                    if buffer_out.len() == buffer_in.len() {
                        for (write, read) in buffer_out.iter_mut().zip(buffer_in.iter()) {
                            write.clone_from_slice(read);
                        }
                    }
                }
            }
            _ => (),
        }
        0
    };
    let guard =
        remote.audio_callback_register(voicemeeter::AudioCallbackMode::MAIN, "my_app", callback)?;
    // It is good practice to wait a bit here before starting the callback, otherwise you may experience some crackling.
    // Other reason for crackle is not quick enough execution, try running in release mode for optimizations.
    std::thread::sleep(std::time::Duration::from_millis(500));

    remote.audio_callback_start()?;

    while running.load(Ordering::SeqCst) {
        std::hint::spin_loop()
    }

    remote.audio_callback_unregister(guard)?;
    println!("total frames: {frame}");
    Ok(())
}
