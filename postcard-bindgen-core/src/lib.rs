#![feature(str_as_str)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "generating")]
pub mod code_gen;
#[cfg(feature = "generating")]
pub mod path;
#[cfg(feature = "generating")]
pub mod registry;
#[cfg(feature = "generating")]
pub mod type_info;

#[cfg(feature = "generating")]
pub use genco::lang;

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
pub struct Exports<L: genco::lang::Lang> {
    pub files: Vec<ExportFile<L>>,
}

#[cfg(feature = "generating")]
impl<L: genco::lang::Lang> Exports<L> {
    pub fn file(&self, content_type: impl AsRef<str>) -> Option<&genco::Tokens<L>> {
        self.files
            .iter()
            .find(|f| f.content_type.as_str() == content_type.as_ref())
            .map(|f| &f.content)
    }
}

#[cfg(feature = "generating")]
#[derive(Debug)]
pub struct ExportFile<L: genco::lang::Lang> {
    pub content_type: String,
    pub content: genco::Tokens<L>,
}
