#!/usr/bin/env bash
# Behaviour tests for scripts/adr.py and scripts/check_register.py.
#
# These are the enforcement layer for the decision process, so they need regression cover for
# the same reason scripts/test-hooks.sh exists: a guard that is never exercised is a guard
# nobody knows is broken.
#
# Everything runs against a throwaway git repository in a temp directory. adr.py resolves its
# ROOT from its own path (`parents[1]`), so copying the two scripts into $TMP/scripts/ is
# enough to point the whole tool at the fixture. Nothing here touches the real repository.
set -uo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pass=0
fail=0

TMP="$(mktemp -d)"
cleanup() { rm -rf "$TMP"; }
trap cleanup EXIT

ok() { pass=$((pass + 1)); printf '  ok    %s\n' "$1"; }
no() { fail=$((fail + 1)); printf '  FAIL  %s\n' "$1"; }

# Assert the command fails AND its stderr mentions the expected fragment. A lint that fails
# for the wrong reason is not a passing test.
expect_error() {
  local label="$1" fragment="$2"
  shift 2
  local output status
  output="$("$@" 2>&1)"
  status=$?
  if [ "$status" -eq 0 ]; then
    no "$label (exited 0; expected failure)"
  elif printf '%s' "$output" | grep -qF -- "$fragment"; then
    ok "$label"
  else
    no "$label (failed, but not with: $fragment)"
    printf '%s\n' "$output" | sed 's/^/        /'
  fi
}

expect_clean() {
  local label="$1"
  shift
  local output status
  output="$("$@" 2>&1)"
  status=$?
  if [ "$status" -eq 0 ]; then
    ok "$label"
  else
    no "$label (exited $status; expected success)"
    printf '%s\n' "$output" | sed 's/^/        /'
  fi
}

adr() { python3 "$TMP/scripts/adr.py" "$@"; }
reg() { python3 "$TMP/scripts/check_register.py" "$@"; }

# --------------------------------------------------------------------------- fixture
mkdir -p "$TMP/scripts" "$TMP/docs/adr" "$TMP/docs/design" "$TMP/docs/design_review/reviews"
cp "$root/scripts/adr.py" "$root/scripts/check_register.py" "$TMP/scripts/"
cp "$root/docs/adr/template.md" "$TMP/docs/adr/template.md"

cat > "$TMP/docs/design/DESIGN.md" <<'EOF'
# Fixture design

## 1. Mission

### 1.1 Binding decisions

#### B1 Repository boundary

## 6. Canonical evidence model

### 6.3 Schema ownership
EOF

