---
id: ADR-0038
title: Bind native query validity and resources to research operations
status: superseded
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-24, DM-26, DM-35, DM-47, DM-50]
design: [§7.3, §8.1, §14.3]
review: docs/design_review/reviews/design_review_research-v2-contract_2026-09-15.md
evidence: Implemented
supersedes: []
superseded-by: ADR-0041
revisit: A recurring cross-family semantic rewrite cannot be implemented by the shared preparation boundary.
verification: independent_family_requirement_fails_before_execution_with_its_own_trace; preparation_failure_retains_its_own_query_identity; operation_shares_materialization_budget_and_does_not_reacquire_its_permit
---

# ADR-0038: Bind native query validity and resources to research operations

## Context

Several individually bounded child queries can accumulate time and retained output without research-operation correlation.

## Scope

Implement the Plan 13 target in the cited design sections. The contract was accepted under
scoped review; implementation, independent final qualification and production activation are now
complete. See the [final evidence](../reports/plan13-final-qualification-2026-09-15.md) and
[accepted closing review](../design_review/reviews/design_review_plan13-implementation_2026-09-15.md).

## Drivers

Truthful scope, bounded useful results, one semantic owner, reproducible evidence and explicit effects.

## Options

- Retain current behavior: rejected by the recorded functional failures.
- Add compatibility branches and local patches: rejected by the authorized hard pivot.
- Replace the shared contract and its consumers: selected.

## Decision

Every research operation binds snapshot inputs, effective policy, deadline, cancellation, retained output and child-query diagnostics. Reuse the shared RuntimeEnv and isolated registration namespaces. Fixed query families declare relation input/output contracts and bounded invariant witnesses. Retain default native analysis/optimization and conservative provider pushdown. Specialized pure UDFs retain canonical identity semantics. Operation admission cannot recursively acquire the same permit pool.

### Consequences

All generated, runtime and human-facing consumers change together. Old contracts are not supported
by the new runtime. Independent source evidence and fixed policy boundaries remain authoritative.

### Compensating controls

Targeted negative/boundary tests, generated schema conformance, native ownership checks and
Plan 13's functional and deletion ledgers. No mocked acceptance or silent fallback.

## Evidence

This is an internal contract decision grounded in current source and the functional review.
Exact upstream interface receipts are recorded separately before their consumers are implemented.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Current implementation boundary | [Source](../../crates/enrichment-store/src/runtime.rs) | 2026-09-15 | `pub struct QueryRuntime` |

## Verification

Required oracles: operation_budget_covers_child_queries; relation_contract_preserves_nullable_provenance. These names describe obligations, not executed results.
Receipts and actual assertion locations are recorded in the Plan 13 execution ledger.

## Boundaries preserved

§B1–§B13 retain their ownership and trust boundaries. Rust owns state, evidence, jobs, policy and
publication; Python is mechanical transport and a separate extractor. DataFusion owns native
relational work, hosted rustdoc is preferred, ty/rust-analyzer semantics and the six epistemic
classes remain distinct. No automatic edits or subprocess execution inside studied repositories.

## More information

[Plan 13](../plans/13-datafusion-research-operations-hard-pivot.md),
[functional review](../design_review/reviews/design_review_mcp-datafusion-fastmcp_2026-09-14.md).

## Current implementation evidence — 2026-09-15

The [scoped contract review](../design_review/reviews/design_review_research-v2-contract_2026-09-15.md)
accepts this decision without certifying the complete implementation. Actual test receipts and
remaining work are in the [execution ledger](../plans/13-research-operations-execution-ledger.md).
[Exact upstream verification](../architecture/plan13-upstream-verification.md) and the
[current source supplement](../architecture/plan13-current-upstream-verification.md) separate
Context7 discovery from pinned DataFusion 55.1.0 and installed FastMCP 4.0.3 evidence.

DataFusion defaults are preserved across separate analyzer and optimizer calls. Family checks
use native Arrow fields; physical streams match physical schemas exactly. Early query traces
include analysis failure. Admission corruption and result-contract defects have different
causes. Full family inventory and workload measurement remain W2/W11 work, not established claims.

## Status history

- 2026-09-15 — superseded by ADR-0041.

- 2026-09-15 — proposed for the authorized hard pivot; implementation and scoped review pending.

- 2026-09-15: accepted following scoped contract review; final Plan 13 qualification remains open.


## Measured implementation refinement — 2026-09-15

Search, overview and comparison now retain only an operation-local native relation at a measured
reuse boundary. DataFusion DiskManager/SpillFile, native StreamingTable and Arrow IPC implement
temporary storage and scans; no persistent cache or custom relational operator is introduced.
The relation retains the operation's exact input leases and configured-policy binding through
its final reader. Native memory reservations and disk quota remain shared across callers.
[Fixture measurements](../reports/plan13-native-operation-measurements-2026-09-15.md) support the
choice without certifying large-library throughput or final client acceptance.

### Final implementation qualification — 2026-09-15

The independent final replay passed 432 native, 223 Python, 12 execution, 2 live and 15 installed
client tests. Source: `224dea96aca19e8482b6715f524ebe0b5948ba039e0c8fac00200aa933d72bf5`.
All active acceptance gates passed. The sole-generation production activation and deployed
raw-MCP smoke passed at 13:49–13:50 UTC; prior state/configuration remain inactive and unchanged.
The [final qualification report](../reports/plan13-final-qualification-2026-09-15.md) records exact
receipts, current component identities and limits. This evidence closes the earlier pending
implementation qualification; historical status entries retain their original meaning.
