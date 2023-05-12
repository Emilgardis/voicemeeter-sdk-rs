use voicemeeter::{AudioCallbackMode, CallbackCommand, VoicemeeterRemote};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the client.
    let remote = VoicemeeterRemote::new()?;

    let guard = remote.audio_callback_register(
        // The mode can be multiple modes
        AudioCallbackMode::MAIN | AudioCallbackMode::OUTPUT,
        "my_app",
        |command: CallbackCommand, _: i32| -> i32 {
            match command {
                // Receive all I/O to record or process it and replace or mix to BUS outputs.rocess audio outputs before master section.
                voicemeeter::CallbackCommand::BufferMain(data) => {
                    // do something with the data, this just simply relays it without any processing
                    let (read, mut write) = data.buffer.get_buffers();
                    write.copy_device_from(&read, remote.program.devices());
                }
                // Process audio outputs before master section.
                voicemeeter::CallbackCommand::BufferOut(data) => {
                    // do something with the data, this just simply relays it without any processing
                    let (read, mut write) = data.buffer.get_buffers();
                    write.copy_device_from(&read, remote.program.devices());
                }
                _ => {}
            }
            0
        },
    )?;

    // It is good practice to wait a bit here before starting the callback,
    // otherwise you may experience some crackling. Another reason
    // for crackle is not quick enough execution, try running in release mode for optimizations.
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("starting callback");
    remote.audio_callback_start()?;

    std::thread::sleep(std::time::Duration::from_millis(10000));

    println!("stopping callback");
    remote.audio_callback_unregister(guard)?;
    Ok(())
}
