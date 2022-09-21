# Postcard Bindings

[![Build status](https://github.com/teamplayer3/postcard-bindings/workflows/Rust/badge.svg)](https://github.com/teamplayer3/postcard-bindings/actions)

The [postcard crate](https://github.com/jamesmunns/postcard) serializes and deserializes rust structs by using the [serde crate](https://github.com/serde-rs/serde) to a byte format. The resulting byte size is minimal. This is very useful if serialization and deserialization is done in rust and share the same structures.

This `crate` can generate bindings from the rust structures for other languages than rust. This allows to use the `postcard crate` from other languages.

> `Crate` is work in progress. By now it can't be used for productions.

## Supported languages

- [ ] JavaScript (WIP)
- [ ] Python

## Usage

The structs for which bindings should be generated must be annotated with the `PostcardBindings` macro. This macro understands `serde` annotation. This means renaming fields and other functionality by `serde` is supported.

## Example

```rust
#[derive(Serialize, PostcardBindings)]
struct Test {
    name: u8,
    other: u16,
}

fn main() {
    export_js_bindings(
        Path::new("./js_export.js"),
        vec![Test::js_bindings()],
        ArchPointerLen::U32, // used for byte amount of `usize` and `isize`
    )
    .unwrap();
}
```
