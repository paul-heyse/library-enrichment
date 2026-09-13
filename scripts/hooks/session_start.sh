#!/usr/bin/env bash
# SessionStart. Environment truth, hard-capped at ~25 lines -- this output is prepended to
# every session and re-injected after every compaction.
set -uo pipefail
. "$(dirname "${BASH_SOURCE[0]}")/_common.sh"
hook_read_input
root="$(hook_root)"
cd "$root" 2>/dev/null || exit 0

{
  if [ -f STATUS.md ]; then sed -n '1,8p' STATUS.md; fi

  echo
  echo "toolchain: $(rustc --version 2>/dev/null || echo 'rustc UNAVAILABLE')"
  if command -v ty >/dev/null 2>&1 || [ -x .venv/bin/ty ]; then
    echo "ty:        $( { .venv/bin/ty --version 2>/dev/null || ty --version 2>/dev/null; } | head -1)"
  else
    echo "ty:        NOT INSTALLED -- Python semantic gates are 'blocked', not skipped."
    echo "           pyrefly/pyright are present on this workstation but are NOT this"
    echo "           project's engine. Do not substitute them (blueprint §1.1)."
  fi

  if [ -f docs/reports/acceptance.json ]; then
    python3 - <<'PY' 2>/dev/null || true
import json
try:
    d = json.load(open("docs/reports/acceptance.json"))
except Exception:
    raise SystemExit
g = d.get("gates", d if isinstance(d, list) else [])
from collections import Counter
c = Counter(x.get("status", "not_run") for x in g)
print("gates:     %d passed / %d failed / %d blocked / %d not_run  (of %d)" % (
    c["passed"], c["failed"], c["blocked"], c["not_run"], len(g)))
PY
  fi
} 2>/dev/null | head -25

exit 0
