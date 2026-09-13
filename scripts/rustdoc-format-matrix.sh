#!/usr/bin/env bash
# Record, for each installed dated nightly, the rustdoc JSON format_version it emits.
#
# This produces two Phase-0 artifacts in one pass:
#   - the compatibility-matrix rows for the rustdoc producer (blueprint §4.1 [S07], §4.4 [S26])
#   - the supported/unsupported format captures acceptance gate R04 needs
#
# The nightly used here is a recorded PRODUCER IDENTITY, not a build domain for our own code.
# It is written to config/toolchains.toml and feeds ProducerRun provenance.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${LIBENR_CACHE_HOME:-$ROOT/.dev-state/cache}/rustdoc-format-matrix"
mkdir -p "$OUT"

probe="$(mktemp -d)"
trap 'rm -rf "$probe"' EXIT
mkdir -p "$probe/src"
cat > "$probe/Cargo.toml" <<'EOF'
[package]
name = "format-probe"
version = "0.0.0"
edition = "2021"
[lib]
path = "src/lib.rs"
EOF
echo 'pub struct Probe;' > "$probe/src/lib.rs"

printf '%-28s %-14s %s\n' TOOLCHAIN FORMAT_VERSION RUSTC
for tc in $(rustup toolchain list | awk '{print $1}' | grep '^nightly-20'); do
  if ! CARGO_TARGET_DIR="$OUT/target-$tc" cargo "+$tc" rustdoc --quiet \
        --manifest-path "$probe/Cargo.toml" --lib \
        -- -Z unstable-options --output-format json >/dev/null 2>&1; then
    printf '%-28s %-14s %s\n' "$tc" "BUILD-FAILED" "-"
    continue
  fi
  json="$OUT/target-$tc/doc/format_probe.json"
  fv="$(python3 -c 'import json,sys;print(json.load(open(sys.argv[1])).get("format_version","?"))' "$json" 2>/dev/null || echo '?')"
  rv="$(rustc "+$tc" --version | awk '{print $2, $3}' | tr -d '()')"
  printf '%-28s %-14s %s\n' "$tc" "$fv" "$rv"
  cp "$json" "$OUT/rustdoc-${tc}-v${fv}.json" 2>/dev/null || true
done
echo
echo "captures: $OUT"
