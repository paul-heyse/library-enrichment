---
id: ADR-0003
title: In-repo development state sandbox
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-28, DM-29]
design: [§B1, §2.3]
review: not-required: backfilled; recorded before the design-review process existed (ADR-0009)
evidence: Tested
supersedes: []
superseded-by: null
revisit: A second development checkout needs to share state, or `.dev-state/` appears in a commit
verification: `just state-leak-check`; the service-state deny cases in `scripts/test-hooks.sh`

---

# ADR-0003: In-repo development state sandbox

## Context

The blueprint is emphatic that service state lives outside working repositories: caches at
`~/.cache/library-enrichment`, retained evidence at `~/.local/share/library-enrichment`, and
"no service code, environments, caches, or generated indexes inside working repositories".

That rule protects *working repositories under study* and keeps production state out of source
control. It does not settle where an **agent development session** should write while building
the service itself. Pointing development sessions at the real XDG paths means every session
mutates live user state, which is hard to snapshot, hard to reset, and makes a leak from a
misresolved directory policy indistinguishable from legitimate use.

## Decision

Development sessions resolve service state to a gitignored in-repo sandbox at `.dev-state/`
(`cache/`, `data/`, `logs/`). Production continues to resolve to the XDG paths; only the
development environment is redirected, via `LIBENR_HOME` / `LIBENR_CACHE_HOME` /
`LIBENR_DATA_HOME` set in `scripts/env.sh`.

`scripts/env.sh` is the single definition, sourced by both `.envrc` (interactive shells, through
direnv) and `scripts/hooks/session_env.sh` (agent sessions, through `CLAUDE_ENV_FILE`), so a
human and an agent cannot end up running against different state.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| State belongs outside working repositories | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §1.1 | 2026-09-13 | "no service code, environments, caches, or generated indexes inside working repositories" |
| Production paths are XDG-shaped | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §2.3 | 2026-09-13 | "`~/.cache/library-enrichment/ # regenerable data`" |
| The resolver must be OS-aware with overrides | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §2.3 | 2026-09-13 | "must use an OS-aware directory resolver with overrides rather than assume these Linux paths" |

The third quote is what makes this legitimate rather than a deviation: the blueprint already
requires configurable overrides. This exercises that mechanism.

## Verification

- `just state-leak-check` digests the real XDG paths before and after a run and fails on any
  change — so if a code path resolves the production policy instead of the configured root, it
  surfaces immediately rather than accumulating silently.
- `.dev-state/` is gitignored, and `scripts/hooks/stop_guard.sh` blocks on any service-state
  directory appearing elsewhere in the repository.
- Acceptance gate C20 (canary working repository unchanged) is unaffected: this concerns where
  *this* repository's development state lives, never where a repository under study is touched.

## Consequences

`.dev-state/` must never be committed; `just state-reset` makes discarding it a one-line
operation. A test that needs the true production resolver must set `LIBENR_TEST_ROOT` and assert
against a temporary root, not against `$HOME`.

## Boundaries preserved

No working repository under study is ever written to or used as a working directory. Production
still resolves to XDG. Nothing about core ownership, the evidence model, or execution policy
changes.

## Status history

- 2026-09-13 — accepted.
