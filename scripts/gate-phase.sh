#!/usr/bin/env bash
# Run the acceptance gate for one phase (blueprint §13).
#
# Gates are additive and phase-scoped. A check whose target does not exist yet reports
# not_run with a reason -- never a false pass, and never a hard failure that tempts anyone
# to weaken the gate.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
n="${1:-}"
case "$n" in 0|1|2|3|4|5|6) ;; *) echo "usage: gate-phase.sh <0-6>" >&2; exit 2 ;; esac

# Phase 4 onwards needs admitted execution images. Export them from the qualification receipt
# so the sandbox tier actually runs. `just test-python` reads the same script, so a gate result
# cannot depend on which recipe was typed.
eval "$(bash "${ROOT}/scripts/execution-env.sh")"

fails=0
step() { # description, command...
  local d="$1"; shift
  printf '\n--- %s\n' "$d"
  if "$@"; then :; else fails=$((fails+1)); printf '    ^ FAILED\n'; fi
}

echo "=== Phase ${n} gate ==="

# Always-on invariants: these hold from commit #1 and must never regress.
step "toolchain pin"        ./scripts/toolchain-check.sh
step "bundle provenance"    ./scripts/provenance-check.sh
step "hook behaviour"       ./scripts/test-hooks.sh
step "rule corpus"          ast-grep test

case "$n" in
  0)
    # Phase 0 runs the suite like every other phase. It used to skip straight to the join,
    # which meant `gate-phase 0` reported against whatever docs/reports/logs/*.json happened to
    # be on disk -- possibly produced by a different tree. A gate that reads `passed` from a
    # stale log is the failure mode this whole pipeline exists to prevent.
    step "tests"                just test
    step "schema conformance"   ./scripts/schema-conformance.sh
    step "dependency policy"    just deps-policy
    step "acceptance report"    just acceptance-report
    step "acceptance integrity" just acceptance-check
    ;;
  *)
    step "format"               just fmt-check
    step "lint"                 just lint
    step "compile"              just check
    step "tests"                just test
    step "live acceptance"      just test-live
    step "client acceptance"    just test-client
    step "schema conformance"   ./scripts/schema-conformance.sh
    step "dependency policy"    just deps-policy
    step "state isolation"      ./scripts/state-leak-check.sh
    step "acceptance report"    just acceptance-report
    step "acceptance integrity" just acceptance-check
    ;;
esac

# Report this phase's gates truthfully.
printf '\n--- gates registered for phase %s\n' "$n"
python3 - "$n" <<'PY' || fails=$((fails+1))
import json, sys, tomllib
from collections import Counter
from pathlib import Path
n = int(sys.argv[1])
reg = tomllib.load(open("tests/gates.toml", "rb"))["gates"]
ids = sorted(g for g, s in reg.items() if s["phase"] == n)
rep = {}
p = Path("docs/reports/acceptance.json")
if p.exists():
    rep = {g["gate_id"]: g for g in json.loads(p.read_text())["gates"]}
c = Counter()
for g in ids:
    st = rep.get(g, {}).get("status", "not_run")
    if not reg[g].get("superseded"):
        c[st] += 1
    lim = rep.get(g, {}).get("limitation", "")
    print(f"  {g}  {st:<8} {reg[g]['scenario'][:58]}" + (f"\n        {lim}" if lim and st != 'passed' else ""))
print(f"\n  phase {n}: {c['passed']} passed / {c['failed']} failed / "
      f"{c['blocked']} blocked / {c['not_run']} not_run  (of {len(ids)})")
if c['not_run'] or c['failed'] or c['blocked']:
    print(f"  Phase {n} is NOT complete. A gate is passed only when a test actually ran.")
    sys.exit(1)
PY

printf '\n=== phase %s: %d check(s) failed ===\n' "$n" "$fails"
[ "$fails" -eq 0 ]
