# Postcard Bindgen

[![Build status](https://github.com/teamplayer3/postcard-bindgen/workflows/Rust/badge.svg)](https://github.com/teamplayer3/postcard-bindgen/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/teamplayer3/postcard-bindgen)
[![Crates.io](https://img.shields.io/crates/v/postcard-bindgen.svg)](https://crates.io/crates/postcard-bindgen)
[![Documentation](https://docs.rs/postcard-bindgen/badge.svg)](https://docs.rs/postcard-bindgen)

`Postcard Bindgen` allows generating code for other languages to serialize to and deserialize from [postcard](https://github.com/jamesmunns/postcard) byte format. This helps to setup a communication between for example a microcontroller and a App using the `postcard crate` and its lightweight memory format.

As main types structs and enums can be annotated with `PostcardBindings` to generate code for them. The generated code can be exported to a npm package to import it into a javascript project.

## Usage

Structs and enums for which bindings should be generated must be annotated with `Serialize`/`Deserialize` from the [serde crate](https://github.com/serde-rs/serde) and the `PostcardBindings` macro from this crate.

The process is divided into two steps. Firstly the annotation step. This is done mostly in a library crate. Secondly in a extra binary crate the annotated structs and enums must be imported (this means the library crate must be defined as a dependency) and as a main function the generation logic added. To generate the npm package this extra binary crate must be run.

> If the `postcard-bindgen` crate is added as a dependency in the generation binary crate the future `generating` must be enabled.

## Example

This example shows how to easily generate a npm package. For this the struct `Test` and the generation logic is in the same rust file.

```rust
#[derive(Serialize, PostcardBindings)]
struct Test {
    name: u8,
    other: u16,
}

fn main() {
    build_npm_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "test".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        generate_bindings!(Test),
    )
    .unwrap();
}
```

To now serialize a struct in javascript the following code can be used.

```js
const test = {
    name: "test",
    other: 23
}

const bytes = serialize("Test", test)
```

## JavaScript Type mapping

<table>
<tr><td> Type Name </td> <td> Rust </td> <td> Js </td></tr>
<tr><td>Unit Type</td><td>

```rust
struct UnitStruct;
```
</td><td>

```javascript
{}
```
</td><tr>
<tr><td>New Type</td><td>

```rust
struct NewType(u8);
```
</td><td>

```javascript
[123]
```
</td><tr>
<tr><td>Tuple Struct</td><td>

```rust
struct TupleStruct(u8, u16, u32);
```
</td><td>

```javascript
[123, 1234, 12345]
```
</td><tr>
<tr><td>Struct</td><td>

```rust
struct Struct {
    a: u8,
    b: u16
};
```
</td><td>

```javascript
{
    a: 123,
    b: 1234
}
```
</td><tr>
<tr><td>Enum</td><td>

```rust
enum Enum {
    A,
    B(u8),
    C {
        a: u8
    }
};
```
</td><td>

```javascript
{
    tag: "A",
},
{
    tag: "B",
    value: 123
},
{
    tag: "C",
    value: {
        a: 123
    }
}
```
</td><tr>
<tr><td>Option</td><td>

```rust
struct OptionTuple(Option<u8>);

struct OptionStruct {
    a: Option<u8>
}
```
</td><td>

```javascript
// OptionTuple(Some(123))
[123]
// OptionTuple(None)
[undefined]

// OptionStruct { a: Some(123) }
{
    a: 123
}
// OptionStruct { a: None }
{}
// or
{
    a: undefined
}
```
</td><tr>
<tr><td>Map</td><td>

```rust
let map_string_key = HashMap::<String, u8>::new();

let map_any_key = HashMap::<u16, u8>::new();
```
</td><td>

```javascript
// map_string_key
{
    key: value
}

// map_any_key
new Map()
```
</td><tr>
</table>

### License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Postcard Bindgen by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions
