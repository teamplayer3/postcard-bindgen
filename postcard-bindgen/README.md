# Postcard Bindgen Test Bindings Project

Generate the bindings from the `postcard-bindgen` crate directory (current directory):

```bash
cargo +1.88 run --example generate_bindings --features="std generating heapless"
```

Then run the npm project:

```bash
cd test-bindings-proj
npm install
npm run run
```
