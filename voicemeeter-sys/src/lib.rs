//! FFI Bindings to Voicemeeter SDK

pub mod bindings;
#[cfg(test)]
mod codegen;

use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::{env, io};

pub static VOICEMEETER_REMOTE: once_cell::sync::OnceCell<VoicemeeterRemote> =
    once_cell::sync::OnceCell::new();

#[doc(inline)]
pub use bindings::VoicemeeterRemote;

use winreg::enums::{KEY_READ, KEY_WOW64_32KEY};

static INSTALLER_UNINST_KEY: &str = "VB:Voicemeeter {17359A74-1236-5467}";
static UNINSTALLER_DIR: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
static DEFAULT_PATH: &str = "C:\\Program Files (x86)\\VB\\Voicemeeter";
static LIBRARY_NAME_64: &str = "VoicemeeterRemote64.dll";
static LIBRARY_NAME_32: &str = "VoicemeeterRemote.dll";

/// Get a reference to voicemeeter remote
pub fn get_voicemeeter() -> Result<&'static VoicemeeterRemote, LoadError> {
    if let Some(remote) = VOICEMEETER_REMOTE.get() {
        Ok(remote)
    } else {
        let path = find_voicemeeter_remote_with_folder()
            .or_else(|_| find_voicemeeter_remote_with_registry())?;
        load_voicemeeter_from_path(&path)
    }
}

/// Load voicemeeter
///
/// Errors if it's already loaded
pub fn load_voicemeeter_from_path(path: &OsStr) -> Result<&'static VoicemeeterRemote, LoadError> {
    VOICEMEETER_REMOTE
        .set(unsafe { VoicemeeterRemote::new(path)? })
        .map_err(|_| LoadError::AlreadyLoaded)?;
    unsafe { Ok(VOICEMEETER_REMOTE.get_unchecked()) }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("library is already loaded")]
    AlreadyLoaded,
    #[error("library could not be loaded")]
    LoadingError(#[from] libloading::Error),
    #[error("library could not be located")]
    RemoteFileError(#[from] RemoteFileError),
}

/// Get VoiceMeeterRemote via a path given by environment key `VOICEMEETER_FOLDER` or it's "default" installation path
pub fn find_voicemeeter_remote_with_folder() -> Result<OsString, RemoteFileError> {
    let path = if let Ok(folder) = env::var("VOICEMEETER_FOLDER") {
        Path::new(&folder).join(LIBRARY_NAME_64)
    } else {
        Path::new(DEFAULT_PATH).join(LIBRARY_NAME_64)
    };
    if path.exists() {
        Ok(path.into_os_string())
    } else {
        Err(RemoteFileError::NotFound(path.display().to_string()))
    }
}

/// Get VoiceMeeterRemote via registry key
pub fn find_voicemeeter_remote_with_registry() -> Result<OsString, RemoteFileError> {
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
    let remote = Path::new(&path)
        .parent()
        .ok_or(RegistryError::UninstallStringInvalid(path.clone()))
        .map(|p| p.join(LIBRARY_NAME_64))?;

    if remote.exists() {
        Ok(remote.into_os_string())
    } else {
        Err(RemoteFileError::NotFound(remote.display().to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RemoteFileError {
    #[error("could not find voicemeeter folder: {}", 0)]
    // TODO: OsString?
    NotFound(String),
    #[error(transparent)]
    RegistryError(#[from] RegistryError),
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
