use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use crate::ExportStrings;

use super::{PackageInfo, Version};

/// Builds a npm package from create language binding strings.
///
/// # Example
/// ```
/// # use postcard_bindgen::{build_package, PackageInfo, PostcardBindings, generate_bindings};
/// # use serde::Serialize;
/// #[derive(Serialize, PostcardBindings)]
/// struct Test {
///     field: u8
/// }
///
/// # fn main() {
/// let parent_dir = std::env::current_dir().unwrap();
/// let package_info = PackageInfo {
///     name: "test-package".into(),
///     version: "0.1.0".try_into().unwrap()
/// };
///
/// build_package(parent_dir.as_path(), package_info, generate_bindings!(Test));
/// # }
/// ```
pub fn build_npm_package(
    parent_dir: &Path,
    package_info: PackageInfo,
    bindings: ExportStrings,
) -> io::Result<()> {
    let mut dir = parent_dir.to_path_buf();
    dir.push(package_info.name.as_str());

    std::fs::create_dir_all(&dir)?;

    let package_json = package_file_src(package_info.name.as_str(), &package_info.version);

    let mut package_json_path = dir.to_owned();
    package_json_path.push("package.json");
    File::create(package_json_path.as_path())?.write_all(package_json.as_bytes())?;

    let mut js_export_path = dir.to_owned();
    js_export_path.push("index.js");
    File::create(js_export_path.as_path())?.write_all(bindings.bindings.as_bytes())?;

    let mut js_export_path = dir;
    js_export_path.push("index.d.ts");
    File::create(js_export_path.as_path())?.write_all(bindings.types.as_bytes())?;

    Ok(())
}

fn package_file_src(package_name: impl AsRef<str>, package_version: &Version) -> String {
    format!(
        "{{\
            \"name\": \"{}\",\
            \"description\": \"Auto generated bindings for postcard format serializing and deserializing javascript to and from bytes.\",\
            \"version\": \"{}\",\
            \"main\": \"index.js\",\
            \"types\": \"index.d.ts\"\
        }}
    ",
        package_name.as_ref(), package_version.to_string()
    )
}


