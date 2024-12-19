mod registry;

use postcard_bindgen_core::code_gen::python::{generate, GenerationSettings};

use registry::init_registry;

#[test]
fn test_runtime_checks() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all();
    let exports = generate(containers, gen_settings, "test".to_owned());

    let runtime_checks_file = exports
        .file("runtime_checks")
        .unwrap()
        .to_file_string()
        .unwrap();
    insta::assert_snapshot!(runtime_checks_file);
}

#[test]
fn test_ser_with_runtime_checks() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all();
    let exports = generate(containers, gen_settings, "test".to_owned());

    let ser_file = exports.file("ser").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(ser_file);
}

#[test]
fn test_ser_without_runtime_checks() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all().runtime_type_checks(false);
    let exports = generate(containers, gen_settings, "test".to_owned());

    let ser_file = exports.file("ser").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(ser_file);
}

#[test]
fn test_des() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all();
    let exports = generate(containers, gen_settings, "test".to_owned());

    let des_file = exports.file("des").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(des_file);
}
