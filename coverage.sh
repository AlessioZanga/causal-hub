#!/bin/bash
set -e
set -o pipefail

# Enable conda environment if not already active.
source ~/miniconda3/etc/profile.d/conda.sh
conda activate causal-hub
# Clean up previous coverage data files.
rm -rf coverage_html *.profraw *.profdata ../*.profraw ../*.profdata
# Enable coverage instrumentation for Rust code when building the Python extension.
RUSTFLAGS="-Cinstrument-coverage" \
LLVM_PROFILE_FILE="coverage-%p-%m.profraw" \
maturin develop
# Run test with coverage collection enabled.
pytest
# Merge the raw coverage data files into a single file.
llvm-profdata merge -sparse *.profraw -o coverage.profdata
# Generate a human-readable coverage report.
llvm-cov show target/debug/libcausal_hub$( \
        [ "$(uname)" = "Darwin" ] && echo ".dylib" || echo ".so" \
    ) \
    --ignore-filename-regex='/.cargo/|.rustup/' \
    -instr-profile=coverage.profdata \
    -show-line-counts-or-regions \
    -output-dir=coverage_html \
    -format=html
# Open the coverage report in the default web browser.
echo "Opening coverage report in the default web browser ..."
nohup xdg-open coverage_html/index.html >/dev/null 2>&1 &
