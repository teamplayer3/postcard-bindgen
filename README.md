# Postcard Bindgen

[![Build status](https://github.com/teamplayer3/postcard-bindgen/workflows/Rust/badge.svg)](https://github.com/teamplayer3/postcard-bindgen/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/teamplayer3/postcard-bindgen)
[![Crates.io](https://img.shields.io/crates/v/postcard-bindgen.svg)](https://crates.io/crates/postcard-bindgen)
[![Documentation](https://docs.rs/postcard-bindgen/badge.svg)](https://docs.rs/postcard-bindgen)

`Postcard Bindgen` allows generating code for other languages to serialize to and deserialize from [postcard](https://github.com/jamesmunns/postcard) byte format. This helps to setup a communication between for example a microcontroller and a App using the `postcard crate` and its lightweight memory format.

As main types structs and enums can be annotated with `PostcardBindings` to generate code for them. The generated code can be exported as a npm package to import it into a JavaScript project or as a pip package for python.

## Supported Languages

* 🌐 <b>JavaScript</b>
* 🐍 <b>Python</b>

## Usage

> :warning: Run the crate that generates the bindings with rust nightly. This is necessary because this crate depends on [genco](https://github.com/udoprog/genco) and this crate uses a nightly feature to detect column changes.

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
    javascript::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "generation-test".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        javascript::GenerationSettings::enable_all(),
        generate_bindings!(Test),
    )
    .unwrap();
}
```

The following code can now be used to serialize an object in JavaScript.

```js
import { serialize } from "generation-test";

const test = {
    name: "test",
    other: 23
}

const bytes = serialize("Test", test)
```

## Type mappings

<table>
<tr><td> Type Name </td> <td> Rust </td> <td> Js </td><td> Python </td></tr>
<tr><td>Unit Type</td><td>

```rust
struct UnitStruct;
```
</td><td>

```javascript
{}
```
</td><td>

```python
class UnitStruct:
    pass

t = UnitStruct()
```
</td></tr>
<tr><td>Tuple Struct</td><td>

```rust
struct TupleStruct(u8, u16, u32);
```
</td><td>

```javascript
[123, 1234, 12345]
```
</td><td>

```python
class TupleStruct(tuple[u8]):
    ...

t = TupleStruct(123, 1234, 12345)
```
</td></tr>
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
</td><td>

```python
@dataclass
class Struct
    a: u8
    b: u16

t = Struct(a = 123, b = 1234)
```
</td></tr>
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
</td><td>

```python
class Enum:
    pass

class Enum_A(Enum):
    pass

class Enum_B(Enum, tuple[u8]):
    ...

@dataclass
class Enum_C(Enum)
    a: u8

a = Enum_A()
b = Enum_B(23)
c = Enum_C(a = 23)
```
</td></tr>
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
</td><td>

```python
# OptionTuple(Some(123))
OptionTuple(123)
# OptionTuple(None)
OptionTuple(None)

# OptionStruct { a: Some(123) }
OptionStruct(a = 123)
# OptionStruct { a: None }
OptionStruct(a = None)
```
</td></tr>
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
</td><td>

```python
# map_string_key
: Dict[str, u8] = {
    key: value
}

# map_any_key
: Dict[u16, u8] = {
    key: value
}
```
</td></tr>
</table>

### License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Postcard Bindgen by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions
