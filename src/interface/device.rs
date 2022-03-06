//! Device related interfaces.
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
    /// Get the desctiption of a specific Audio Input Device.
    pub fn get_input_device(
        &self,
        index: impl Into<ZIndex>,
    ) -> Result<InputDevice, GetDeviceError> {
        let mut r#type = 0;
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
            r#type: VBVMR_DEVTYPE(r#type),
            name: unsafe { CStr::from_ptr(ptr::addr_of!(name[0])) }
                .to_string_lossy()
                .into_owned(),
            hardware_id: unsafe { CStr::from_ptr(ptr::addr_of!(hardware_id[0])) }
                .to_string_lossy()
                .into_owned(),
        })
    }

    pub(crate) unsafe fn _get_input_device(
        &self,
        index: i32,
        r#type: Option<&mut i32>,
        name: Option<&mut [c_char; 256]>,
        hardware_id: Option<&mut [c_char; 256]>,
    ) -> Result<(), GetDeviceError> {
        let null_i32 = ptr::null_mut();
        let r#type = if let Some(mut p) = r#type {
            ptr::addr_of_mut!(p)
        } else {
            null_i32
        };
        let null_c = ptr::null_mut();
        let name_p = if let Some(p) = name {
            ptr::addr_of_mut!(p[0])
        } else {
            null_c
        };
        let hardware_id_p = if let Some(p) = hardware_id {
            ptr::addr_of_mut!(p[0])
        } else {
            null_c
        };

        let res = unsafe {
            self.raw
                .VBVMR_Input_GetDeviceDescA(index, r#type as *mut _, name_p, hardware_id_p)
        };
        //cleanup
        match res {
            0 => Ok(()),
            s => Err(GetDeviceError(s)),
        }
    }

    /// Get the desctiption of a specific Audio Output Device.
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
            r#type: VBVMR_DEVTYPE(r#type),
            name: unsafe { CStr::from_ptr(ptr::addr_of!(name[0])) }
                .to_string_lossy()
                .into_owned(),
            hardware_id: unsafe { CStr::from_ptr(ptr::addr_of!(hardware_id[0])) }
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
        let r#type = crate::opt_or_null(r#type);
        let name_p = crate::opt_or_null(name.map(|a| &mut a[0]));
        let hardware_id_p = crate::opt_or_null(hardware_id.map(|a| &mut a[0]));
        let res = unsafe {
            self.raw
                .VBVMR_Output_GetDeviceDescA(index, r#type as *mut _, *name_p, *hardware_id_p)
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
    pub r#type: bindings::VBVMR_DEVTYPE,
    /// Device name
    pub name: String,
    /// Hardware ID
    pub hardware_id: String,
}

/// A Audio Output Device.
#[derive(Debug)]
pub struct OutputDevice {
    /// The type of the device.
    pub r#type: bindings::VBVMR_DEVTYPE,
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
