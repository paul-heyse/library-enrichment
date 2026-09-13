#!/usr/bin/env bash
# Stop. Two single-shot checks for the failure modes that only become visible at the end
# of a turn. Honours stop_hook_active so it never loops.
set -uo pipefail
. "$(dirname "${BASH_SOURCE[0]}")/_common.sh"
hook_read_input
[ "${HOOK_STOP_ACTIVE:-False}" = "True" ] && exit 0
root="$(hook_root)"
cd "$root" 2>/dev/null || exit 0
problems=""

# 1. A gate claimed passed without a command and a log is an unsupported claim.
if [ -f docs/reports/acceptance.json ]; then
  bad=$(python3 - <<'PY' 2>/dev/null || true
import json
try:
    d = json.load(open("docs/reports/acceptance.json"))
except Exception:
    raise SystemExit
g = d.get("gates", d if isinstance(d, list) else [])
bad = [x.get("gate_id","?") for x in g
       if x.get("status") == "passed" and not (x.get("command") and x.get("log_path"))]
if bad:
    print(", ".join(bad))
PY
)
  [ -n "$bad" ] && problems="${problems}Gates marked 'passed' without a recorded command and log: ${bad}.
A gate is 'passed' only when a command actually ran and its log was written. A missing tool or
credential is 'blocked' with the prerequisite named; never attempted is 'not_run'. Regenerate
with \`just acceptance-report\` rather than hand-editing the report.
"
fi

# 2. Service state must not accumulate in the repository outside .dev-state/.
leak=$(git status --porcelain --ignored 2>/dev/null \
       | awk '{print $NF}' \
       | grep -E '^(blobs|snapshots|capsules|jobs|logs|contexts|bundles)/' | head -5 || true)
[ -n "$leak" ] && problems="${problems}Service state inside the repository (blueprint §2.3):
${leak}
Evidence snapshots, capsules, job records and logs belong in \$LIBENR_HOME (.dev-state/).
"

if [ -n "$problems" ]; then
  printf '%s' "$problems" >&2
  exit 2
fi
exit 0
