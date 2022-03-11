use fundsp::hacker32::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use voicemeeter::types::Device;
use voicemeeter::{AudioCallbackMode, CallbackCommand, VoicemeeterRemote};

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

    let mut eq_l = pipe::<U10, _, _>(|i| bell_hz(1000.0 + 1000.0 * i as f32, 1.0, db_amp(0.0)));
    let mut eq_r = pipe::<U10, _, _>(|i| bell_hz(1000.0 + 1000.0 * i as f32, 1.0, db_amp(0.0)));
    for (c, (fc, gain, q)) in [
        (24, 6.4f32, 0.40f32),
        (207, -3.0, 0.38),
        (2423, 3.1, 2.15),
        (7066, 4.0, 4.67),
        (10444, 4.8, 1.72),
        (1223, -1.6, 3.98),
        (4543, 3.5, 5.15),
        (5576, -4.7, 3.96),
        (6408, 2.4, 5.86),
        (8553, 0.6, 3.68),
    ]
    .into_iter()
    .enumerate()
    {
        eq_l.node_mut(c).set_gain(db_amp(gain));
        eq_l.node_mut(c).set_center(fc as f32);
        eq_l.node_mut(c).set_q(q);
        eq_r.node_mut(c).set_gain(db_amp(gain));
        eq_r.node_mut(c).set_center(fc as f32);
        eq_r.node_mut(c).set_q(q);
    }

    let mut frame = 0;
    // This is the callback command that will be called by voicemeeter on every audio frame,
    // start, stop and change.
    // This callback can capture data from its scope
    let callback = |command: CallbackCommand, _nnn: i32| -> i32 {
        match command {
            CallbackCommand::Starting(info) => {
                eq_l.reset(Some(info.info.samplerate as f64));
                println!("starting!\n{info:?}")
            }
            CallbackCommand::Ending(_) => println!("ending!"),
            CallbackCommand::Change(info) => println!("application change requested!\n{info:?}"),
            voicemeeter::CallbackCommand::BufferOut(data) => {
                frame += 1;
                let (read, mut write) = data.buffer.get_buffers();
                write.copy_device_from(
                    &read,
                    remote
                        .program
                        .devices()
                        .iter()
                        .filter(|d| d != &&Device::OutputA1),
                );

                write
                    .output_a1
                    .apply_all_samples(&read.output_a1, |ch, r, w| {
                        if ch == 0 {
                            eq_l.tick(std::slice::from_ref(r), std::slice::from_mut(w))
                        } else if ch == 1 {
                            eq_r.tick(std::slice::from_ref(r), std::slice::from_mut(w))
                        } else {
                            *w = *r;
                        }
                    });
            }
            _ => (),
        }
        0
    };
    // Register the callback
    let guard = remote.audio_callback_register(
        // The mode can be multiple modes
        AudioCallbackMode::OUTPUT,
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
