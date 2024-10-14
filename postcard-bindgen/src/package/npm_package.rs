use core::borrow::Borrow;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use postcard_bindgen_core::{
    code_gen::js::{generate, GenerationSettings},
    registry::BindingType,
};

use super::{PackageInfo, Version};

/// Builds a npm package from created language binding strings.
///
/// # Example
/// ```
/// # use postcard_bindgen::{javascript::{build_package, GenerationSettings}, PackageInfo, PostcardBindings, generate_bindings};
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
/// build_package(parent_dir.as_path(), package_info, GenerationSettings::enable_all(), generate_bindings!(Test));
/// # }
/// ```
pub fn build_npm_package(
    parent_dir: &Path,
    package_info: PackageInfo,
    gen_settings: impl Borrow<GenerationSettings>,
    bindings: impl AsRef<[BindingType]>,
) -> io::Result<()> {
    let mut dir = parent_dir.to_path_buf();
    dir.push(package_info.name.as_str());

    std::fs::create_dir_all(&dir)?;

    let exports = generate(bindings, gen_settings);

    let package_json = package_file_src(
        package_info.name.as_str(),
        &package_info.version,
        exports.file("ts").is_some(),
    );

    let mut package_json_path = dir.to_owned();
    package_json_path.push("package.json");
    File::create(package_json_path.as_path())?.write_all(package_json.as_bytes())?;

    let js_export_path = dir.join("index.js");
    File::create(js_export_path.as_path())?.write_all(
        exports
            .file("js")
            .unwrap()
            .to_file_string()
            .unwrap()
            .as_bytes(),
    )?;

    if let Some(file) = exports.file("ts") {
        let ts_export_path = dir.join("index.d.ts");
        File::create(ts_export_path.as_path())?
            .write_all(file.to_file_string().unwrap().as_bytes())?;
    }

    Ok(())
}

fn package_file_src(
    package_name: impl AsRef<str>,
    package_version: &Version,
    ts_types_enabled: bool,
) -> String {
    format!("\
{{
    \"name\": \"{}\",
    \"description\": \"Auto generated bindings for postcard format serializing and deserializing javascript to and from bytes.\",
    \"version\": \"{}\",
    \"main\": \"index.js\"{}
}}",
        package_name.as_ref(), package_version.to_string(), if ts_types_enabled { ",\n\t\"types\": \"index.d.ts\"" } else { "" }
    )
}
