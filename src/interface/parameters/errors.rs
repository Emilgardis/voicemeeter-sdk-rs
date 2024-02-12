use crate::types::ZIndex;

/// Parameter is out of range for current program
#[derive(thiserror::Error, Debug, Clone)]
#[error("out of range: {name}({index}) is not supported on `{program}`")]
pub struct OutOfRangeError {
    /// Name of the parameter `base` i.e "Strip" or "Bus"
    pub name: String,
    /// Index that was out of range
    pub index: ZIndex,
    /// Current program
    pub program: super::VoicemeeterApplication,
}

/// Device is invalid for the current program and parameter
#[derive(thiserror::Error, Debug, Clone)]
#[error("invalid device: {device:?} is not supported on `{program}`")]
#[non_exhaustive]
pub struct DeviceError {
    /// Current program
    pub program: super::VoicemeeterApplication,
    /// Device that was invalid
    pub device: crate::types::Device,
}

/// Invalid strip/bus type for a specific parameter
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum InvalidTypeError {
    /// Expected Physical
    #[error(
        "{name}[{strip_index}] needs to be a physical {name} for access to parameter `{parameter}`"
    )]
    ExpectedPhysical {
        /// Name of the parameter `base` i.e "Strip" or "Bus"
        name: &'static str,
        /// Index that was used
        strip_index: ZIndex,
        /// Parameter that expected a physical strip/bus
        parameter: String,
    },
    /// Expected Virtual
    #[error(
        "{name}[{strip_index}] needs to be a virtual {name} for access to parameter `{parameter}`"
    )]
    ExpectedVirtual {
        /// Name of the parameter `base` i.e "Strip" or "Bus"
        name: &'static str,
        /// Index that was used
        strip_index: ZIndex,
        /// Parameter that expected a physical strip/bus
        parameter: String,
    },
    /// Expected Strip
    #[error("expected a strip, got `{device}`")]
    ExpectedStrip {
        /// Device received
        device: String,
    },
    /// Expected Bus
    #[error("expected a bus, got `{device}`")]
    ExpectedBus {
        /// Device received
        device: String,
    },
}

/// Invalid Voicemeeter program
#[derive(thiserror::Error, Debug, Clone)]
#[error("{parameter} requires programs {expected:?} to be accessed, program is {found}")]
pub struct InvalidVoicemeeterVersion {
    /// Expected programs
    pub expected: &'static [super::VoicemeeterApplication],
    /// Found program
    pub found: super::VoicemeeterApplication,
    /// Parameter that expected a physical strip/bus
    pub parameter: String,
}

/// Invalid Parameter
#[derive(thiserror::Error, Debug, Clone)]
pub enum ParameterError {
    /// Version is not compatible with parameter
    #[error(transparent)]
    Version(#[from] InvalidVoicemeeterVersion),
    /// Strip/bus is not compatible with parameter
    #[error(transparent)]
    Type(#[from] InvalidTypeError),
    /// Parameter index is out of range
    #[error(transparent)]
    OutOfRange(#[from] OutOfRangeError),
    /// Device is invalid for parameter
    #[error(transparent)]
    Device(#[from] DeviceError),
}
