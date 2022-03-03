#![feature(is_some_with)]
use std::io::Write;

pub fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    let remote = voicemeeter::VoicemeeterRemote::new()?;
    dbg!(remote.is_macrobutton_dirty()?);
    //dbg!(remote.run_voicemeeter(voicemeeter::types::VoicemeeterApplication::PotatoX64Bits)?);
    println!("{}", remote.get_voicemeeter_version()?);
    let mut stdout = std::io::stdout();
    let mut c = 0;
    let val: u32 = std::env::args().nth(1).unwrap().parse()?;
    remote.set_macrobutton_state(1, val == 1, false)?;
    loop {
        match remote.is_macrobutton_dirty() {
            Ok(true) => {
                writeln!(
                    stdout,
                    "Button 0: {}, {}",
                    remote.get_macrobutton_state(0u32)?,
                    remote.get_macrobutton_trigger_state(0u32)?,
                )?;
                writeln!(
                    stdout,
                    "Button 1: {}, {}",
                    remote.get_macrobutton_state(1u32)?,
                    remote.get_macrobutton_trigger_state(1u32)?,
                )?;
                writeln!(stdout, "--- {}", c)?;
                c += 1;
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            e => {
                std::thread::sleep(std::time::Duration::from_millis(100));
                e?;
            }
        }
    }
}
