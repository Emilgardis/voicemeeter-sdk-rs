#![feature(is_some_with)]
use std::io::Write;
use voicemeeter::types::LogicalButton;
pub fn main() -> Result<(), color_eyre::Report> {
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
                );
                writeln!(
                    stdout,
                    "Button 1: {}, {}",
                    remote.get_macrobutton_state(1u32)?,
                    remote.get_macrobutton_trigger_state(1u32)?,
                );
                writeln!(stdout, "--- {}", c);
                c += 1;
                std::thread::sleep_ms(100);
            }
            e => {
                std::thread::sleep_ms(100);
                e?;
            }
        }
    }
    Ok(())
}
