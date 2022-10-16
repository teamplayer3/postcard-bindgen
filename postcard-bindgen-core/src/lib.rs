extern crate alloc;

mod code_gen;
pub mod registry;
pub mod type_info;
mod utils;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use genco::{prelude::JavaScript, quote, Tokens};
use handlebars::Handlebars;

use code_gen::{
    ser_des::{
        gen_deserialize_func, gen_ser_des_classes, gen_ser_des_functions, gen_serialize_func,
    },
    type_checking::gen_type_checkings,
};
use registry::BindingType;
use serde::Serialize;

pub use code_gen::type_checking::ts::gen_ts_typings;

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

#[macro_export]
macro_rules! generate_bindings {
    ($( $x:ty ),*) => {
        {
            let mut reg = postcard_bindgen::BindingsRegistry::default();
            $(
                <$x as postcard_bindgen::JsBindings>::create_bindings(&mut reg);
            )*
            let bindings = reg.into_entries();
            (postcard_bindgen::generate_js(&bindings).to_file_string().unwrap(), postcard_bindgen::gen_ts_typings(bindings).to_file_string().unwrap())
        }
    };
}

pub fn export_bindings(path: &Path, bindings: impl AsRef<str>) -> io::Result<()> {
    let mut file = File::create(path.join("js_export.js"))?;
    file.write_all(bindings.as_ref().as_bytes())?;
    Ok(())
}

pub fn generate_js(tys: impl AsRef<[BindingType]>) -> Tokens<JavaScript> {
    let ser_des_body = gen_ser_des_functions(&tys);
    let type_checks = gen_type_checkings(&tys);
    quote!(
        $(gen_ser_des_classes())
        $ser_des_body
        $type_checks
        $(gen_serialize_func(&tys))
        $(gen_deserialize_func(tys))
    )
}

pub fn build_npm_package(
    path: &Path,
    bindings: (impl AsRef<str>, impl AsRef<str>),
) -> io::Result<()> {
    let mut dir = path.to_path_buf();
    dir.push("test-bindings");

    std::fs::create_dir_all(&dir)?;

    let package_json = package_file_src("test".into(), "0.1.0".into()).unwrap();

    let mut package_json_path = dir.to_owned();
    package_json_path.push("package.json");
    File::create(package_json_path.as_path())?.write_all(package_json.as_bytes())?;

    let mut js_export_path = dir.to_owned();
    js_export_path.push("index.js");
    File::create(js_export_path.as_path())?.write_all(bindings.0.as_ref().as_bytes())?;

    let mut js_export_path = dir;
    js_export_path.push("index.d.ts");
    File::create(js_export_path.as_path())?.write_all(bindings.1.as_ref().as_bytes())?;

    Ok(())
}

static PACKAGE_FILE_TEMPLATE: &[u8] = include_bytes!("gen_src/package-template.json");

fn package_file_src(
    package_name: String,
    package_version: String,
) -> Result<String, handlebars::RenderError> {
    #[derive(Serialize)]
    struct TemplateData {
        package_name: String,
        package_version: String,
    }

    let template_data = TemplateData {
        package_name,
        package_version,
    };
    Handlebars::new().render_template(
        String::from_utf8(PACKAGE_FILE_TEMPLATE.into())
            .unwrap()
            .as_str(),
        &template_data,
    )
}
