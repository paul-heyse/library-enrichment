#!/usr/bin/env bash
# Wrapper: regenerate (when the emitter exists), assert reproducibility, then run the
# corpus and structural checks.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# 1. Reproducible generation.
if [ -f Cargo.toml ] && cargo metadata --no-deps --format-version 1 2>/dev/null \
     | grep -q '"name":"emit-schemas"'; then
  generated_check="$(mktemp -d)"
  trap 'rm -rf "$generated_check"' EXIT
  cargo run --quiet -p enrichment-core --bin emit-schemas -- --out "$generated_check" || exit 1
  if ! diff -ru schemas/generated "$generated_check"; then
    echo "schema-conformance: FAILED -- regenerated schemas differ from schemas/generated." >&2
    echo "  Generation must be reproducible. Commit the regenerated output, or fix the" >&2
    echo "  nondeterminism (map ordering, timestamps) in the emitter." >&2
    exit 1
  fi
else
  echo "schema-conformance: NOTE -- no emit-schemas binary yet; generation check is not_run (phase 0)."
fi

# Generate boundary models into staging too: validating only JSON cannot detect stale DTOs.
model_check="$(mktemp -d)"
trap 'rm -rf "${generated_check:-}" "$model_check"' EXIT
cp pyproject.toml "$model_check/pyproject.toml"
uv run datamodel-codegen \
  --input schemas/generated --input-file-type jsonschema --output "$model_check/models" \
  --output-model-type pydantic_v2.BaseModel --use-standard-collections --use-union-operator \
  --target-python-version 3.14 --field-constraints --formatters ruff-format --disable-timestamp --deserialize-default-values enum || exit 1
if ! diff -ru --exclude=__pycache__ --exclude=.ruff_cache python/enrichment_mcp/_generated "$model_check/models"; then
  echo "schema-conformance: FAILED -- generated Python models are stale" >&2
  exit 1
fi
uv run python scripts/schema-conformance.py
