# Postcard Bindgen

[![Build status](https://github.com/teamplayer3/postcard-bindgen/workflows/Rust/badge.svg)](https://github.com/teamplayer3/postcard-bindgen/actions)

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
    export_bindings(
        Path::new("./js_export.js"),
        generate_bindings!(Test), // register container for generating bindings
    )
    .unwrap();
}
```

## JavaScript Type mapping

### Unit Type
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
    key: "A",
},
{
    key: "B",
    value: [123]
},
{
    key: "C",
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
</table>


