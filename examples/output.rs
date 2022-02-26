use std::time::Duration;

use voicemeeter::{interface::callback::commands::HasAudioBuffer, CallbackCommand};

pub fn main() -> Result<(), color_eyre::Report> {
    let remote = voicemeeter::VoicemeeterRemote::new()?;
    println!("{}", remote.get_voicemeeter_version()?);
    let mut printed = false;
    remote.audio_callback_register_output("TESTing", |command, _| {
        if !printed {
            println!("WE*RE IN")
        };
        printed = true;
        match command {
            CallbackCommand::Starting(info) => {
                println!("Starting: {:#?}", info);
            }
            CallbackCommand::Ending(_) => todo!(),
            CallbackCommand::Change(_) => todo!(),
            CallbackCommand::BufferOut(data) => {
                //println!("{data:?}");
                unsafe {
                    let v = *data.buffer.audiobuffer_r[0];
                    println!("{v:?} ");
                }
            }
            b => todo!("not implemented for: {:?}", b.name()),
        }
        0
    })?;
    remote.audio_callback_start()?;
    std::thread::sleep(Duration::from_secs(10));
    remote.audio_callback_unregister()?;
    Ok(())
}
