#!/usr/bin/env bash
# Start an implementation plan under docs/plans/NN-<slug>.md.
#
# A plan records HOW work is sequenced and verified. A decision record (docs/adr/) records
# WHAT was decided and why. The two have different lifecycles, which is why they live in
# different directories: a plan is edited while it is being executed; an ADR is immutable
# once accepted.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
slug="${1:-}"

if [ -z "$slug" ]; then
  echo "usage: just plan <kebab-case-slug>" >&2
  exit 2
fi
if ! printf '%s' "$slug" | grep -Eq '^[a-z0-9-]+$'; then
  echo "error: slug '$slug' must be lowercase kebab-case" >&2
  exit 2
fi

mkdir -p "$root/docs/plans"
last=0
for existing in "$root"/docs/plans/[0-9][0-9]-*.md; do
  [ -e "$existing" ] || continue
  n="$(basename "$existing" | cut -c1-2)"
  [ "$((10#$n))" -gt "$last" ] && last="$((10#$n))"
done
next="$(printf '%02d' $((last + 1)))"
file="$root/docs/plans/${next}-${slug}.md"

if [ -e "$file" ]; then
  echo "exists: $file" >&2
  exit 1
fi

cat > "$file" <<EOF
---
title: ${slug}
status: draft
date: $(date +%F)
adrs: []
phase: 0
---

# ${slug}

## Context

Why this work is being done: the problem it addresses and the intended outcome.

## Decisions

The decision records this plan implements, by ID. A decision that is not yet recorded is a
blocker, not a paragraph here -- \`just adr-new\` first.

## Plan

The sequence. Each step names what changes and what proves it.

## Verification

How the work is checked end to end. Name the recipes and tests, not "run the tests".

## Open items

What is deliberately unresolved. Anything deferred with a trigger belongs in
\`docs/adr/register.md\` as well, so it has an owner and a date.

## Outcome (recorded after implementation)

### What was built

What actually exists now, with a charter §D evidence label for each claim.

### A mistake made and corrected

At least one. A plan whose outcome records no mistake was either not executed or not read
honestly.

### Deviations from the plan, deliberate

What was done differently and why. A deviation that changed a decision needs an ADR, not a
paragraph here.
EOF

echo "${file#"$root"/}"
