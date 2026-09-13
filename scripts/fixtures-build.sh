#!/usr/bin/env bash
# Build the deterministic Rust fixtures the Phase 1 gates run against.
#
# For each release under tests/fixtures/crates/enr-fixture-<version>/ this produces, into
# tests/fixtures/:
#
#   rustdoc/enr-fixture-<v>-default.json          rustdoc JSON, default features
#   rustdoc/enr-fixture-<v>-all-features.json     rustdoc JSON, --all-features
#   upstream/docsrs/enr-fixture-<v>.json.zst      the all-features capture, zstd-compressed as
#                                                 docs.rs serves it (the fixture manifest says
#                                                 all-features = true), for releases >= 0.2.0
#   upstream/static/enr-fixture-<v>.crate         the `.crate` tarball (`cargo package`)
#   upstream/index/enr-fixture.ndjson             a sparse-index document with real checksums
#   upstream/api/enr-fixture-<v>.json             a crates.io version record
#   rustdoc/PROVENANCE.toml                       toolchain, commit, target, features, format
#                                                 version and sha256 for every capture
#
# Release 0.1.0 deliberately has NO hosted JSON: that is gate R03's "hosted rustdoc JSON
# missing" case. Release 0.2.0 has it.
#
# The rustdoc producer identity is the dated nightly from config/toolchains.toml, never a bare
# `+nightly`. Builds go to a target directory under the development state sandbox, never into
# the fixture directory, and never touch the service's own workspace (blueprint §2.3).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

: "${LIBENR_CACHE_HOME:?fixtures-build needs LIBENR_CACHE_HOME (direnv or scripts/env.sh sets it)}"

TOOLCHAIN="$(sed -n 's/^toolchain = "\(.*\)"/\1/p' config/toolchains.toml | head -n 1)"
[ -n "$TOOLCHAIN" ] || { echo "fixtures-build: no rustdoc_producer.toolchain in config/toolchains.toml" >&2; exit 1; }
rustup run "$TOOLCHAIN" rustc --version >/dev/null 2>&1 || {
  echo "fixtures-build: toolchain $TOOLCHAIN is not installed; run: rustup toolchain install $TOOLCHAIN" >&2
  exit 1
}
command -v zstd >/dev/null || { echo "fixtures-build: zstd CLI is required" >&2; exit 1; }

TARGET_DIR="$LIBENR_CACHE_HOME/fixtures-target"
OUT="$ROOT/tests/fixtures"
mkdir -p "$OUT/rustdoc" "$OUT/upstream/docsrs" "$OUT/upstream/static" "$OUT/upstream/index" "$OUT/upstream/api" "$TARGET_DIR"

HOST_TARGET="$(rustc -vV | sed -n 's/^host: //p')"
RUSTC_RELEASE="$(rustup run "$TOOLCHAIN" rustc --version | sed 's/^rustc //; s/ (.*//')"
RUSTC_COMMIT="$(rustup run "$TOOLCHAIN" rustc -vV | sed -n 's/^commit-hash: \(.\{9\}\).*/\1/p')"
TODAY="$(date -u +%Y-%m-%d)"

PROV="$OUT/rustdoc/PROVENANCE.toml"
{
  echo "# Provenance for every committed fixture capture. Regenerate with: just fixtures-build"
  echo "# These are OUR captures of OUR fixture crate (ADR 0002): no third-party licence applies."
  echo "generated_at = \"$TODAY\""
  echo "toolchain = \"$TOOLCHAIN\""
  echo "rustc_release = \"$RUSTC_RELEASE\""
  echo "rustc_commit_hash = \"$RUSTC_COMMIT\""
  echo "host_target = \"$HOST_TARGET\""
} > "$PROV"

INDEX="$OUT/upstream/index/enr-fixture.ndjson"
: > "$INDEX"

