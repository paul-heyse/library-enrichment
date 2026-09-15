---
id: ADR-0024
title: Execute typed evidence queries through bounded DataFusion plans
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-19, DM-23, DM-26, DM-32, DM-36, DM-38, DM-39, DM-40, DM-42]
design: [§B7, §7.1, §7.3, §8.4, §14.3]
review: docs/design_review/reviews/design_review_arrow-target-contract_2026-09-14.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A measured query fails its semantic or resource contract using the current built-in operators.
verification: query_contract; query_resources; search_truth_table; comparison_relational; arrow_projection; query_workload
---

# ADR-0024: Execute typed evidence queries through bounded DataFusion plans

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

Use a daemon-owned bounded RuntimeEnv and request-scoped catalogs of validated immutable
snapshots. Exact manifest file sets and schemas are authority; directory listing, path and
mtime are not. Cache admission/providers/metadata under dependency and byte limits. Cold
admission validates full content; unchanged warm requests do not decode every table again.
Changed bytes/witnesses, schema or validator invalidate admission. Cache eviction never
regenerates library evidence.

Rust SearchSpec defines eligibility, case/token rules, scored fields, alias folding and stable
order. Store lowering uses native DataFusion Expr predicates. Specialized scoring remains a
pure Rust batch kernel through a narrow ScalarUDF adapter; native grouping/windowing/order and
keyset paging precede bounded payload hydration. Exact counts follow the same folded scope.
No silent candidate cap, false empty success, or oversized first hit is permitted.

Use relational projections, aggregates, joins and windows for overview, inspect, comparison
and provenance/export closure. Keep semantic canonicalization and specialized scoring in
ordinary Rust. Typed alternative sets, configuration confounders and epistemic classes survive
lowering. Lexical ancestry and MemberOf are distinct, not mutually substitutable.

Bound managed memory, concurrent work, batches by rows and bytes, result buffers, spill and
deadlines. Account separately for allocations outside the DataFusion pool. Cancellation releases
streams/tasks/leases and temporary files. Preserve semantic field metadata deliberately through
scan and expression boundaries. Record bounded diagnostics from the actual executed plan;
do not rerun EXPLAIN ANALYZE or expose arbitrary SQL/host paths. Physically sort before declaring
sort metadata and select pruning/bloom/row-group choices by measured workload. No nested-pruning
or answer-proportional scan guarantee is claimed.

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
| Pinned interface supports the selected boundary | [primary source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/execution/src/memory_pool/mod.rs) and local pinned Cargo registry | 2026-09-14 | `Intermediate memory used as data streams through the system is not accounted` |
| No historical preservation obligation | User instruction in this session | 2026-09-14 | "We are still in the design phase" |

## Verification

query_contract; query_resources; search_truth_table; comparison_relational; arrow_projection; query_workload. These target tests are mandatory work, not a claim they have run.
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
