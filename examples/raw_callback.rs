fn main() {
    // get the dll and handle
    let vm = voicemeeter::get_voicemeeter_raw().unwrap();

    unsafe {
        if vm.VBVMR_Login() < 0 {
            panic!("Login failed");
        }
        if vm.VBVMR_IsParametersDirty() == 0 {
            let mut vm_type = 0i32;
            vm.VBVMR_GetVoicemeeterType(&mut vm_type);
            println!("Voicemeeter type: {}", vm_type);
        }

        let mut clientname = [0u8; 64];
        let name = "my testasdadssdasd";
        clientname[0..name.len()].copy_from_slice(name.as_bytes());

        let res = vm.VBVMR_AudioCallbackRegister(
            2 | 4,
            Some(callback),
            std::ptr::null_mut(),
            clientname.as_mut_ptr() as *mut _,
        );

        vm.VBVMR_AudioCallbackStart();

        loop {
            std::thread::yield_now();
        }
    }
}

unsafe extern "C" fn callback(
    user_data: *mut std::os::raw::c_void,
    command: std::os::raw::c_long,
    buffer: *mut std::os::raw::c_void,
    nnn: std::os::raw::c_long,
) -> std::os::raw::c_long {
    let ptr = voicemeeter::interface::callback::data::RawCallbackData::from_ptr(buffer);

    let callback = voicemeeter::interface::callback::CallbackCommand::new_unchecked(
        voicemeeter::types::VoicemeeterApplication::VoicemeeterPotato,
        voicemeeter::bindings::VBVMR_CBCOMMAND(command),
        ptr,
    );

    // just copy read to write.
    let devices = voicemeeter::types::Device::all();
    match callback {
        voicemeeter::CallbackCommand::BufferIn(b_in) => {
            let (read, mut write) = b_in.buffer.get_buffers();
            write.copy_device_from(&read, devices)
        }
        voicemeeter::CallbackCommand::BufferOut(out) => {
            let (read, mut write) = out.buffer.get_buffers();
            write.copy_device_from(&read, devices)
        },
        voicemeeter::CallbackCommand::BufferMain(main) => {
            let (read, mut write) = main.buffer.get_buffers();
            write.copy_device_from(&read, devices)
        },
        _ => {}
    }
    0
}
