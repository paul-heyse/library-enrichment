#!/usr/bin/env bash
# Wrapper: regenerate (when the emitter exists), assert reproducibility, then run the
# corpus and structural checks.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# 1. Reproducible generation.
if [ -f Cargo.toml ] && cargo metadata --no-deps --format-version 1 2>/dev/null \
     | grep -q '"name":"emit-schemas"'; then
  cargo run --quiet -p enrichment-core --bin emit-schemas -- --out schemas/generated || exit 1
  if ! git diff --exit-code --quiet -- schemas/generated; then
    echo "schema-conformance: FAILED -- regenerating schemas/generated changed the tree." >&2
    echo "  Generation must be reproducible. Commit the regenerated output, or fix the" >&2
    echo "  nondeterminism (map ordering, timestamps) in the emitter." >&2
    git diff --stat -- schemas/generated >&2
    exit 1
  fi
else
  echo "schema-conformance: NOTE -- no emit-schemas binary yet; generation check is not_run (phase 0)."
fi

# 2 and 3.
if [ -x .venv/bin/python ]; then .venv/bin/python scripts/schema-conformance.py
else python3 scripts/schema-conformance.py; fi
