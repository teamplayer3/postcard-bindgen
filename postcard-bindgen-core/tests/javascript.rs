mod registry;

use postcard_bindgen_core::code_gen::js::{generate, GenerationSettings};

use registry::init_registry;

#[test]
fn test_runtime_checks() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all();
    let (exports, _meta) = generate(containers, gen_settings);

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
    let (exports, _meta) = generate(containers, gen_settings);

    let ser_file = exports.file("ser").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(ser_file);
}

#[test]
fn test_ser_without_runtime_checks() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all().runtime_type_checks(false);
    let (exports, _meta) = generate(containers, gen_settings);

    let ser_file = exports.file("ser").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(ser_file);
}

#[test]
fn test_des() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all();
    let (exports, _meta) = generate(containers, gen_settings);

    let des_file = exports.file("des").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(des_file);
}

#[test]
fn test_ts_types() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all().runtime_type_checks(false);
    let (exports, _meta) = generate(containers, gen_settings);

    let ts_file = exports.file("ts").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(ts_file);
}

#[test]
fn test_ser_cjf_module() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all().esm_module(false);
    let (exports, _meta) = generate(containers, gen_settings);

    let ser_file = exports.file("ser").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(ser_file);
}

#[test]
fn test_des_cjf_module() {
    let containers = init_registry().into_entries();

    let gen_settings = GenerationSettings::enable_all().esm_module(false);
    let (exports, _meta) = generate(containers, gen_settings);

    let des_file = exports.file("des").unwrap().to_file_string().unwrap();
    insta::assert_snapshot!(des_file);
}
