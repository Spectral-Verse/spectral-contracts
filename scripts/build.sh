#!/bin/bash
set -e

echo "Building Spectra contracts..."

cargo build --target wasm32-unknown-unknown --release

echo "Build complete."
