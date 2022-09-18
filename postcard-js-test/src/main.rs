use std::path::Path;

use postcard_js_lib::{export_js_bindings, ArchPointerLen, JsExportable};
use postcard_js_proc_macro::TypescriptDefinition;
use serde_derive::Serialize;

#[derive(Serialize, TypescriptDefinition)]
struct Test {
    name: u8,
    other: u16,
}

#[derive(Serialize, TypescriptDefinition)]
struct OtherTest {
    name: u8,
    other: u16,
}

fn main() {
    export_js_bindings(
        Path::new("./js_export.js"),
        vec![Test::js_bindings(), OtherTest::js_bindings()],
        ArchPointerLen::U32,
    )
    .unwrap();
}
