# Example no_std

Shows how this crate can be used in a project to generate bindings.

## Setup

The main project must be build up as a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). The executable to generate the bindings is a member of it.

```toml
[workspace]
members = [
    "gen-bindings"
]
```

## Usage

To build the bindings run the following in the workspace root dir:

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