#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=tests/helpers.sh
source "$SCRIPT_DIR/helpers.sh"

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v cargo >/dev/null 2>&1; then
    skip_test "cargo is not installed or not in PATH"
    exit 0
fi

REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RUST_SHELL_DIR="$REPO_ROOT/caelestia-shell"

if [[ ! -f "$RUST_SHELL_DIR/Cargo.toml" ]]; then
    skip_test "caelestia-shell/Cargo.toml does not exist"
    exit 0
fi

if cargo check --manifest-path "$RUST_SHELL_DIR/Cargo.toml" --quiet 2>/dev/null; then
    : # pass
else
    fail "cargo check failed on caelestia-shell"
fi

if [[ $CAELESTIA_TEST_FAILURES -gt 0 ]]; then
    exit 1
fi
