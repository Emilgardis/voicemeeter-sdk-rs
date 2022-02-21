pub fn main() -> Result<(), color_eyre::Report> {
    let remote = voicemeeter::VoicemeeterRemote::new()?;
    std::thread::sleep_ms(1000);
    dbg!(remote.get_output_device(64i32)?);
    dbg!(remote.is_parameters_dirty()?);
    //dbg!(remote.run_voicemeeter(voicemeeter::types::VoicemeeterApplication::PotatoX64Bits)?);
    println!("{}", remote.get_voicemeeter_version()?);
    println!(
        "float thing aaa {}",
        remote.get_parameter_float("Strip[0].Mono")?
    );
    loop {
        match remote.is_parameters_dirty() {
            Ok(d) => {
                if d {
                    println!(
                        "float thing aaa {}",
                        remote.get_parameter_float("Strip[0].Mono")?
                    );
                    //dbg!(remote.is_parameters_dirty()?);
                    std::thread::sleep_ms(20);
                }
            }
            e => {
                std::thread::sleep_ms(20);
                e?;
            }
        }
    }
    Ok(())
}
