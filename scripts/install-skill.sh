#!/usr/bin/env bash
# Compatibility command name only; all installation behavior lives in one implementation.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec uv run --frozen --project "$ROOT" python "$ROOT/scripts/skill_install.py" "$@"
