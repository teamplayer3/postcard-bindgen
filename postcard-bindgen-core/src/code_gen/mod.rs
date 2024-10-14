mod available_check;
mod field_accessor;
mod import_registry;
mod utils;
mod variable_path;

pub mod js;
pub mod python;

use crate::type_info::NumberMeta;

const U8_BYTES_CONST: &str = "U8_BYTES";
const U16_BYTES_CONST: &str = "U16_BYTES";
const U32_BYTES_CONST: &str = "U32_BYTES";
const U64_BYTES_CONST: &str = "U64_BYTES";
const U128_BYTES_CONST: &str = "U128_BYTES";

impl NumberMeta {
    pub(crate) fn as_byte_string(&self) -> &'static str {
        let bytes = match self {
            NumberMeta::Integer { bytes, .. } => bytes,
            NumberMeta::FloatingPoint { bytes } => bytes,
        };
        match bytes {
            1 => U8_BYTES_CONST,
            2 => U16_BYTES_CONST,
            4 => U32_BYTES_CONST,
            8 => U64_BYTES_CONST,
            16 => U128_BYTES_CONST,
            _ => unreachable!(),
        }
    }
}
