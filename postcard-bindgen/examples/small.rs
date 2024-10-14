use postcard_bindgen::{build_package, generate_bindings, PackageInfo, PostcardBindings};
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

    build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "test".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        generate_bindings!(Test),
    )
    .unwrap();
}
