use postcard_bindgen::{generate_bindings, javascript, python, PackageInfo, PostcardBindings};
use serde::Serialize;

#[derive(Serialize, PostcardBindings)]
struct Test {
    name: String,
    other: f64,
}

fn main() {
    println!(
        "{:?}",
        postcard::to_vec::<_, 20>(&Test {
            name: "test".into(),
            other: 17.2343,
        })
        .unwrap()
    );

    javascript::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "test".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        javascript::GenerationSettings::enable_all()
            .runtime_type_checks(true)
            .esm_module(false)
            .module_structure(true),
        generate_bindings!(Test),
    )
    .unwrap();

    python::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "test".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        python::GenerationSettings::enable_all()
            .runtime_type_checks(true)
            .module_structure(true),
        generate_bindings!(Test),
    )
    .unwrap();
}
