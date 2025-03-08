# Example no_std

Demonstrates how to use this crate in a project to generate bindings.

## Setup

The main project must be structured as a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). The executable responsible for generating the bindings must be a member of this workspace.

```toml
[workspace]
members = [
    "gen-bindings"
]
```

It is important to enable the `generating` feature in the `gen-bindings` crate. However, this feature should not be enabled in the main library crate. This setup allows the crate to be used in a `no_std` environment and on a different target than the development machine.

Main `Cargo.toml`:
```toml
[dependencies]
postcard-bindgen = "0.6"

serde = { version = "1", features = ["derive"] }
```

Package `gen-bindings` `Cargo.toml`:
```toml
[dependencies]
postcard-bindgen = { version = "0.6", features = ["generating"] }

no-std = { path = "../" }
```

## Usage

To build the JavaScript bindings, run one of the following scripts in the current directory:

- Windows:
  ```bash
  gen-bindings.ps1
  ```

- Linux:
  ```bash
  gen-bindings.sh
  ```

## Good to Know

Generating the bindings requires features from the standard library. Because of this, the `gen-bindings` crate must be compiled for a different target. The required `target` must be explicitly defined, which is handled in the `gen-bindings.*` scripts.