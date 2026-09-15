---
id: ADR-0035
title: Select exact definitions when public symbol paths collide
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-06, DM-11, DM-16, DM-50]
design: [§6.3, §7.1]
review: docs/design_review/reviews/design_review_inspection-definition-selection_2026-09-14.md
evidence: Tested
supersedes: []
superseded-by: null
revisit: A producer requires selecting distinct bindings of one definition at an identical public path.
verification: test_same_path_definitions_can_be_selected_without_merging_kinds; schema-conformance
---

# ADR-0035: Select exact definitions when public symbol paths collide

## Context

Actual Plan 12 A04 research of Serde 1.0.228 found two definitions at `serde::Serialize`.
Inspection returned ambiguity with a single deduplicated candidate path, `serde::Serialize`.
Following that suggestion repeated the same ambiguity. The target model correctly retained both
kinds, but the public inspection request could not select their distinct identities.

## Scope

Add optional `definition_id` to the closed Rust inspection request and forward it through the
thin MCP adapter. Replace inspection's string candidates with typed records carrying `path`,
`definition_id`, `kind` and nullable `qualifier`.

## Drivers

Make an ambiguous result actionable without guessing, merging kinds, enumerating the whole
library or creating an alternate retrieval path.

## Options

- Keep repeating qualified paths: cannot resolve identical public-path collisions.
- Invent an encoded path syntax: couples presentation strings to identity parsing.
- Select the existing definition ID explicitly within the requested path and snapshot: selected.

## Decision

A supplied definition ID is a native DataFusion predicate combined with the requested public
path, before bounded observation hydration. It never widens the snapshot or path scope. A bare
name may use the existing native suffix lookup; a qualified Rust or Python path must match
exactly. Unknown or mismatched selections return an explicit unavailable result.

Ambiguity returns distinct, deterministic typed candidates. A client resubmits a candidate's
path and definition ID. Once selected, all independent observations of that definition remain
available; this option does not choose a source/stub winner. Execution jobs retain the selected
definition in their normalized request and durable identity.

### Consequences

The generated request and tool-data schemas change directly. No compatibility parser for the
old candidate strings is added. Stored evidence identities and relations do not change; the
field selects existing facts rather than producing or rewriting them.

### Compensating controls

A real Python wheel with a class in source and a function in stubs at the same public path
must return two selectable definitions. Selection must preserve kind, work after restart and
reject a definition paired with a nonmatching qualified path. Schema conformance checks the
Rust/Python contract.

## Evidence

| Claim | Primary evidence | Observed | Detail |
|---|---|---|---|
| The former ambiguity response was not actionable | `.dev-state/logs/clients/run-70yh5a6q/A04/A04-claude.json` and independent witness | 2026-09-14 | Actual Serde 1.0.228 inspection found two definitions but offered the same path again |
| Filtering belongs before presentation | [Native lookup](../../crates/enrichment-store/src/query.rs) | 2026-09-14 | `col("definition_id").eq(lit(id))` is combined with the path predicate |
| Independent observations stay separate | [Inspection](../../crates/enrichment-daemon/src/ops/inspect.rs) | 2026-09-14 | Candidate selection retains a definition's qualified observations |

No new upstream API or dependency is introduced. This is a correction to the service's own
selection contract discovered through actual final output assessment.

## Verification

`test_same_path_definitions_can_be_selected_without_merging_kinds` exercises real wheel/static
extraction, native publication, MCP ambiguity/selection and restart. `just schema-conformance`
checks generated consumers. Current execution results belong in the Plan 12 ledger.

## Boundaries preserved

§B1–§B13 remain intact. Rust owns requests, identities, selection, policy and publication.
Python forwards the generated contract. No SQL tool, domain engine switch, new persistent
index, source/stub reconciliation or implied execution is introduced.

## More information

[Plan 12 final output assessment](../plans/12-architecture-first-completion.md),
[Design §7.1](../design/DESIGN.md#71-tool-behavior-requirements).

## Status history

- 2026-09-14 — proposed after an actual client exposed an unselectable same-path definition.

- 2026-09-14 — accepted after scoped review; real selection/mismatch/restart and schema conformance passed.
