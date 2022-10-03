use std::path::PathBuf;

use postcard_bindgen::{export_bindings, generate_bindings, PostcardBindings};
use serde_derive::Serialize;

extern crate alloc;

#[derive(Serialize, PostcardBindings)]
struct A;

#[derive(Serialize, PostcardBindings)]
struct B(u8);

#[derive(Serialize, PostcardBindings)]
#[allow(dead_code)]
enum C {
    A,
    B(u8),
    C(A, B),
    D { a: u8, b: B },
}

#[derive(Serialize, PostcardBindings)]
struct D {
    a: u8,
    b: C,
    c: A,
}

fn export_path() -> PathBuf {
    let mut exec_path = std::env::current_exe().unwrap();
    exec_path.pop();
    exec_path.push("js_export.js");
    exec_path
}

fn main() {
    export_bindings(&export_path(), generate_bindings!(A, B, C, D)).unwrap();
}
