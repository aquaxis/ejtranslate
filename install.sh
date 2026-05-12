#!/bin/sh
# ejtranslate installer (one-liner-callable).
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/aquaxis/ejtranslate/main/install.sh | sh
#
# Builds and installs the `ejtranslate` binary from source using `cargo install`.
# Requires a Rust toolchain.

set -eu

REPO="https://github.com/aquaxis/ejtranslate"

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo is required. Install Rust from https://rustup.rs and re-run." >&2
    exit 1
fi

echo "Installing ejtranslate from ${REPO} ..."
cargo install --git "${REPO}" --locked ejtranslate

cat <<'EOF'

Done.

Make sure ~/.cargo/bin is on your PATH, then run:
  ejtranslate --help

Requires a local Ollama server. Pull the default model with:
  ollama pull translategemma:12b
EOF
