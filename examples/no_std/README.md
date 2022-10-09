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

```bash
cargo run --package gen-bindings
```
