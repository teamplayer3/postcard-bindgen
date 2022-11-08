use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use postcard_bindgen_core::ExportStrings;

/// Export the generated binding string to a file with passed file name at passed path.
///
/// Only javascript code will be exported. To export the generated typescript typings
/// as well use [`crate::build_npm_package`].
///
/// If the file already exists if will be overwritten.
pub fn export_bindings(
    path: &Path,
    file_name: impl AsRef<str>,
    bindings: ExportStrings,
) -> io::Result<()> {
    let mut file = File::create(path.join(file_name.as_ref()))?;
    file.write_all(bindings.js_file.as_bytes())?;
    Ok(())
}
