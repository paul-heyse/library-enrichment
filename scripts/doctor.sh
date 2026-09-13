#!/usr/bin/env bash
# Report which tools this repository needs, and which are actually present.
#
# A missing hard requirement FAILS. Reporting a tool as present when it is not is exactly the
# kind of untruthful claim the acceptance discipline exists to prevent.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
hard_missing=0

req()  { # name, command, why
  if command -v "$2" >/dev/null 2>&1; then
    printf '  %-22s %s\n' "$1" "$( "$2" --version 2>&1 | head -1 )"
  else
    printf '  %-22s MISSING (required) -- %s\n' "$1" "$3"; hard_missing=$((hard_missing+1))
  fi
}
opt()  {
  if command -v "$2" >/dev/null 2>&1; then
    printf '  %-22s %s\n' "$1" "$( "$2" --version 2>&1 | head -1 )"
  else
    printf '  %-22s absent -- %s\n' "$1" "$3"
  fi
}

echo "core:"
req rustc   rustc   "the service is written in Rust"
req cargo   cargo   "workspace build"
req uv      uv      "the only supported Python package manager"
req just    just    "the task surface"
req python3 python3 "scripts and the adapter"

echo
echo "producers and analysis:"
# ty is a hard requirement: blueprint §1.1 binds Python semantics to it, and substituting
# another checker is never acceptable. Absence makes the Python semantic gates `blocked`.
if command -v ty >/dev/null 2>&1 || [ -x "$ROOT/.venv/bin/ty" ]; then
  printf '  %-22s %s\n' ty "$( { "$ROOT/.venv/bin/ty" --version 2>/dev/null || ty --version 2>/dev/null; } | head -1 )"
else
  printf '  %-22s MISSING (required) -- add an exact pin to [dependency-groups] dev, then `just sync`\n' ty
  printf '  %-22s NOTE: pyrefly and pyright are present on this workstation and globally\n' ''
  printf '  %-22s       plugin-enabled. Neither is the engine for this project (§1.1).\n' ''
  printf '  %-22s       Their presence is not a substitute; the gates are `blocked`.\n' ''
  hard_missing=$((hard_missing+1))
fi
req  rust-analyzer rust-analyzer "Rust semantic evidence over LSP"
opt  griffe        griffe        "installed into .venv by \`just sync\`"

echo
echo "sandboxing (build and runtime execution profiles):"
have_sandbox=0
for s in bwrap podman docker; do
  if command -v "$s" >/dev/null 2>&1; then printf '  %-22s present\n' "$s"; have_sandbox=1
  else printf '  %-22s absent\n' "$s"; fi
done
[ "$have_sandbox" -eq 0 ] && printf '  %-22s no isolation available: build/runtime profiles must return POLICY_DENIED\n' '->'

echo
echo "optional tooling:"
for t in cargo-nextest cargo-deny ast-grep taplo typos direnv sccache; do
  cmd="${t#cargo-}"; [ "$t" != "$cmd" ] && cmd="cargo"
  if command -v "${t}" >/dev/null 2>&1 || command -v "${t#cargo-}" >/dev/null 2>&1; then
    printf '  %-22s present\n' "$t"
  else printf '  %-22s absent\n' "$t"; fi
done

echo
echo "toolchain pin:"
"$ROOT/scripts/toolchain-check.sh" || hard_missing=$((hard_missing+1))

echo
if [ "$hard_missing" -gt 0 ]; then
  echo "doctor: ${hard_missing} hard requirement(s) missing." >&2
  exit 1
fi
echo "doctor: all hard requirements present"
