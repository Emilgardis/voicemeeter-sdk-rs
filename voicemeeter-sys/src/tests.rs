use std::env;
use std::io::Read;
use std::path::PathBuf;

#[test]
#[ignore]
/// Generates and assures that a new invokation of bindgen will provide the same output for the bindings.
fn assure_accurate_binding() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let header = dir.join("sdk/VoicemeeterRemote.h");
    let bindings = bindgen::Builder::default()
        .header(header.display().to_string())
        .raw_line(
            "#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unaligned_references)]
#![allow(deref_nullptr)]
#![allow(clippy::missing_safety_doc)]
#![allow(non_snake_case)]",
        )
        .dynamic_library_name("VoicemeeterRemote")
        .dynamic_link_require_all(true)
        .generate()
        .expect("unable to generate bindings");
    let path = dir.join("src/bindings.rs");
    if env::var("BLESS").is_ok() {
        bindings
            .write_to_file(&path)
            .expect("Couldn't write bindings!");
    } else {
        let mut current = std::fs::File::open(&path).unwrap();
        let mut curr = String::new();
        current.read_to_string(&mut curr).unwrap();
        assert_eq!(curr, bindings.to_string());
    }
}
