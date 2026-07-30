use insta::assert_snapshot;
use postcard_bindgen::{
    generate_bindings,
    python::{self, GenerationSettings},
    PackageInfo,
};

#[test]
fn test_build_pip_module() {
    #[derive(postcard_bindgen::PostcardBindings)]
    #[allow(unused)]
    struct Test {
        field: u8,
    }

    let tmp_dir = tempfile::tempdir().unwrap();
    let package_name = "halpi2-fw-i2c-postcard";

    let package_info = PackageInfo {
        name: package_name.into(),
        version: "0.1.0".try_into().unwrap(),
    };

    let res = python::build_package(
        tmp_dir.path(),
        package_info,
        GenerationSettings::enable_all(),
        generate_bindings!(Test),
    );

    assert!(res.is_ok());

    let project_dir = tmp_dir.path().join(package_name);
    assert!(project_dir.exists());
    assert!(project_dir.is_dir());

    let pyproject_file = project_dir.join("pyproject.toml");
    let pyproject_content = std::fs::read_to_string(pyproject_file).unwrap();
    assert_snapshot!("build_pip_module_pyproject", pyproject_content);

    let mod_dir = project_dir.join("src").join(package_name.replace("-", "_"));
    assert!(mod_dir.exists());
    assert!(mod_dir.is_dir());

    let types_dir = mod_dir.join("types");
    assert!(types_dir.exists());
    assert!(types_dir.is_dir());

    let type_file = types_dir.join("_test.py");
    assert!(type_file.exists());
    assert!(type_file.is_file());
    let type_file_content = std::fs::read_to_string(type_file).unwrap();

    assert_snapshot!("build_pip_module_types", type_file_content);
}
