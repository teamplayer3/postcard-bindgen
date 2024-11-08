use std::io::Write;

use postcard_bindgen::{generate_bindings, javascript, PackageInfo, PostcardBindings};
use serde::Serialize;

mod a {
    use super::*;

    #[derive(Serialize, PostcardBindings)]
    pub struct FooBar {
        pub a: u8,
        pub b: String,
    }
}

mod b {
    use super::*;

    #[derive(Serialize, PostcardBindings)]
    pub struct FooBar {
        pub a: u8,
        pub b: String,
    }
}

#[derive(Serialize, PostcardBindings)]
struct Message {
    a: a::FooBar,
    b: b::FooBar,
}

fn main() {
    javascript::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "js-test-bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        javascript::GenerationSettings::enable_all().runtime_type_checks(false),
        generate_bindings!(a::FooBar, b::FooBar, Message),
    )
    .unwrap();

    let m = Message {
        a: a::FooBar {
            a: 42,
            b: "Hello, World!".into(),
        },
        b: b::FooBar {
            a: 42,
            b: "Hello, World!".into(),
        },
    };

    let postcard_bytes = postcard::to_vec::<_, 100>(&m).unwrap();
    let mut file =
        std::fs::File::create(std::env::current_dir().unwrap().join("serialized.bytes")).unwrap();
    file.write_all(postcard_bytes.as_slice()).unwrap();
}
