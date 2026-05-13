#!/bin/bash
set -e

echo "Running Spectra contract tests..."

cargo test

echo "Tests complete."
