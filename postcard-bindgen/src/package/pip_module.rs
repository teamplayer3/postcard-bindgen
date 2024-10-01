use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use crate::ExportStrings;

use super::{PackageInfo, Version};

pub fn build_pip_module(
    parent_dir: &Path,
    package_info: PackageInfo,
    bindings: ExportStrings,
) -> io::Result<()> {
    let mut dir = parent_dir.to_path_buf();
    dir.push(package_info.name.as_str());
    std::fs::create_dir_all(&dir)?;

    let package_name = package_info.name.replace("-", "_");

    let mod_toml = mod_file_src(&package_name, &package_info.version);

    let mut mod_toml_path = dir.to_owned();
    mod_toml_path.push("pyproject.toml");
    File::create(mod_toml_path.as_path())?.write_all(mod_toml.as_bytes())?;

    dir.push("src");
    dir.push(&package_name);

    std::fs::create_dir_all(&dir)?;

    let mut bindings_export_path = dir.to_owned();
    bindings_export_path.push("__init__.py");
    File::create(bindings_export_path.as_path())?.write_all(bindings.bindings.as_bytes())?;

    Ok(())
}

fn mod_file_src(package_name: impl AsRef<str>, package_version: &Version) -> String {
    
    let package_name = package_name.as_ref();
    let package_version = package_version.to_string();

    format!(
        "
[project]
name = \"{package_name}\"
version = \"{package_version}\"
authors = [
  {{ name=\"postcard-bindgen\" }},
]
description = \"Auto generated bindings for postcard format serializing and deserializing python to and from bytes.\"
requires-python = \">=3.8\"
",
    )
}