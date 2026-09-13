#!/usr/bin/env bash
# Assert the active Rust toolchain is the one rust-toolchain.toml pins.
#
# This workstation's rustup default is nightly. If rust-toolchain.toml is missing, ignored,
# or overridden by an inherited RUSTUP_TOOLCHAIN, every build silently targets nightly and
# gate R09 ("compile probe passes on nightly but fails on project stable") is untestable.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

want="$(python3 -c 'import tomllib,sys;print(tomllib.load(open(sys.argv[1],"rb"))["toolchain"]["channel"])' "$ROOT/rust-toolchain.toml")"
got="$(cd "$ROOT" && rustc --version | awk '{print $2}')"

if [ -n "${RUSTUP_TOOLCHAIN:-}" ]; then
  echo "toolchain-check: FAIL -- RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN} is set and overrides rust-toolchain.toml." >&2
  echo "                 Unset it; scripts/env.sh does this for direnv and agent sessions." >&2
  exit 1
fi

if [ "$got" != "$want" ]; then
  echo "toolchain-check: FAIL -- active rustc is ${got}, rust-toolchain.toml pins ${want}." >&2
  echo "                 This workstation's rustup default is nightly ($(rustup show active-toolchain 2>/dev/null | head -1))." >&2
  echo "                 Run: rustup toolchain install ${want}" >&2
  exit 1
fi
echo "toolchain-check: OK -- rustc ${got} matches the pin"
