# Postcard Bindgen Test Bindings Project

Generate the bindings from the `postcard-bindgen` crate directory (current directory):

```bash
cargo +1.88 run --example generate_bindings --features="std generating heapless"
```

The `generate_bindings` example generates the `js-test-bindings` Node package
and serializes the Rust structures to `serialized.bytes`. The
TypeScript project uses `js-test-bindings` as a local dependency, loads
`serialized.bytes`, and deserializes it with the generated bindings. It compares
that result with the value produced by serializing and deserializing the same
structure in JavaScript.

Then run the npm project:

```bash
cd test-bindings-proj
npm install
npm run run
```
