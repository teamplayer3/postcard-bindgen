use std::path::PathBuf;

use postcard_bindgen::{export_js_bindings, ArchPointerLen, JsExportable, PostcardBindings};
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
    // string: std::string::String,
    // alloc_string: alloc::string::String,
    array: std::vec::Vec<u8>,
    #[serde(rename = "allocArray")]
    alloc_array: alloc::vec::Vec<u32>,
}

fn export_path() -> PathBuf {
    let mut exec_path = std::env::current_exe().unwrap();
    exec_path.pop();
    exec_path.push("js_export.js");
    exec_path
}

fn main() {
    export_js_bindings(
        export_path().as_path(),
        vec![Test::js_bindings(), OtherTest::js_bindings()],
        ArchPointerLen::U32,
    )
    .unwrap();
}
