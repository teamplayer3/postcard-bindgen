use std::io::Write;

use postcard_bindgen::{export_bindings, generate_bindings, PostcardBindings};
use serde_derive::Serialize;

#[derive(Serialize, PostcardBindings)]
struct A;

#[derive(Serialize, PostcardBindings)]
struct B(u8, Vec<u16>, String);

#[derive(Serialize, PostcardBindings)]
#[allow(dead_code)]
enum C {
    A,
    B(u8),
    C(A, B),
    D { a: Vec<u8>, b: B },
}

#[derive(Serialize, PostcardBindings)]
struct D {
    a: u8,
    b: C,
    c: A,
    d: Vec<u8>,
    e: Option<u8>,
    f: &'static [u8],
    g: &'static str,
}

fn main() {
    export_bindings(
        std::env::current_dir()
            .unwrap()
            .join("js_export.js")
            .as_path(),
        generate_bindings!(A, B, C, D),
    )
    .unwrap();

    let d = D {
        a: 123,
        b: C::D {
            a: vec![6, 123],
            b: B(231, vec![182, 1234], "hello from rust".into()),
        },
        c: A,
        d: vec![234, 21],
        e: None,
        f: &[123, 23],
        g: "Hello",
    };
    let postcard_bytes = postcard::to_vec::<_, 100>(&d).unwrap();
    let mut file =
        std::fs::File::create(std::env::current_dir().unwrap().join("serialized.bytes")).unwrap();
    file.write_all(postcard_bytes.as_slice()).unwrap();
}
