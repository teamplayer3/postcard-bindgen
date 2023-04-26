use postcard_bindgen::{build_npm_package, generate_bindings, PackageInfo, PostcardBindings};
use serde::Serialize;

#[derive(Serialize, PostcardBindings)]
struct Test {
    name: u8,
    other: u16,
}

fn main() {
    build_npm_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "test".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        generate_bindings!(Test),
    )
    .unwrap();
}
