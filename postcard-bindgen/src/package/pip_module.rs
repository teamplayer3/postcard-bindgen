use core::borrow::Borrow;
use std::{
    fs::{create_dir_all, File},
    io::{self, Write},
    path::Path,
};

use postcard_bindgen_core::{
    code_gen::python::{generate, GenerationSettings},
    registry::ContainerCollection,
};

use super::{PackageInfo, Version};

pub fn build_pip_module(
    parent_dir: &Path,
    package_info: PackageInfo,
    gen_settings: impl Borrow<GenerationSettings>,
    containers: ContainerCollection,
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

    let exports = generate(&containers, gen_settings, package_info.name);

    let bindings_export_path = dir.to_owned();

    for file in exports.files {
        let path = bindings_export_path.join(format!("{}.py", file.content_type));
        let dir_path = {
            let mut p = path.clone();
            p.pop();
            p
        };
        create_dir_all(dir_path)?;
        File::create(path.as_path())?
            .write_all(file.content.to_file_string().unwrap().as_bytes())?;
    }

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
