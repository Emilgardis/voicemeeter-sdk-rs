use std::io::Write;

pub fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    let remote = voicemeeter::VoicemeeterRemote::new()?;
    dbg!(remote.is_parameters_dirty()?);
    dbg!(remote.is_macrobutton_dirty()?);
    dbg!(remote.parameters().option().sr().get()?);
    dbg!(remote.is_parameters_dirty()?);
    dbg!(remote.parameters().bus(0)?.mode().get()?);
    //dbg!(remote.parameters().bus(0)?.mode().set_normal(false)?);
    std::thread::sleep(std::time::Duration::from_millis(500));
    //dbg!(remote.run_voicemeeter(voicemeeter::types::VoicemeeterApplication::PotatoX64Bits)?);
    println!("{}", remote.get_voicemeeter_version()?);
    let mut stdout = std::io::stdout();
    let mut c = 0;
    let val: u32 = std::env::args().nth(1).unwrap_or("0".to_owned()).parse()?;
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
            }
            e => {
                e?;
            }
        }
        match remote.is_parameters_dirty() {
            Ok(true) => {
                dbg!(remote.parameters().bus(0)?.eq(0).gain(0).get()?);
                dbg!(remote.parameters().strip(0)?.eq(0)?.gain(0).get()?);
            }
            e => {
                e?;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
