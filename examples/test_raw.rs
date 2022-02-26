

pub fn main() -> Result<(), color_eyre::Report> {
    let rem = voicemeeter::get_voicemeeter_raw()?;

    unsafe {
        println!("login? {}", rem.VBVMR_Login());
        let res = rem.VBVMR_Output_GetDeviceNumber();
        println!("{res}");
    }
    Ok(())
}
