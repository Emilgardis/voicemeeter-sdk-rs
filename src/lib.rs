#![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! Voicemeeter sdk
//!
//! Create a new instance of the Voicemeeter SDK. The instance is automatically logged in.
//!
//! ```rust,no_run
//! use voicemeeter::VoicemeeterRemote;
//!
//! let remote = VoicemeeterRemote::new()?;
//! println!("{}", remote.get_voicemeeter_version()?);
//!
//! Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#[allow(missing_docs)]
/// Raw FFI Bindings
#[allow(rustdoc::broken_intra_doc_links)]
pub mod bindings;
#[cfg(feature = "interface")]
pub mod interface;
#[cfg(feature = "interface")]
pub mod types;

use std::ffi::{OsStr, OsString};

use std::io;
use std::path::Path;

#[doc(hidden)]
#[cfg(feature = "interface")]
pub static VOICEMEETER_REMOTE: once_cell::sync::OnceCell<VoicemeeterRemoteRaw> =
    once_cell::sync::OnceCell::new();

#[doc(inline, hidden)]
pub use bindings::{VoicemeeterRemoteRaw, VBVMR_AUDIOCALLBACK as AudioCallbackMode};
#[doc(inline)]
#[cfg(feature = "interface")]
pub use interface::VoicemeeterRemote;

#[doc(inline)]
#[cfg(feature = "interface")]
pub use interface::callback::commands::CallbackCommand;

use winreg::enums::{KEY_READ, KEY_WOW64_32KEY};

static INSTALLER_UNINST_KEY: &str = "VB:Voicemeeter {17359A74-1236-5467}";
static UNINSTALLER_DIR: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
static LIBRARY_NAME_64: &str = "VoicemeeterRemote64.dll";

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
#[tracing::instrument]
fn load_voicemeeter_from_path(path: &OsStr) -> Result<&'static VoicemeeterRemoteRaw, LoadError> {
    tracing::debug!("loading voicemeeter");
    VOICEMEETER_REMOTE
        .set(unsafe { VoicemeeterRemoteRaw::new(path)? })
        .map_err(|_| LoadError::AlreadyLoaded)?;
    unsafe { Ok(VOICEMEETER_REMOTE.get_unchecked()) }
}

/// Load error while loading the Voicemeeter remote DLL
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// Remote is already loaded. Not a hard error.
    #[error("library is already loaded")]
    AlreadyLoaded,
    /// Error while loading the DLL.
    #[error("library could not be loaded")]
    LoadingError(#[from] libloading::Error),
    /// Could not locate the dll
    #[error("library could not be located")]
    RemoteFileError(#[from] RemoteFileError),
}

/// Get VoiceMeeterRemote via registry key
#[tracing::instrument]
pub(crate) fn find_voicemeeter_remote_with_registry() -> Result<OsString, RemoteFileError> {
    tracing::debug!("finding voicemeeter dll");
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
        .ok_or_else(|| RegistryError::UninstallStringInvalid(path.clone()))
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

/// Error while trying to get Voicemeeter location
#[derive(Debug, thiserror::Error)]
pub enum RemoteFileError {
    /// Voicemeeter dll not found at path
    #[error("could not find voicemeeter folder: {}", 0)]
    // TODO: OsString?
    NotFound(String),
    /// Registry error
    #[error(transparent)]
    RegistryError(#[from] RegistryError),
}

/// Registry errors
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    /// Could not find the uninstall folder in HKLM for 32-bit apps
    #[error("could not find uninstall folder in hklm")]
    CouldNotFindUninstallReg(#[source] io::Error),
    /// Could not find voicemeeter in uninstall registry
    #[error("could not find voicemeeter in registry. Is Voicemeeter installed?")]
    CouldNotFindVM(#[source] io::Error),
    /// Could not find voicemeeter uninstall string in registry
    #[error("could not find voicemeeter uninstall string")]
    CouldNotFindUninstallString,
    /// Given uninstall exe is not a valid path
    #[error("given uninstall exe is not a valid path: {:?}", 0)]
    UninstallStringInvalid(String),
}

/// Get a pointer to a `T` if option is [`Some`](Option::Some) or a null ptr if it's [`None`](Option::None)
#[cfg(feature = "interface")]
pub(crate) fn opt_or_null<T>(option: Option<&mut T>) -> *mut &mut T {
    if let Some(mut p) = option {
        std::ptr::addr_of_mut!(p)
    } else {
        std::ptr::null_mut()
    }
}
