#!/bin/bash

# This script provides a reliable way to build and run the Naldom compiler,
# avoiding race conditions present with `cargo run`.

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Building the project..."
# Build all workspace members first to ensure all artifacts are available.
cargo build

echo "Running the compiler..."
# Execute the compiled CLI binary, passing all script arguments to it.
./target/debug/naldom-cli "$@"