#!/usr/bin/env bash
# Assert the real XDG service paths were untouched.
#
# Development redirects service state into .dev-state/ (scripts/env.sh). This proves the
# redirection actually held: if a code path resolved the production directory policy instead
# of the configured root, it shows up here rather than silently accumulating in $HOME.
#
#   scripts/state-leak-check.sh --snapshot   record the current state
#   scripts/state-leak-check.sh              compare against the recorded state
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SNAP="${ROOT}/.dev-state/.xdg-snapshot"
REAL_CACHE="${XDG_CACHE_HOME:-$HOME/.cache}/library-enrichment"
REAL_DATA="${XDG_DATA_HOME:-$HOME/.local/share}/library-enrichment"
# The daemon's socket resolver reaches XDG_RUNTIME_DIR before XDG_CACHE_HOME, so a run that
# resolved the production policy would write here and neither path above would notice.
# See crates/enrichment-daemon/src/paths.rs and ADR 0006.
REAL_RUNTIME="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}/library-enrichment"

# Path, type, mode AND content. Size and mtime alone would miss an in-place rewrite that kept
# the length -- a snapshot pointer flipped to another ID, a config value changed, a journal
# entry rewritten. Those are exactly the writes worth catching, and they are all same-length.
# Sockets and FIFOs are listed by type rather than read, because reading one blocks.
digest() {
  for d in "$REAL_CACHE" "$REAL_DATA" "$REAL_RUNTIME"; do
    if [ -e "$d" ]; then
      find "$d" \( -type f -printf '%P\t%y\t%m\t' -exec sha256sum -b {} \; \) -o \
                \( -type l -printf '%P\t%y\t%m\t%l\n' \) -o \
                \( ! -type f ! -type l -printf '%P\t%y\t%m\n' \) 2>/dev/null | LC_ALL=C sort
    else
      printf 'ABSENT\t%s\n' "$d"
    fi
  done | sha256sum | cut -d' ' -f1
}

if [ "${1:-}" = "--snapshot" ]; then
  mkdir -p "$(dirname "$SNAP")"
  digest > "$SNAP"
  echo "state-leak-check: snapshot recorded ($(cat "$SNAP"))"
  exit 0
fi

now="$(digest)"
if [ ! -f "$SNAP" ]; then
  mkdir -p "$(dirname "$SNAP")"; echo "$now" > "$SNAP"
  echo "state-leak-check: no prior snapshot; recorded baseline ($now)"
  exit 0
fi

before="$(cat "$SNAP")"
if [ "$now" != "$before" ]; then
  echo "state-leak-check: FAILED -- the real XDG service paths changed." >&2
  echo "  ${REAL_CACHE}" >&2
  echo "  ${REAL_DATA}" >&2
  echo "  before ${before}" >&2
  echo "  after  ${now}" >&2
  echo "  Development state belongs in \$LIBENR_HOME (.dev-state/). Something resolved the" >&2
  echo "  production directory policy instead of the configured root." >&2
  exit 1
fi
echo "state-leak-check: OK -- real XDG service paths unchanged"
