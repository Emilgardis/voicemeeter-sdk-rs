use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};
use std::env;
use std::io::Read;
use std::path::PathBuf;

#[test]
#[ignore]
/// Generates and assures that a new invokation of bindgen will provide the same output for the bindings.
fn assure_accurate_binding() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let header = dir.join("../sdk/VoicemeeterRemote.h");
    let bindings = bindgen::Builder::default()
        .header(header.display().to_string())
        .raw_line(
            "#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unaligned_references)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(deref_nullptr)]
#![allow(clippy::missing_safety_doc)]
#![allow(non_snake_case)]",
        )
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        .blocklist_function("VBVMR_Local.*")
        .blocklist_function("VBVMR_GetRequestVB0STREAMPTR")
        .blocklist_function("VBVMR_SetParametersWEx")
        .blocklist_function("VBVMR_LoginEx")
        .blocklist_function("VBVMR_MB_PushSettings")
        .dynamic_library_name("VoicemeeterRemoteRaw")
        .dynamic_link_require_all(true)
        .generate()
        .expect("unable to generate bindings");
    let path = dir.join("../src/bindings.rs");
    if env::var("BLESS").is_ok() {
        bindings
            .write_to_file(&path)
            .expect("Couldn't write bindings!");
    } else {
        let mut current = std::fs::File::open(&path).unwrap();
        let mut curr = String::new();
        current.read_to_string(&mut curr).unwrap();
        assert_str_eq!(curr, bindings.to_string());
    }
}
