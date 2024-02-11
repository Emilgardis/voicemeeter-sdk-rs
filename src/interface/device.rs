//! Device related interfaces.
//!
//! # Functions
//!
//! * [`get_total_input_device`](VoicemeeterRemote::get_total_input_device)
//! * [`get_total_output_device`](VoicemeeterRemote::get_total_output_device)
//! * [`get_input_device`](VoicemeeterRemote::get_input_device)
//! * [`get_output_device`](VoicemeeterRemote::get_output_device)
use std::{ffi::CStr, os::raw::c_char, ptr};

use crate::{
    bindings::{self, VBVMR_DEVTYPE},
    types::ZIndex,
};

use super::VoicemeeterRemote;

impl VoicemeeterRemote {
    /// Get the number of Audio Input Devices available on the system.
    pub fn get_total_input_device(&self) -> Result<i32, GetTotalDeviceError> {
        let res = unsafe { self.raw.VBVMR_Input_GetDeviceNumber() };
        if res < 0 {
            Err(GetTotalDeviceError(res))
        } else {
            Ok(res)
        }
    }
    /// Get the number of Audio Output Devices available on the system.
    pub fn get_total_output_device(&self) -> Result<i32, GetTotalDeviceError> {
        let res = unsafe { self.raw.VBVMR_Output_GetDeviceNumber() };
        if res < 0 {
            Err(GetTotalDeviceError(res))
        } else {
            Ok(res)
        }
    }
    /// Get the description of a specific Audio Input Device.
    pub fn get_input_device(
        &self,
        index: impl Into<ZIndex>,
    ) -> Result<InputDevice, GetDeviceError> {
        let mut r#type = -1;
        let index = index.into().0;
        let mut name = [0 as c_char; 256];
        let mut hardware_id = [0 as c_char; 256];
        unsafe {
            self._get_input_device(
                index,
                Some(&mut r#type),
                Some(&mut name),
                Some(&mut hardware_id),
            )?;
        }
        Ok(InputDevice {
            r#type: DeviceType::from(r#type),
            name: unsafe { CStr::from_ptr(name.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            hardware_id: unsafe { CStr::from_ptr(hardware_id.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
        })
    }

    pub(crate) unsafe fn _get_input_device(
        &self,
        index: i32,
        mut r#type: Option<&mut i32>,
        name: Option<&mut [c_char; 256]>,
        hardware_id: Option<&mut [c_char; 256]>,
    ) -> Result<(), GetDeviceError> {
        let type_p = crate::opt_or_null(r#type);
        let name_p = crate::opt_or_null(name.map(|a| &mut a[0]));
        let hardware_id_p = crate::opt_or_null(hardware_id.map(|a| &mut a[0]));

        let res = unsafe {
            self.raw
                .VBVMR_Input_GetDeviceDescA(index, type_p, name_p, hardware_id_p)
        };
        //cleanup
        match res {
            0 => Ok(()),
            s => Err(GetDeviceError(s)),
        }
    }

    /// Get the description of a specific Audio Output Device.
    pub fn get_output_device(
        &self,
        index: impl Into<ZIndex>,
    ) -> Result<OutputDevice, GetDeviceError> {
        let mut r#type = 0;
        let index = index.into().0;
        let mut name = [0 as c_char; 256];
        let mut hardware_id = [0 as c_char; 256];
        unsafe {
            self._get_output_device(
                index,
                Some(&mut r#type),
                Some(&mut name),
                Some(&mut hardware_id),
            )?;
        }
        Ok(OutputDevice {
            r#type: DeviceType::from(r#type),
            name: unsafe { CStr::from_ptr(name.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            hardware_id: unsafe { CStr::from_ptr(hardware_id.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
        })
    }

    pub(crate) unsafe fn _get_output_device(
        &self,
        index: i32,
        r#type: Option<&mut i32>,
        name: Option<&mut [c_char; 256]>,
        hardware_id: Option<&mut [c_char; 256]>,
    ) -> Result<(), GetDeviceError> {
        let type_p = crate::opt_or_null(r#type);
        let name_p = crate::opt_or_null(name.map(|a| &mut a[0]));
        let hardware_id_p = crate::opt_or_null(hardware_id.map(|a| &mut a[0]));
        let res = unsafe {
            self.raw
                .VBVMR_Output_GetDeviceDescA(index, type_p, name_p, hardware_id_p)
        };
        //cleanup
        match res {
            0 => Ok(()),
            s => Err(GetDeviceError(s)),
        }
    }
}

/// A Audio Input Device.
#[derive(Debug)]
pub struct InputDevice {
    /// The type of the device.
    pub r#type: DeviceType,
    /// Device name
    pub name: String,
    /// Hardware ID
    pub hardware_id: String,
}

/// Represents the type of an audio device.
#[repr(i32)]
#[derive(Debug)]
pub enum DeviceType {
    /// MME (Multimedia Extension) audio driver.
    Mme = VBVMR_DEVTYPE::MME.0,
    /// WDM (Windows Driver Model) audio driver.
    Wdm = VBVMR_DEVTYPE::WDM.0,
    /// KS (Kernel Streaming) audio driver.
    Ks = VBVMR_DEVTYPE::KS.0,
    /// ASIO (Audio Stream Input/Output) audio driver.
    Asio = VBVMR_DEVTYPE::ASIO.0,
    /// Other audio device types not explicitly defined.
    Other(VBVMR_DEVTYPE),
}

impl From<i32> for DeviceType {
    /// Converts an integer value to a DeviceType.
    fn from(value: i32) -> Self {
        match VBVMR_DEVTYPE(value) {
            VBVMR_DEVTYPE::MME => DeviceType::Mme,
            VBVMR_DEVTYPE::WDM => DeviceType::Wdm,
            VBVMR_DEVTYPE::KS => DeviceType::Ks,
            VBVMR_DEVTYPE::ASIO => DeviceType::Asio,
            o => DeviceType::Other(o),
        }
    }
}

/// A Audio Output Device.
#[derive(Debug)]
pub struct OutputDevice {
    /// The type of the device.
    pub r#type: DeviceType,
    /// Device name
    pub name: String,
    /// Hardware ID
    pub hardware_id: String,
}

/// Error when getting the device description.
#[derive(Debug, thiserror::Error, Clone)]
#[error("unexpected device: error code {0}")]
pub struct GetDeviceError(pub i32);

/// Error when getting the total devices.
#[derive(Debug, thiserror::Error, Clone)]
#[error("could not get total device number: error code {0}")]
pub struct GetTotalDeviceError(pub i32);
