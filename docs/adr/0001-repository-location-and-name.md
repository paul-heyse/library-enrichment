---
id: ADR-0001
title: Repository location and name
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-11]
design: [§12.1]
review: not-required: repository identity; it governs no design surface
evidence: Implemented
supersedes: []
superseded-by: null
revisit: The repository is relocated, or a second checkout is needed on this workstation
verification: `rg -n '/home/paul/library-enrichment' scripts justfile` returns nothing; every root is derived at run time

---

# ADR-0001: Repository location and name

## Context

The blueprint's §2.3 filesystem layout and its §12.2 registration templates both write the
repository as `~/src/library-enrichment`, with `REPO="$HOME/src/library-enrichment"`. The
specification bundle was delivered at `/home/paul/library_enrichment` — a different parent
directory and an underscore rather than a hyphen.

The operator's other projects (`~/smartref`, `~/CodeFabric`) sit at the top level of `$HOME`,
not under `~/src`.

## Decision

The repository lives at `~/library-enrichment`: hyphenated to match the package, crate, service,
and MCP server names, but at the top level of `$HOME` alongside the operator's other projects
rather than under `~/src`.

No script or configuration file hardcodes this path. Everything derives its root from
`git rev-parse --show-toplevel`, `justfile_directory()`, or `$CLAUDE_PROJECT_DIR`, so relocating
the repository again requires no edits.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| The blueprint proposes `~/src/library-enrichment` | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §2.3 | 2026-09-13 | "`~/src/library-enrichment/ # implementation only`" |
| Registration templates hardcode that path | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §12.2 | 2026-09-13 | "`REPO=\"$HOME/src/library-enrichment\"`" |

## Verification

`scripts/provenance-check.sh`, `scripts/test-hooks.sh`, and every `just` recipe resolve their
own root. Acceptance gate C13 ("tool runs from an unrelated current directory") asserts the
installed service finds its configuration and state regardless of the caller's cwd.

## Consequences

The §12.2 registration commands must substitute the actual path. `docs/operations/` documents
the registration step with a derived `REPO`, so the documented command is correct here and
portable elsewhere.

## Boundaries preserved

All of them. This is a filesystem-location choice; it changes nothing about repository/state
separation, core ownership, or the tool contract.

## Status history

- 2026-09-13 — accepted.