for crate_dir in "$ROOT"/tests/fixtures/crates/enr-fixture-*/; do
  version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$crate_dir/Cargo.toml" | head -n 1)"
  echo "==> enr-fixture $version"
  manifest="$crate_dir/Cargo.toml"

  # A lockfile is committed with each fixture so `cargo package` and `cargo rustdoc` are
  # reproducible; the fixture has no dependencies, so this is a formality that stays stable.
  (cd "$crate_dir" && CARGO_TARGET_DIR="$TARGET_DIR" cargo generate-lockfile --offline 2>/dev/null || CARGO_TARGET_DIR="$TARGET_DIR" cargo generate-lockfile)

  for profile in default all-features; do
    extra=()
    [ "$profile" = "all-features" ] && extra=(--all-features)
    CARGO_TARGET_DIR="$TARGET_DIR/$version-$profile" \
      cargo "+$TOOLCHAIN" rustdoc --quiet --manifest-path "$manifest" --lib "${extra[@]}" \
        -- -Z unstable-options --output-format json
    src="$TARGET_DIR/$version-$profile/doc/enr_fixture.json"
    dst="$OUT/rustdoc/enr-fixture-$version-$profile.json"
    cp "$src" "$dst"
    fmt="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["format_version"])' "$dst")"
    sha="$(sha256sum "$dst" | cut -d' ' -f1)"
    {
      echo
      echo "[[capture]]"
      echo "file = \"rustdoc/enr-fixture-$version-$profile.json\""
      echo "crate = \"enr-fixture\""
      echo "version = \"$version\""
      echo "features = \"$profile\""
      echo "target = \"$HOST_TARGET\""
      echo "format_version = $fmt"
      echo "sha256 = \"$sha\""
    } >> "$PROV"
    echo "    $profile: format_version=$fmt"
  done

  # docs.rs serves the all-features build (the manifest says so) zstd-compressed. 0.1.0 is
  # left without hosted JSON on purpose: gate R03.
  if [ "$version" != "0.1.0" ]; then
    zstd -q -f -19 "$OUT/rustdoc/enr-fixture-$version-all-features.json" \
      -o "$OUT/upstream/docsrs/enr-fixture-$version.json.zst"
    zstd -q -f -19 "$OUT/rustdoc/enr-fixture-$version-all-features.json" \
      -o "$OUT/upstream/docsrs/enr-fixture-$version-$HOST_TARGET.json.zst"
  fi

  # The `.crate` tarball, exactly as crates.io would store it.
  CARGO_TARGET_DIR="$TARGET_DIR/$version-package" \
    cargo package --quiet --manifest-path "$manifest" --no-verify --allow-dirty
  cp "$TARGET_DIR/$version-package/package/enr-fixture-$version.crate" "$OUT/upstream/static/"
  cksum="$(sha256sum "$OUT/upstream/static/enr-fixture-$version.crate" | cut -d' ' -f1)"
  size="$(stat -c %s "$OUT/upstream/static/enr-fixture-$version.crate")"

  # One sparse-index line per version, with the real checksum.
  python3 - "$INDEX" "$version" "$cksum" <<'PY'
import json, sys
index, version, cksum = sys.argv[1:4]
entry = {
    "name": "enr-fixture",
    "vers": version,
    "deps": [],
    "cksum": cksum,
    "features": {"default": ["std"], "std": [], "extra": []},
    "yanked": False,
    "rust_version": "1.70",
    "v": 2,
    "pubtime": "2026-09-13T00:00:00Z",
}
with open(index, "a") as f:
    f.write(json.dumps(entry, separators=(",", ":")) + "\n")
PY

  # A crates.io version record with the fields the service reads.
  python3 - "$OUT/upstream/api/enr-fixture-$version.json" "$version" "$cksum" "$size" <<'PY'
import json, sys
out, version, cksum, size = sys.argv[1:5]
doc = {"version": {
    "num": version, "yanked": False, "checksum": cksum, "crate_size": int(size),
    "license": "MIT OR Apache-2.0", "edition": "2021", "rust_version": "1.70",
    "repository": "https://github.com/paul-heyse/library-enrichment",
    "documentation": "https://docs.rs/enr-fixture", "homepage": None,
    "created_at": "2026-09-13T00:00:00.000000Z",
    "features": {"default": ["std"], "std": [], "extra": []}, "has_lib": True,
}}
with open(out, "w") as f:
    json.dump(doc, f, indent=2)
    f.write("\n")
PY
done

echo "fixtures-build: wrote captures under tests/fixtures/{rustdoc,upstream}; provenance in $PROV"
