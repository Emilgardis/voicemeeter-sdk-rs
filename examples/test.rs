use std::{ptr::{self, null}, os::raw::c_char};

use voicemeeter::voicemeeter_sys as vm_sys;

pub fn main() -> Result<(), color_eyre::Report> {
    let rem = vm_sys::get_voicemeeter()?;
    
    unsafe {
        //println!("login? {}", rem.VBVMR_Login());
        //println!("login? {}", rem.VBVMR_Login());
        let mut ver: i32 = 0;
         rem.VBVMR_GetVoicemeeterVersion(&mut ver);
         println!("version: {:x}", ver);
         let mut devname = [0i8;256];
         let res = rem.VBVMR_Output_GetDeviceDescA(0, &mut ver, ptr::addr_of_mut!(devname[0]), ptr::null_mut());
         println!("{res}");
         println!("devtyppe: {:x}", ver);
         println!("devname: {:?}", std::ffi::CStr::from_ptr(ptr::addr_of!(devname[0])))
    }
    Ok(())
}