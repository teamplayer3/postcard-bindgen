#!/bin/bash

# nightly is required for Python bindings
cargo +nightly run -p gen-bindings --target $(rustc -vV | sed -n 's|host: ||p')
