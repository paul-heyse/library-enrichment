---
id: ADR-0022
title: Replace development evidence with one typed Arrow schema
status: superseded
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-11, DM-12, DM-15, DM-24, DM-46]
design: [§3.1, §3.3, §6.1, §6.3]
review: docs/design_review/reviews/design_review_arrow-target-contract_2026-09-14.md
evidence: Proposed
supersedes: [ADR-0020]
superseded-by: ADR-0043
revisit: A new evidence distinction cannot be represented or a deployed consumer requires storage compatibility.
verification: typed_model; arrow_projection; snapshot_admission; evidence_retention; development_reset
---

# ADR-0022: Replace development evidence with one typed Arrow schema

## Context

The user approved the entirety of Plan 10 after explicitly requesting a hard design pivot.
The existing storage/query slice duplicates domain rules and materializes full evidence before
answering narrow questions. There is no deployed historical-store compatibility requirement.

## Scope

The corresponding contracts in [Plan 10](../plans/10-arrow-datafusion-architecture.md), including
its specified negative oracles. This record accepts a target contract, not completed behavior.

## Drivers

Typed meaning, reproducibility, one authority, bounded execution and coherent publication.

## Options

- Preserve the old tables/readers: rejected by the user's explicit hard-pivot direction.
- Add a new storage platform: no current need; existing Arrow/DataFusion capabilities suffice.
- Replace the model and execution within Rust ownership: selected.

## Decision

Use one new snapshot storage version 4.0. Remove formats 1–3 readers, compatibility defaults
and historical snapshot fixtures. Existing development library evidence is disposable at a
scoped operator reset; source work, credentials and unrelated state are outside that reset.
Reset refuses active or uncertain owned execution and preserves recovery journals until
absence is confirmed.

Normalize definitions, public bindings, independent API observations, relationships, fragments,
producer runs, inputs and coverage into typed Arrow relations. Lexical ancestry derives from
typed public path components; it is distinct from producer-observed semantic membership.
Tagged references distinguish symbol, definition, document/feature/example and external or
unknown targets. Struct validity and list order preserve absence/empty/alternative meanings.
Closed enum vocabularies have one token spelling and explicit validation. An open JSON field is
permitted only for versioned producer extensions, never a replacement for queried structure.

Semantic producer binding IDs exclude attempt clocks/logs; observation identity binds qualified
content and producer semantics, then snapshot identity binds the complete context/input/coverage
closure. Eventual snapshot IDs and physical file hashes do not enter their own preimages.
Actual attempt/run attribution remains explicit and repeated equivalent acquisition appends
associations without mutating the selected snapshot. Citation locators are acquisition-specific.

In the target lifecycle, validated evidence for an unchanged exact context does not expire
with age. Mutable version lookup freshness, query/provider cache eviction and temporary cleanup
do not invalidate facts. New versions acquire separate facts; manual evidence cleanup is an
explicit operation. Runtime observations remain scoped to recorded environment, inputs and
policy. These ADR-0020 semantics remain; historical-read obligations are replaced.

### Consequences

Development data is reset rather than migrated. New explicit schema, validation and query
contracts replace implicit glue. Full target acceptance requires exercised producer, operation,
recovery and client behavior, not source inventories.

### Compensating controls

The verification identifiers below cover malformed input, interruption, resource exhaustion
and truthful reporting. Full matching-source acceptance remains mandatory.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Pinned interface supports the selected boundary | [primary source](https://raw.githubusercontent.com/apache/arrow-rs/59.3.0/arrow-schema/src/datatype.rs) and local pinned Cargo registry | 2026-09-14 | `Struct(Fields)` |
| No historical preservation obligation | User instruction in this session | 2026-09-14 | "We are still in the design phase" |

## Verification

typed_model; arrow_projection; snapshot_admission; evidence_retention; development_reset. These target tests are mandatory work, not a claim they have run.
Plan 10 §6 describes their independent expected behavior and failure cases. Source/log/tool
receipts bind future execution evidence. No acceptance ID is removed to ease the pivot.

## Boundaries preserved

§B1–§B13 retain Rust ownership, repository isolation, the separate Python worker/adapter,
ty and rust-analyzer producers, hosted-first Rust documentation, local stdio/daemon transport
and a separate Context7 connection. No graph/vector database, arbitrary-shell/SQL MCP endpoint
or automatic project editing is added.

## More information

[Plan 10](../plans/10-arrow-datafusion-architecture.md);
[independent assessment](../design_review/reviews/design_review_arrow-datafusion-assessment_2026-09-14.md).

## Status history

- 2026-09-14 — proposed for target-contract review; implementation authorized.
- 2026-09-14 — accepted at Proposed contract scope by the target-contract review; implementation and executable acceptance remain open.
- 2026-09-15 — superseded by ADR-0043.
