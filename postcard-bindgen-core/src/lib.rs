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

    pub fn pop_file(&mut self, content_type: impl AsRef<str>) -> Option<genco::Tokens<L>> {
        let index = self
            .files
            .iter()
            .position(|f| f.content_type.as_str() == content_type.as_ref())?;
        Some(self.files.remove(index).content)
    }
}

#[cfg(feature = "generating")]
pub struct ExportFile<L: genco::lang::Lang> {
    pub content_type: String,
    pub content: genco::Tokens<L>,
}

#[cfg(feature = "generating")]
impl<L> core::fmt::Debug for Exports<L>
where
    L: genco::lang::Lang,
    L::Item: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Exports")
            .field("files", &self.files)
            .finish()
    }
}

#[cfg(feature = "generating")]
impl<L> core::fmt::Debug for ExportFile<L>
where
    L: genco::lang::Lang,
    L::Item: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ExportFile")
            .field("content_type", &self.content_type)
            .field("content", &self.content)
            .finish()
    }
}
