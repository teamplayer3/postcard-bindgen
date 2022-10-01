use std::path::PathBuf;

use postcard_bindgen::{export_bindings, generate_bindings, PostcardBindings};
use serde_derive::Serialize;

extern crate alloc;

#[derive(Serialize, PostcardBindings)]
struct Test {
    name: u8,
    other: u16,
}

#[derive(Serialize, PostcardBindings)]
struct OtherTest {
    name: u8,
    #[allow(dead_code)]
    #[serde(skip)]
    other: u16,
    test: Test,
}

#[derive(Serialize, PostcardBindings)]
struct TupleStruct(u8, u16, u32);

fn export_path() -> PathBuf {
    let mut exec_path = std::env::current_exe().unwrap();
    exec_path.pop();
    exec_path.push("js_export.js");
    exec_path
}

fn main() {
    export_bindings(&export_path(), generate_bindings!(OtherTest, Test)).unwrap();
}
