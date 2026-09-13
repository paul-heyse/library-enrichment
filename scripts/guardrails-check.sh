#!/usr/bin/env bash
# Detect changes to the enforcement layer.
#
# `scripts/hooks/pre_edit.sh` PREVENTS an agent from editing the files below. It cannot detect
# a change made another way -- by the operator, by a bypass, or by an edit to pre_edit.sh
# itself, since the deny list that protects those files lives inside one of them. A guard
# cannot check itself.
#
# So this is the complementary half: a digest over the layer, compared against a recorded
# manifest. A change does not fail forever -- it fails until someone re-records it deliberately,
# which puts a visible, reviewable line in the diff next to the change it covers. That is the
# property worth having. It is detection, not prevention, and it is not tamper-proof: anyone who
# can edit the layer can also re-record the manifest. What it removes is the possibility of the
# change being *quiet*.
#
# Run by `just guardrails-check` and by `just ci`.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# Not under docs/provenance/: that directory records the specification as DELIVERED, and this
# manifest records the enforcement layer as it stands now. Conflating the two would make a
# deliberate guardrail change look like tampering with frozen provenance.
MANIFEST="config/guardrails.sha256"

# The files `.claude/rules/process-immutable.md` names as "the rules an agent works under".
# A file that is expected but missing is recorded as ABSENT rather than skipped, so deleting a
# rule is as visible as editing one.
current() {
  {
    # `guardrails-check.sh` covers itself. It is not in pre_edit.sh's deny list, so editing the
    # checker is a second escape hatch alongside re-recording -- listing it here does not close
    # that (nothing can, from inside), but it does mean the edit cannot be silent.
    printf '%s\n' AGENTS.md CLAUDE.md .claude/settings.json scripts/env.sh \
                  scripts/guardrails-check.sh
    find .claude/rules -type f -name '*.md' 2>/dev/null
    find scripts/hooks -type f -name '*.sh' 2>/dev/null
  } | LC_ALL=C sort | while IFS= read -r p; do
    if [ -f "$p" ]; then
      printf '%s  %s\n' "$(sha256sum "$p" | cut -d' ' -f1)" "$p"
    else
      printf 'ABSENT  %s\n' "$p"
    fi
  done
}

if [ "${1:-}" = "--record" ]; then
  if [ "${LIBENR_ALLOW_GUARDRAIL_RERECORD:-0}" != "1" ]; then
    echo "guardrails-check: re-recording the enforcement-layer manifest is an operator action." >&2
    echo "  It asserts that a change to the rules an agent works under was intended." >&2
    echo "  Re-run with LIBENR_ALLOW_GUARDRAIL_RERECORD=1 if that is what you mean." >&2
    exit 2
  fi
  current > "$MANIFEST"
  echo "guardrails-check: recorded $(wc -l < "$MANIFEST") file(s) -> ${MANIFEST}"
  exit 0
fi

if [ ! -f "$MANIFEST" ]; then
  echo "guardrails-check: ${MANIFEST} does not exist." >&2
  echo "  Record it with: LIBENR_ALLOW_GUARDRAIL_RERECORD=1 just guardrails-record" >&2
  exit 1
fi

if diff_out=$(current | diff - "$MANIFEST" 2>&1); then
  echo "guardrails-check: OK -- $(wc -l < "$MANIFEST") enforcement-layer file(s) unchanged"
  exit 0
fi

echo "guardrails-check: FAILED -- the enforcement layer changed." >&2
echo >&2
printf '%s\n' "$diff_out" >&2
echo >&2
echo "  These are the rules an agent works under, not part of the work." >&2
echo "  If the change was intended, record it in docs/adr/ and then re-record:" >&2
echo "    LIBENR_ALLOW_GUARDRAIL_RERECORD=1 just guardrails-record" >&2
echo "  If it was not, revert it." >&2
exit 1