write_record() {
  local file="$1" num="$2" status="$3" extra="${4:-}"
  cat > "$TMP/docs/adr/$file" <<EOF
---
id: ADR-$num
title: A fixture decision
status: $status
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-02]
design: [§6.3]
review: not-required: fixture
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A fixture trigger fires.
verification: \`just adr-lint\`

---

# ADR-$num: A fixture decision

## Context

$extra The fixture context.

## Decision

The fixture decision.

## Status history

- 2026-09-13 — $status.
EOF
}

write_record "0001-fixture.md" "0001" "accepted"

cat > "$TMP/docs/adr/register.md" <<'EOF'
# Deferred-decision register

| R-NN | item | ADR | trigger | check | owner | last-checked | next-check | status |
|---|---|---|---|---|---|---|---|---|
| R-01 | A fixture deferral | ADR-0001 | A fixture trigger fires | $ `echo checked` | paul-heyse | 2026-09-13 | 2099-01-01 | open |
EOF

( cd "$TMP" \
  && git init -q -b main \
  && git config user.email t@example.com \
  && git config user.name t \
  && git add -A >/dev/null \
  && git commit -qm "fixture" ) || { echo "could not build the git fixture" >&2; exit 1; }

adr index >/dev/null
( cd "$TMP" && git add -A >/dev/null && git commit -qm "index" ) || true

echo "adr.py lint"
expect_clean "a well-formed corpus passes" adr lint
expect_clean "a well-formed register passes" reg --lint

# --------------------------------------------------------------------------- field checks
snapshot="$(cat "$TMP/docs/adr/0001-fixture.md")"
revert() { printf '%s' "$snapshot" > "$TMP/docs/adr/0001-fixture.md"; }

sed -i 's|^design: \[§6.3\]$|design: [§99.9]|' "$TMP/docs/adr/0001-fixture.md"
expect_error "a § citation resolving to no DESIGN.md heading is rejected" \
  "resolves to no heading in DESIGN.md" adr lint
revert

sed -i 's|^design: \[§6.3\]$|design: [6.3]|' "$TMP/docs/adr/0001-fixture.md"
expect_error "a citation without the § is rejected" "must look like" adr lint
revert

sed -i '/^verification: /d' "$TMP/docs/adr/0001-fixture.md"
expect_error "a missing verification: is rejected" "missing front-matter keys verification" adr lint
revert

sed -i 's|^verification: .*$|verification:|' "$TMP/docs/adr/0001-fixture.md"
expect_error "an empty verification: is rejected" "must state something observable" adr lint
revert

sed -i 's|^revisit: .*$|revisit:|' "$TMP/docs/adr/0001-fixture.md"
expect_error "an empty revisit: is rejected" "revisit must state something observable" adr lint
revert

sed -i 's|^evidence: Proposed$|evidence: passed|' "$TMP/docs/adr/0001-fixture.md"
expect_error "a gate state used as an evidence label is rejected" \
  "not a charter §D label" adr lint
revert

sed -i 's|^level: decision$|level: whatever|' "$TMP/docs/adr/0001-fixture.md"
expect_error "an unknown level is rejected" "level 'whatever' not in" adr lint
revert

sed -i 's|^principles: \[DM-02\]$|principles: [DM2]|' "$TMP/docs/adr/0001-fixture.md"
expect_error "a malformed principle ID is rejected" "must match DM-NN" adr lint
revert

sed -i 's|^review: not-required: fixture$|review: docs/design_review/reviews/nope.md|' \
  "$TMP/docs/adr/0001-fixture.md"
expect_error "a review: path that does not exist is rejected" "does not exist" adr lint
revert

sed -i 's|^id: ADR-0001$|id: ADR-0042|' "$TMP/docs/adr/0001-fixture.md"
expect_error "an id that disagrees with the filename is rejected" \
  "does not match filename number" adr lint
revert

# --------------------------------------------------------------------------- numbering
write_record "0003-gap.md" "0003" "proposed"
expect_error "a gap in the numbering is rejected" "not contiguous from 0001" adr lint
rm "$TMP/docs/adr/0003-gap.md"

# --------------------------------------------------------------------------- immutability
# The property that matters most, and the one the real repository cannot currently exercise:
# every record carrying front matter was added on the working branch, so there is nothing on
# main to diff against. Here there is.
sed -i 's|^## Context$|## Context\n\nAn edit that should have been a supersession.|' \
  "$TMP/docs/adr/0001-fixture.md"
expect_error "editing an accepted record's body is rejected" \
  "section 'Context' changed on an accepted record" adr lint
revert

sed -i 's|^design: \[§6.3\]$|design: [§1.1]|' "$TMP/docs/adr/0001-fixture.md"
expect_error "changing an accepted record's front matter is rejected" \
  "design changed on an accepted record" adr lint
revert

# Status and the status history are the two things that may move.
sed -i 's|^status: accepted$|status: deprecated|' "$TMP/docs/adr/0001-fixture.md"
printf -- '- 2026-09-14 — deprecated.\n' >> "$TMP/docs/adr/0001-fixture.md"
adr index >/dev/null
expect_clean "a status change plus a status-history line is allowed" adr lint
revert
adr index >/dev/null

# A proposed record is still open for editing.
write_record "0002-proposed.md" "0002" "proposed"
adr index >/dev/null
( cd "$TMP" && git add -A >/dev/null && git commit -qm "proposed record" ) || true
sed -i 's|^## Context$|## Context\n\nStill being drafted.|' "$TMP/docs/adr/0002-proposed.md"
expect_clean "a proposed record may still be edited" adr lint
rm "$TMP/docs/adr/0002-proposed.md"
adr index >/dev/null

# --------------------------------------------------------------------------- index
printf '\n| stale row |\n' >> "$TMP/docs/adr/README.md"
expect_error "a stale index is rejected by lint" "README.md is stale" adr lint
expect_error "a stale index is rejected by index --check" "is stale" adr index --check
adr index >/dev/null
expect_clean "regenerating the index makes it clean again" adr index --check

# --------------------------------------------------------------------------- register
regsnap="$(cat "$TMP/docs/adr/register.md")"
regrevert() { printf '%s' "$regsnap" > "$TMP/docs/adr/register.md"; }

sed -i 's@2099-01-01 | open@2020-01-01 | open@' "$TMP/docs/adr/register.md"
expect_error "an overdue open row is rejected" "is in the past" reg --lint
regrevert

sed -i 's@| ADR-0001 |@| ADR-0099 |@' "$TMP/docs/adr/register.md"
expect_error "a row citing an unknown ADR is rejected" "references unknown ADR-0099" reg --lint
regrevert

sed -i "s@[\$] \`echo checked\`@@" "$TMP/docs/adr/register.md"
expect_error "a row with no check is rejected" "check is empty" reg --lint
regrevert

sed -i 's@| paul-heyse |@|  |@' "$TMP/docs/adr/register.md"
expect_error "a row with no owner is rejected" "owner is empty" reg --lint
regrevert

expect_clean "--due runs the shell checks" reg --due
if reg --due 2>&1 | grep -q "No register rows are due"; then
  ok "a row due in 2099 is not reported as due"
else
  no "a row due in 2099 is not reported as due"
fi

# A check cell containing an escaped pipe is one cell, not two.
cat > "$TMP/docs/adr/register.md" <<'EOF'
# Deferred-decision register

| R-NN | item | ADR | trigger | check | owner | last-checked | next-check | status |
|---|---|---|---|---|---|---|---|---|
| R-01 | A fixture deferral | — | A fixture trigger fires | $ `ls \| wc -l` | paul-heyse | 2026-09-13 | 2099-01-01 | open |
EOF
expect_clean "an escaped pipe inside a check cell stays one cell" reg --lint
regrevert

# --------------------------------------------------------------------------- new / supersede
expect_clean "adr new allocates the next number" adr new another-fixture --title "A second decision"
if [ -f "$TMP/docs/adr/0002-another-fixture.md" ]; then
  ok "adr new wrote 0002-another-fixture.md"
else
  no "adr new wrote 0002-another-fixture.md"
fi
expect_error "adr new rejects a non-kebab-case slug" "must be lowercase kebab-case" \
  adr new "Not A Slug" --title "x"

sed -i 's|^status: proposed$|status: accepted|' "$TMP/docs/adr/0002-another-fixture.md"
sed -i 's|^design: \[§1.1\]$|design: [§6.3]|' "$TMP/docs/adr/0002-another-fixture.md"
sed -i 's|^review: not-required: REASON$|review: not-required: fixture|' \
  "$TMP/docs/adr/0002-another-fixture.md"
expect_clean "adr supersede links both records" adr supersede 0001 0002
if grep -q '^superseded-by: ADR-0002$' "$TMP/docs/adr/0001-fixture.md" \
  && grep -q '^status: superseded$' "$TMP/docs/adr/0001-fixture.md" \
  && grep -q '^supersedes: \[ADR-0001\]$' "$TMP/docs/adr/0002-another-fixture.md"; then
  ok "supersession is symmetric and the old status moved"
else
  no "supersession is symmetric and the old status moved"
fi

sed -i 's|^supersedes: \[ADR-0001\]$|supersedes: []|' "$TMP/docs/adr/0002-another-fixture.md"
adr index >/dev/null
expect_error "a one-sided supersession is rejected" "does not name it in supersedes" adr lint

echo
printf 'adr-lint tests: %d passed, %d failed\n' "$pass" "$fail"
[ "$fail" -eq 0 ]
