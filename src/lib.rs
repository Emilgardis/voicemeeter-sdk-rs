#![deny(unsafe_op_in_unsafe_fn)]
///! Voicemeeter sdk
#[cfg(test)]
pub mod codegen;

pub mod bindings;
pub mod interface;
pub mod types;

use std::ffi::{OsStr, OsString};
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::{env, io};

pub static VOICEMEETER_REMOTE: once_cell::sync::OnceCell<VoicemeeterRemoteRaw> =
    once_cell::sync::OnceCell::new();

#[doc(inline, hidden)]
pub use bindings::VoicemeeterRemoteRaw;
#[doc(inline)]
pub use interface::VoicemeeterRemote;

use winreg::enums::{KEY_READ, KEY_WOW64_32KEY};

static INSTALLER_UNINST_KEY: &str = "VB:Voicemeeter {17359A74-1236-5467}";
static UNINSTALLER_DIR: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
static LIBRARY_NAME_64: &str = "VoicemeeterRemote64.dll";
static LIBRARY_NAME_32: &str = "VoicemeeterRemote.dll";

#[doc(hidden)]
/// Get a reference to voicemeeter remote
pub fn get_voicemeeter_raw() -> Result<&'static VoicemeeterRemoteRaw, LoadError> {
    if let Some(remote) = VOICEMEETER_REMOTE.get() {
        Ok(remote)
    } else {
        let path = find_voicemeeter_remote_with_registry()?;
        load_voicemeeter_from_path(&path)
    }
}

/// Load voicemeeter
///
/// Errors if it's already loaded
fn load_voicemeeter_from_path(path: &OsStr) -> Result<&'static VoicemeeterRemoteRaw, LoadError> {
    VOICEMEETER_REMOTE
        .set(unsafe { VoicemeeterRemoteRaw::new(path)? })
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

/// Get VoiceMeeterRemote via registry key
pub(crate) fn find_voicemeeter_remote_with_registry() -> Result<OsString, RemoteFileError> {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let voicemeeter_uninst = if let Ok(reg) = hklm
        .open_subkey(UNINSTALLER_DIR)
        .and_then(|s| s.open_subkey(INSTALLER_UNINST_KEY))
    {
        // TODO: This will almost always fail, esp. on 64bit systems.
        reg
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

#[test]
#[ignore]
fn registry_check() -> Result<(), RemoteFileError> {
    dbg!(find_voicemeeter_remote_with_registry()?);
    Ok(())
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

/// Get a pointer to a `T` if option is [`Some`](Option::Some) or a null ptr if it's [`None`](Option::None)
pub(crate) fn opt_or_null<T>(option: Option<&mut T>) -> *mut &mut T {
    if let Some(mut p) = option {
        std::ptr::addr_of_mut!(p)
    } else {
        std::ptr::null_mut()
    }
}
