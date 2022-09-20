use std::path::Path;

use postcard_bindings::{export_js_bindings, ArchPointerLen, JsExportable, TypescriptBindings};
use serde_derive::Serialize;

extern crate alloc;

#[derive(Serialize, TypescriptBindings)]
struct Test {
    name: u8,
    other: u16,
}

#[derive(Serialize, TypescriptBindings)]
struct OtherTest {
    name: u8,
    other: u16,
    // string: std::string::String,
    // alloc_string: alloc::string::String,
    array: std::vec::Vec<u8>,
    alloc_array: alloc::vec::Vec<u32>,
}

fn main() {
    export_js_bindings(
        Path::new("./js_export.js"),
        vec![Test::js_bindings(), OtherTest::js_bindings()],
        ArchPointerLen::U32,
    )
    .unwrap();
}
