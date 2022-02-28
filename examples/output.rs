#![feature(let_else)]
use ctrlc;
use tracing_subscriber::fmt::format::FmtSpan;
use std::sync::mpsc::channel;
use std::time::Duration;

use voicemeeter::{
    interface::callback::commands::{BufferOut, BufferOutData, HasAudioBuffer},
    types::Channel,
    AudioCallbackMode, CallbackCommand,
};

pub fn main() -> Result<(), color_eyre::Report> {
    install_eyre()?;
    install_tracing();
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
    let remote = voicemeeter::VoicemeeterRemote::new()?;
    println!("{}", remote.get_voicemeeter_version()?);
    let mut printed = false;
    remote.audio_callback_register(AudioCallbackMode::MAIN, "TESTing", move |command, _| {

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
                        tracing::debug!("{}", write.len());
                        write.copy_from_slice(read.as_ref())
                    }
                }
            }
            b => todo!("not implemented for: {:?}", b.name()),
        }
        0
    })?;

    remote.audio_callback_start()?;
    rx.recv().expect("Could not receive from channel.");
    remote.audio_callback_unregister()?;

    Ok(())
}

fn install_eyre() -> eyre::Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().add_default_filters().into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |pi| {
        tracing::error!("{}", panic_hook.panic_report(pi));
    }));
    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::NONE)
        .compact();
    #[rustfmt::skip]
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .map(|f| {
            f.add_directive("hyper=error".parse().expect("could not make directive"))
                .add_directive("h2=error".parse().expect("could not make directive"))
                .add_directive("rustls=error".parse().expect("could not make directive"))
                .add_directive("tungstenite=error".parse().expect("could not make directive"))
                .add_directive("retainer=info".parse().expect("could not make directive"))
            //.add_directive("tower_http=error".parse().unwrap())
        })
        .expect("could not make filter layer");

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
