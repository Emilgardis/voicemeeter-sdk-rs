use crate::types::ZIndex;

pub(crate) static STRIP: &str = "Strip";
pub(crate) static BUS: &str = "Bus";
pub(crate) static VOICEMEETER_OPTION: &str = "Option";

/// Parameter is out of range for current program
#[derive(thiserror::Error, Debug, Clone)]
#[error("out of range: {name}({index})")]
pub struct OutOfRangeError {
    /// Name of the parameter `base` i.e "Strip" or "Bus"
    pub name: &'static str,
    /// Index that was out of range
    pub index: ZIndex,
}

/// Invalid strip/bus type for a specific parameter
#[derive(thiserror::Error, Debug, Clone)]
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
}
