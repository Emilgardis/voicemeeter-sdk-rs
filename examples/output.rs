use std::time::Duration;

use voicemeeter::{
    interface::callback::commands::HasAudioBuffer, AudioCallbackMode, CallbackCommand,
};

pub fn main() -> Result<(), color_eyre::Report> {
    let remote = voicemeeter::VoicemeeterRemote::new()?;
    let program = remote.get_voicemeeter_type()?;
    println!("{}", remote.get_voicemeeter_version()?);
    let mut printed = false;
    let mut frame = 0u64;
    remote.audio_callback_register(AudioCallbackMode::OUTPUT, "TESTing", |command, _| {
        if !printed {
            println!("WE*RE IN")
        };
        printed = true;
        match command {
            CallbackCommand::Starting(info) => {
                println!("Starting: {:#?}", info);
            }
            CallbackCommand::Ending(_) => println!("good bye!"),
            CallbackCommand::Change(_) => todo!(),
            CallbackCommand::BufferOut(mut data) => {
                frame += 1;
                //println!("{data:?}");
                let (buffer_in, mut buffer_out) = data
                    .buffer
                    .read_write_buffer_on_channel(voicemeeter::types::Channel::OutputA1)
                    .unwrap();
                println!("bufferlens: {}, {}", buffer_in.len(), buffer_out.len());
                for k in buffer_out.iter_mut().zip(buffer_in.iter()) {
                    let (write, read) = k;
                    write.copy_from_slice(read);
                }
            }
            b => todo!("not implemented for: {:?}", b.name()),
        }
        0
    })?;
    remote.audio_callback_start()?;
    std::thread::sleep(Duration::from_secs(10));
    remote.audio_callback_unregister()?;
    std::thread::sleep(Duration::from_secs(2));

    Ok(())
}
