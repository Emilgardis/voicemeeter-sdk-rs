use std::time::Duration;

pub fn main() -> Result<(), color_eyre::Report> {
    let remote = voicemeeter::VoicemeeterRemote::new()?;
    println!("{}", remote.get_voicemeeter_version()?);
    let mut printed = false;
    remote.audio_callback_register_output("TESTing", |command, _b, _a| {
        if !printed {
            println!("WE*RE IN")
        };
        printed = true;
        match command {
            voicemeeter::types::CallbackCommand::Starting => println!("Starting"),
            voicemeeter::types::CallbackCommand::Ending => println!("Ending"),
            voicemeeter::types::CallbackCommand::Change => println!("Change"),
            //voicemeeter::types::CallbackCommand::BufferIn => println!("BufferIn"),
            //voicemeeter::types::CallbackCommand::BufferOut => println!("BufferOut"),
            voicemeeter::types::CallbackCommand::BufferMain => println!("BufferMain"),
            _ => (),
        }
        0
    })?;
    remote.audio_callback_start()?;
    std::thread::sleep(Duration::from_secs(10));
    remote.audio_callback_unregister()?;
    Ok(())
}
