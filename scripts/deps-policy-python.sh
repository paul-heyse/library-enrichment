#!/usr/bin/env bash
# Python half of the dependency policy (blueprint §1.1). cargo-deny covers the Rust half.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOCK="${ROOT}/uv.lock"

[ -f "$LOCK" ] || { echo "deps-policy(python): no uv.lock yet (phase 0)"; exit 0; }

BANNED='fastapi grpcio redis lancedb chromadb qdrant-client weaviate-client pinecone-client sentence-transformers torch transformers faiss-cpu faiss-gpu'
found=""
for pkg in $BANNED; do
  grep -qE "^name = \"${pkg}\"$" "$LOCK" && found="${found} ${pkg}"
done

# pyrefly/pyright/mypy are a separate class: not "heavy", just not this project's engine.
WRONG_ENGINE='pyrefly pyright basedpyright mypy'
engine=""
for pkg in $WRONG_ENGINE; do
  grep -qE "^name = \"${pkg}\"$" "$LOCK" && engine="${engine} ${pkg}"
done

rc=0
if [ -n "$found" ]; then
  echo "deps-policy(python): FAILED -- banned dependency class in uv.lock:${found}" >&2
  echo "  Blueprint §1.1: no graph database, embeddings, vector database, GPU workload," >&2
  echo "  gRPC, Redis or additional server framework in the initial implementation." >&2
  rc=1
fi
if [ -n "$engine" ]; then
  echo "deps-policy(python): FAILED -- wrong Python semantic engine in uv.lock:${engine}" >&2
  echo "  Blueprint §1.1 and §5.4 bind Python semantics to Astral ty." >&2
  rc=1
fi
[ "$rc" -eq 0 ] && echo "deps-policy(python): OK"
exit $rc
