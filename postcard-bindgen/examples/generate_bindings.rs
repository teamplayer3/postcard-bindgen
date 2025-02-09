use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, Write},
    num::NonZero,
    ops::{Deref, DerefMut, Range},
    sync::Mutex,
};

use postcard_bindgen::{generate_bindings, javascript, python, PackageInfo, PostcardBindings};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PostcardBindings)]
struct A;

#[derive(Debug, Serialize, Deserialize, PostcardBindings)]
struct B(u8, Vec<u16>, String, HashMap<u16, u8>);

#[derive(Debug, Serialize, Deserialize, PostcardBindings)]
#[allow(dead_code)]
enum C {
    A,
    B(u8),
    C(A, B),
    D { a: Vec<u8>, b: B, c: bool },
}

#[derive(Debug, Serialize, Deserialize, PostcardBindings)]
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
    l: e::E,
    m: (u8, String, Vec<u8>),
    n: bool,
    o: NonZero<u32>,
    p: i8,
    q: i16,
}

mod e {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PostcardBindings)]
    pub struct E(pub u8, pub f::F);

    pub mod f {
        use super::*;

        #[derive(Debug, Serialize, Deserialize, PostcardBindings)]
        pub struct F(pub u8);
    }
}

fn main() {
    javascript::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "js-test-bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        javascript::GenerationSettings::enable_all()
            .runtime_type_checks(true)
            .esm_module(false)
            .module_structure(true),
        generate_bindings!(A, B, e::E, C, D, e::f::F),
    )
    .unwrap();

    python::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "py-test-bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        python::GenerationSettings::enable_all()
            .runtime_type_checks(true)
            .module_structure(true),
        generate_bindings!(A, B, e::E, C, D, e::f::F),
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
        l: e::E(123, e::f::F(234)),
        m: (123, "hello".into(), vec![1, 2, 3]),
        n: true,
        o: NonZero::new(123).unwrap(),
        p: -123,
        q: -1234,
    };
    let postcard_bytes = postcard::to_vec::<_, 100>(&d).unwrap();
    let mut file =
        std::fs::File::create(std::env::current_dir().unwrap().join("serialized.bytes")).unwrap();
    file.write_all(postcard_bytes.as_slice()).unwrap();

    let mut file_read = std::fs::File::open("serialized_own.bytes").unwrap();
    static mut BUFFER: Vec<u8> = Vec::new();

    unsafe {
        file_read.read_to_end(&mut BUFFER).unwrap();
    }

    let deserialized: D = unsafe { postcard::from_bytes(BUFFER.as_slice()).unwrap() };

    println!("{:?}", deserialized);
}
