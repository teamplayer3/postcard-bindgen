#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "generating")]
mod code_gen;

#[cfg(feature = "generating")]
mod utils;

pub mod registry;
pub mod type_info;

#[cfg(feature = "generating")]
pub use code_gen::{generate, generate_typings};

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
#[cfg(feature = "generating")]
pub struct ExportStrings {
    pub bindings: String,
    pub types: String,
}
