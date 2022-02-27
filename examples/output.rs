#![feature(let_else)]
use std::time::Duration;

use voicemeeter::{
    interface::callback::commands::{BufferOut, BufferOutData, HasAudioBuffer},
    types::Channel,
    AudioCallbackMode, CallbackCommand,
};

pub fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("trace")),
        )
        .with_writer(std::io::stderr)
        .with_file(true)
        .with_line_number(true)
        .compact()
        .init();
    let remote = voicemeeter::VoicemeeterRemote::new()?;
    println!("{}", remote.get_voicemeeter_version()?);
    let mut printed = false;
    let mut frame = 0u64;
    remote.audio_callback_register(AudioCallbackMode::MAIN, "TESTing", |command, _| {
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
            CallbackCommand::BufferMain(mut data) => {
                // unsafe {
                //     let data = unsafe { data.buffer.data() };
                //     let read = data.0; // std::slice::from_raw_parts_mut(data.0[0], 1024);
                //     let base_offset = std::ptr::addr_of!(read[0]);
                //     println!("{:?}", read.iter().map(|x| format!("{0:p}: {1:?}",  x, x.as_ref())).collect::<Vec<_>>());
                // }
                let buffer = &mut data.buffer;
                for channel in Channel::potato_channels() {
                    let channel = &channel;
                    let (buffer_in, buffer_out) = match buffer.read_write_buffer_on_channel(channel)
                    {
                        Some(b) => b,
                        None => continue,
                    };
                    for (write, read) in buffer_out.iter_mut().zip(buffer_in.iter()) {
                        write.copy_from_slice(read)
                    }
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
