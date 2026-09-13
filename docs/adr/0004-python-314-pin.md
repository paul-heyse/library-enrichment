---
id: ADR-0004
title: Python 3.14 pin, pending upstream verification
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-31, DM-48]
design: [§12.1]
review: not-required: backfilled; recorded before the design-review process existed (ADR-0009)
evidence: Tested
supersedes: []
superseded-by: null
revisit: A required dependency drops 3.14 support, or 3.15 becomes the pinned interpreter
verification: `just sync` then `just test-python`; `uv.lock` pins fastmcp 4.0.3, griffe 2.3.0 and ty 0.0.80 on 3.14.7

---

# ADR-0004: Python 3.14 pin, pending upstream verification

## Context

The operator's other Python projects pin `requires-python = ">=3.14,<3.15"`, and matching that
keeps one interpreter story across the workstation. This machine's system interpreter is 3.12.3;
`uv` can install 3.14.

The risk is that this project's three Python dependencies are all young. FastMCP 4, Griffe, and
`ty` must each genuinely support 3.14, and **none of that has been verified yet** — `ty` is not
currently installed here at all.

This ADR is `proposed`, not `accepted`, precisely because the evidence table below is empty.

## Decision

Pin Python 3.14 (`.python-version`, `requires-python = ">=3.14,<3.15"`, ruff
`target-version = "py314"`). The condition this ADR was opened under has been discharged: all
three dependencies resolve and import on 3.14.7, at `fastmcp==4.0.3`, `griffe==2.3.0` and
`ty==0.0.80`, each pinned exactly with `uv.lock` committed.

The fallback, had any of the three lagged, was 3.13. It was not needed and is recorded only so
the reasoning is legible later.

Note that the interpreter analysed by `ty` in a consumer capsule is a separate choice from the
interpreter this service runs on, and capsules must support a range regardless of what we pin
here.

## Evidence

Established by execution on 2026-09-13: each package was resolved by `uv` under
`requires-python = ">=3.14,<3.15"` and then imported on CPython 3.14.7.

| Claim | Source | Retrieved | Evidence |
|---|---|---|---|
| FastMCP 4 resolves and imports on 3.14 | executed | 2026-09-13 | `fastmcp==4.0.3`; `from fastmcp import FastMCP` → `fastmcp.server.server.FastMCP` |
| Griffe resolves and imports on 3.14 | executed | 2026-09-13 | `griffe==2.3.0`; `inspect.signature(griffe.load)` readable |
| ty resolves and runs on 3.14 | executed | 2026-09-13 | `ty==0.0.80`; `.venv/bin/ty --version` → `ty 0.0.80` |
| The interpreter is genuinely 3.14 | executed | 2026-09-13 | `sys.version` → `3.14.7` |

This is evidence of resolution and import, which is what the pin decision needs. It is **not**
evidence that every runtime path works on 3.14; that is established by the test suite as it is
written. `ty 0.0.80` is pre-1.0 and should be expected to churn.

## Verification

`just doctor` fails when `ty` is absent. `just sync` fails if the lockfile cannot be resolved
for the pinned interpreter. The Phase 0 gate runs both.

## Consequences

If 3.14 holds, the workstation has one interpreter story. If it does not, moving to 3.13 means
editing `.python-version`, `requires-python`, ruff's `target-version`, and this ADR — all
cheap, provided it happens before a lockfile and CI depend on the pin.

## Boundaries preserved

All of them.

## Status history

- 2026-09-13 — accepted.
