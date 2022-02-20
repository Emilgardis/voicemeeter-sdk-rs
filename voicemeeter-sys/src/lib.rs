//! FFI Bindings to Voicemeeter SDK

pub mod bindings;
#[cfg(test)]
mod codegen;

use std::{
    env,
    ffi::OsString,
    io,
    path::{Path},
};

#[doc(inline)]
pub use bindings::VoicemeeterRemote;

use winreg::enums::{KEY_READ, KEY_WOW64_32KEY};

static INSTALLER_UNINST_KEY: &str = "VB:Voicemeeter {17359A74-1236-5467}";
static UNINSTALLER_DIR: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
static DEFAULT_PATH: &str = "C:\\Program Files (x86)\\VB\\Voicemeeter";

pub fn find_voicemeeter_folder() -> Result<OsString, DefaultFolderError> {
    let path = if let Ok(folder) = env::var("VOICEMEETER_FOLDER") {
        Path::new(&folder).to_owned()
    } else {
        Path::new(DEFAULT_PATH).to_owned()
    };
    if path.exists() {
        Ok(path.into_os_string())
    } else {
        Err(DefaultFolderError::NotFound(DEFAULT_PATH.to_string()))
    }
}

/// Get the voice meeter folder via registry key
pub fn find_voicemeeter_folder_registry() -> Result<OsString, RegistryError> {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let voicemeeter_uninst = if let Ok(reg) = hklm.open_subkey(UNINSTALLER_DIR) {
        reg.open_subkey(INSTALLER_UNINST_KEY)
            .map_err(RegistryError::CouldNotFindVM)?
    } else {
        hklm.open_subkey_with_flags(UNINSTALLER_DIR, KEY_READ | KEY_WOW64_32KEY)
            .map_err(RegistryError::CouldNotFindUninstallReg)?
            .open_subkey(INSTALLER_UNINST_KEY)
            .map_err(RegistryError::CouldNotFindVM)?
    };
    let path: String = voicemeeter_uninst
        .get_value("UninstallString")
        .map_err(|_| RegistryError::CouldNotFindUninstallString)?;
    Path::new(&path)
        .parent()
        .ok_or(RegistryError::UninstallStringInvalid(path.clone()))
        .map(|p| p.as_os_str().to_os_string())
}

#[derive(Debug, thiserror::Error)]
pub enum DefaultFolderError {
    #[error("could not find voicemeeter folder: {}", 0)]
    NotFound(String),
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("could not find uninstall folder in hklm")]
    CouldNotFindUninstallReg(#[source] io::Error),
    #[error("could not find voicemeeter in registry. Is Voicemeeter installed?")]
    CouldNotFindVM(#[source] io::Error),
    #[error("could not find voicemeeter uninstall string")]
    CouldNotFindUninstallString,
    #[error("given uninstall exe is not a valid path: {:?}", 0)]
    UninstallStringInvalid(String),
}
