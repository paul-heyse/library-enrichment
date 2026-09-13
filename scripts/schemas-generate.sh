#!/usr/bin/env bash
# Regenerate wire schemas from the authoritative Rust types, then the Python boundary DTOs.
#
# Blueprint §6.3: Rust DTOs and the canonical table model are authoritative. Generate versioned
# JSON Schemas from the Rust wire types, then the Pydantic boundary DTOs from those schemas.
# Never hand-edit either output; never hand-maintain the three definitions in parallel.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [ ! -f Cargo.toml ]; then
  echo "schemas-generate: no Cargo workspace yet (phase 0)."
  exit 0
fi
if ! cargo metadata --no-deps --format-version 1 2>/dev/null | grep -q '"name":"emit-schemas"'; then
  echo "schemas-generate: no emit-schemas binary yet (phase 0)."
  exit 0
fi

echo "==> JSON Schemas from Rust wire types"
cargo run --quiet -p enrichment-core --bin emit-schemas -- --out schemas/generated

echo "==> Pydantic boundary DTOs from those schemas"
if [ -x .venv/bin/datamodel-codegen ]; then
  .venv/bin/datamodel-codegen \
    --input schemas/generated --input-file-type jsonschema \
    --output python/enrichment_mcp/_generated \
    --output-model-type pydantic_v2.BaseModel \
    --use-standard-collections --use-union-operator --target-python-version 3.14
else
  echo "    datamodel-codegen absent; add it to [dependency-groups] dev and run \`just sync\`." >&2
fi

echo "==> verifying the result still satisfies the frozen contract"
./scripts/schema-conformance.sh
