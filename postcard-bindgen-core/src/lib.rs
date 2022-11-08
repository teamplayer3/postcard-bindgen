extern crate alloc;

mod code_gen;
mod utils;

pub mod registry;
pub mod type_info;

pub use code_gen::{generate_js, type_checking::ts::gen_ts_typings};

pub enum ArchPointerLen {
    U32,
    U64,
}

impl ArchPointerLen {
    #[allow(unused)]
    pub(crate) fn into_bytes(self) -> usize {
        match self {
            ArchPointerLen::U32 => 4,
            ArchPointerLen::U64 => 8,
        }
    }
}

/// Helper struct to pass the generated language strings to an export function.
pub struct ExportStrings {
    pub js_file: String,
    pub ts_file: String,
}
