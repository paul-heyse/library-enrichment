#!/usr/bin/env bash
# Print the `export` lines that tell the sandbox test tier which images are qualified.
#
# Sourced, or `eval`'d, by anything that runs the tests: `just test-python`, `just gate-phase`.
# One definition, because the alternative is what it replaced -- `gate-phase` read the receipt
# and `just ci` did not, so the same tree reported 13 gates `passed` one way and 17 `blocked`
# the other. A gate result that depends on which recipe you typed is not evidence.
#
# Absent a receipt this prints nothing and says so on stderr. The sandbox tests then skip and
# their gates report `blocked` with the setup recipe named -- the truthful state, never a false
# pass (blueprint §13, .claude/rules/evidence-truthfulness.md).
#
# The receipt lives beside the images it qualifies, in the execution root: qualification is a
# property of an image *in a root*, so a receipt from another root says nothing here.
set -euo pipefail

root="${LIBENR_EXECUTION_ROOT:-${LIBENR_CACHE_HOME:-/nonexistent}/podman}"
receipt="${root}/admitted-images.json"

if [ ! -f "$receipt" ]; then
  echo "execution: no qualification receipt at ${receipt}" >&2
  echo "  sandbox gates will report blocked; run 'just execution-images --apply'" >&2
  echo "  then 'just execution-qualify --apply'" >&2
  exit 0
fi

uv run --frozen python - "$receipt" "$root" <<'PY'
import json
from pathlib import Path
import shlex
import sys

with open(sys.argv[1]) as handle:
    receipt = json.load(handle)
recorded = Path(receipt["execution_root"])
selected = Path(sys.argv[2])
if not recorded.is_absolute() or recorded.resolve() != selected.resolve():
    raise SystemExit("execution: qualification receipt does not match the selected root")
if set(receipt["images"]) - {"rust", "python"}:
    raise SystemExit("execution: qualification receipt contains an unknown ecosystem")
print(f'export LIBENR_EXECUTION_TEST_ROOT={shlex.quote(str(recorded))}')
for ecosystem, image in receipt["images"].items():
    print(f'export LIBENR_EXECUTION_TEST_{ecosystem.upper()}={shlex.quote(image)}')
PY

echo "execution: images admitted by ${receipt}" >&2
