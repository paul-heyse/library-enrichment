---
id: ADR-0023
title: Publish the relational catalog through one coherent generation
status: superseded
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-09, DM-14, DM-30, DM-31, DM-35]
design: [§B7, §6.3, §8.2, §8.4]
review: docs/design_review/reviews/design_review_arrow-target-contract_2026-09-14.md
evidence: Proposed
supersedes: []
superseded-by: ADR-0042
revisit: A real transaction or storage workload cannot meet its bounded consistency/cost contract with immutable manifests.
verification: snapshot_admission; catalog_generation; publication_crashes; concurrent_catalog_publishers; export_closure
---

# ADR-0023: Publish the relational catalog through one coherent generation

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

Rust owns an immutable relational catalog containing release/environment/context/snapshot
records. One exact-file manifest and durable root selection determine visibility. Lookup views
use the same pinned generation. No set of separately updated JSON indexes is authoritative.
Reuse unchanged immutable artifacts between catalog generations; never rewrite library facts
merely because another context is added.

The process ownership lock excludes another daemon. An in-process commit coordinator serializes
publishers: stage independent work first, then read the latest generation under the commit lock,
merge validated additions and atomically publish the next root. Concurrent successful jobs must
both remain discoverable. Root replacement and directory synchronization are the commit boundary.

An enrichment publication declares the base snapshot it extends. Under the commit lock compare
that base with the context's selected snapshot. If another enrichment won, retain the completed
candidate as unselected and rebase its disjoint typed observations onto the new base, revalidate,
and retry with the new base precondition. Conflicting observations remain alternatives; they
are not overwritten. If bounded retry is exhausted report a publication conflict, not success
selecting stale evidence. Tests cover same-context disjoint additions as well as distinct contexts.

Validate local invariants with Arrow kernels and relational uniqueness/reference/closure
invariants with bounded DataFusion staging plans. Unvalidated staging providers carry no
optimizer constraints. Only admitted immutable providers declare actual PK/Unique constraints.
Source/target reference kinds determine conditional foreign keys; aliases do not make
symbols.definition_id unique.

Stage, validate, synchronize files and manifest, publish the immutable snapshot, then commit the
catalog generation. Faults leave either an old or new coherent root. Pinned read/export leases
protect exact files against manual cleanup. Startup reconciliation distinguishes staging,
unreferenced complete artifacts, committed facts and uncertain execution. Automatic temporary
cleanup never deletes retained evidence or ownership needed to finish recovery.

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
| Pinned interface supports the selected boundary | [primary source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/common/src/functional_dependencies.rs) and local pinned Cargo registry | 2026-09-14 | `does not check whether the argument is valid` |
| No historical preservation obligation | User instruction in this session | 2026-09-14 | "We are still in the design phase" |

## Verification

snapshot_admission; catalog_generation; publication_crashes; concurrent_catalog_publishers; export_closure. These target tests are mandatory work, not a claim they have run.
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
- 2026-09-15 — superseded by ADR-0042.
