use std::{collections::HashMap, io::Write, ops::Range};

use postcard_bindgen::{build_npm_package, generate_bindings, PackageInfo, PostcardBindings};
use serde::Serialize;

#[derive(Serialize, PostcardBindings)]
struct A;

#[derive(Serialize, PostcardBindings)]
struct B(u8, Vec<u16>, String, HashMap<u16, u8>);

#[derive(Serialize, PostcardBindings)]
#[allow(dead_code)]
enum C {
    A,
    B(u8),
    C(A, B),
    D { a: Vec<u8>, b: B, c: bool },
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
    h: Range<u16>,
    i: HashMap<String, u16>,
    j: HashMap<u16, u8>,
    k: [u8; 10],
    m: (u8, String, Vec<u8>),
    n: bool,
}

fn main() {
    build_npm_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "test-bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        generate_bindings!(A, B, C, D),
    )
    .unwrap();

    let d = D {
        a: 123,
        b: C::D {
            a: vec![6, 123],
            b: B(
                231,
                vec![182, 1234],
                "hello from rust".into(),
                HashMap::new(),
            ),
            c: false,
        },
        c: A,
        d: vec![234, 21],
        e: None,
        f: &[123, 23],
        g: "Hello",
        h: (10..30),
        i: {
            let mut map = HashMap::new();
            map.insert("a".into(), 23);
            map
        },
        j: {
            let mut map = HashMap::new();
            map.insert(234, 23);
            map.insert(23, 99);
            map
        },
        k: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        m: (123, "hello".into(), vec![1, 2, 3]),
        n: true,
    };
    let postcard_bytes = postcard::to_vec::<_, 100>(&d).unwrap();
    let mut file =
        std::fs::File::create(std::env::current_dir().unwrap().join("serialized.bytes")).unwrap();
    file.write_all(postcard_bytes.as_slice()).unwrap();
}
