#!/usr/bin/env bash
# Verify every file delivered in the specification bundle still has its delivered bytes.
#
# MANIFEST.sha256 was written against the bundle's original layout. PATHMAP.tsv records
# where each of those files now lives. This resolves one through the other, which is
# stronger than `sha256sum -c`: it proves AND documents that the restructure preserved
# every delivered byte.
#
# Files mapped `in-place` are frozen contracts. If this fails on one of them, something
# edited a frozen contract -- the fix is to revert, not to regenerate the manifest.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# ADR-0036/0037 preserve the original manifest and relocate only superseded product-skill
# bytes through the new, independently hashed predecessor map. Both seals are mandatory.
verify_bundle() {
local MANIFEST="$1"
local PATHMAP="$2"
local ok bad missing unmapped total want orig cur got disp

[ -f "$MANIFEST" ] || { echo "provenance: missing $MANIFEST" >&2; exit 1; }
[ -f "$PATHMAP" ]  || { echo "provenance: missing $PATHMAP"  >&2; exit 1; }

ok=0; bad=0; missing=0; unmapped=0

while read -r want orig; do
  [ -n "${want:-}" ] || continue
  case "$want" in \#*) continue ;; esac

  cur="$(awk -F'\t' -v o="$orig" '$1==o {print $2; exit}' "$PATHMAP")"
  if [ -z "$cur" ]; then
    printf 'UNMAPPED  %s\n' "$orig" >&2; unmapped=$((unmapped+1)); continue
  fi
  if [ ! -f "$ROOT/$cur" ]; then
    printf 'MISSING   %s -> %s\n' "$orig" "$cur" >&2; missing=$((missing+1)); continue
  fi

  got="$(sha256sum "$ROOT/$cur" | cut -d' ' -f1)"
  if [ "$got" = "$want" ]; then
    ok=$((ok+1))
  else
    disp="$(awk -F'\t' -v o="$orig" '$1==o {print $3; exit}' "$PATHMAP")"
    printf 'ALTERED   %s (%s)\n          expected %s\n          actual   %s\n' \
      "$cur" "$disp" "$want" "$got" >&2
    if [ "$disp" = "in-place" ]; then
      printf '          This is a FROZEN CONTRACT. Revert it, or open an ADR (/adr) that\n' >&2
      printf '          re-freezes the bundle under a new dated directory.\n' >&2
    fi
    bad=$((bad+1))
  fi
done < <(sed 's/  */\t/' "$MANIFEST" | awk -F'\t' '{print $1"\t"$2}' | tr '\t' ' ' | awk '{print $1, $2}')

total=$((ok+bad+missing+unmapped))
printf 'provenance: %d/%d verified' "$ok" "$total"
[ "$bad" -gt 0 ]      && printf ', %d ALTERED' "$bad"
[ "$missing" -gt 0 ]  && printf ', %d MISSING' "$missing"
[ "$unmapped" -gt 0 ] && printf ', %d UNMAPPED' "$unmapped"
printf '\n'

[ $((bad+missing+unmapped)) -eq 0 ]
}

CURRENT="$ROOT/docs/provenance/bundle-2026-09-17-native-delivery"
PREVIOUS="$ROOT/docs/provenance/bundle-2026-09-15-research-v2"
verify_bundle "$CURRENT/MANIFEST.sha256" "$CURRENT/PATHMAP.tsv"
verify_bundle "$PREVIOUS/MANIFEST.sha256" "$CURRENT/RESEARCH_V2_PATHMAP.tsv"
verify_bundle "$ROOT/docs/provenance/bundle-2026-09-13/MANIFEST.sha256" "$PREVIOUS/PREDECESSOR_PATHMAP.tsv"
