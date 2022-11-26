# Example no_std

Shows how to use this crate in a project to generate bindings.

## Setup

The main project must be build up as a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). The executable to generate the bindings is a member of it.

```toml
[workspace]
members = [
    "gen-bindings"
]
```

Important is to enable the `generating` feature in the `gen-bindings` crate. In the main library crate the feature is not enabled. This allows using this crate in a `no_std` and different target than this computer context.

Main `Cargo.toml`:
```toml
[dependencies]
postcard-bindgen = "0.1"

serde = { version = "1", features = ["derive"]}
```

Package `gen-bindings` `Cargo.toml`:
```toml
[dependencies]
postcard-bindgen = { version = "0.1", features = ["generating"] }

no-std = { path = "../" }
```

## Usage

To build the javascript bindings run one of the following scripts in the current directory:

- Windows
  ```bash
  gen-bindings.ps1
  ```

- Linux
  ```bash
  gen-bindings.sh
  ```

## Good to know

The target of the main binary (in this case no real usage for the `no_std` library) is another as for example linux or windows. To run the `gen-bindings` member of the main workspace on the current system, the `target` must be explicitly defined. This is done in the scripts `gen-bindings.*`.