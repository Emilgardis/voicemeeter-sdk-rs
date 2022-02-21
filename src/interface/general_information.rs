use crate::types::VoicemeeterApplication;

use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    pub fn get_voicemeeter_type(
        &self,
    ) -> Result<VoicemeeterApplication, GetVoicemeeterInformationError> {
        let mut t = 0i32;
        let res = unsafe { self.raw.VBVMR_GetVoicemeeterType(&mut t) };
        match res {
            0 => Ok(VoicemeeterApplication::from(t)),
            -1 => Err(GetVoicemeeterInformationError::CannotGetClient),
            -2 => Err(GetVoicemeeterInformationError::NoServer),
            s => Err(GetVoicemeeterInformationError::Other(s)),
        }
    }
    pub fn get_voicemeeter_version(
        &self,
    ) -> Result<VoicemeeterVersion, GetVoicemeeterInformationError> {
        let mut t = 0i32;
        let res = unsafe { self.raw.VBVMR_GetVoicemeeterVersion(&mut t) };
        match res {
            0 => {
                let a: [u8; 4] = bytemuck::cast(t);
                Ok(VoicemeeterVersion(a[3], a[2], a[1], a[0]))
            }
            -1 => Err(GetVoicemeeterInformationError::CannotGetClient),
            -2 => Err(GetVoicemeeterInformationError::NoServer),
            s => Err(GetVoicemeeterInformationError::Other(s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VoicemeeterVersion(pub u8, pub u8, pub u8, pub u8);

impl std::fmt::Display for VoicemeeterVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum GetVoicemeeterInformationError {
    #[error("cannot get client (unexpected)")]
    CannotGetClient,
    #[error("no server")]
    NoServer,
    #[error("unexpected error occurred: error code {0}")]
    Other(i32),
    #[error("got an unexpected response")]
    InvalidResponse(String),
}
