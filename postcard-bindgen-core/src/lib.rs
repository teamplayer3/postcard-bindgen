extern crate alloc;

mod code_gen;
mod npm_packet;
mod utils;

pub mod registry;
pub mod type_info;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

pub use code_gen::{generate_js, type_checking::ts::gen_ts_typings};
pub use npm_packet::{build_npm_package, PacketInfo, Version, VersionFromStrError};

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

pub fn export_bindings(path: &Path, bindings: impl AsRef<str>) -> io::Result<()> {
    let mut file = File::create(path.join("js_export.js"))?;
    file.write_all(bindings.as_ref().as_bytes())?;
    Ok(())
}
