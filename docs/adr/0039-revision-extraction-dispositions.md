---
id: ADR-0039
title: Record safe revision omissions and required source coverage
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-22, DM-30, DM-43, DM-45]
design: [§10]
review: docs/design_review/reviews/design_review_research-v2-contract_2026-09-15.md
evidence: Implemented
supersedes: []
superseded-by: null
revisit: A selected package requires a linked input to establish its source closure.
verification: archive::tests; revision_required_link_is_incomplete
---

# ADR-0039: Record safe revision omissions and required source coverage

## Context

A harmless root CLAUDE.md link blocks the entire DataFusion revision archive. Skipping arbitrary extraction failures would hide required inputs.

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

Validate every member path/type/collision and expansion budget before extraction or omission. Never create or follow links. Retain a typed archive disposition record for permitted omissions and preserve raw archive identity. Reject unsafe paths, devices, malformed metadata and conflicting entries. Assess selected package and workspace inputs; omitted required or unproven source closure prevents a complete-source/build claim. Native coverage joins reflect scoped omissions; producer policy identity includes this rule.

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
| Current implementation boundary | [Source](../../crates/enrichment-core/src/archive/mod.rs) | 2026-09-15 | `ArchiveError::UnsupportedEntry` |

## Verification

Required oracles: revision_omits_unrelated_links_without_following; revision_required_link_is_incomplete. These names describe obligations, not executed results.
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

## Status history

- 2026-09-15 — proposed for the authorized hard pivot; implementation and scoped review pending.

- 2026-09-15: accepted following scoped contract review; final Plan 13 qualification remains open.

### Final implementation qualification — 2026-09-15

The independent final replay passed 432 native, 223 Python, 12 execution, 2 live and 15 installed
client tests. Source: `224dea96aca19e8482b6715f524ebe0b5948ba039e0c8fac00200aa933d72bf5`.
All active acceptance gates passed. The sole-generation production activation and deployed
raw-MCP smoke passed at 13:49–13:50 UTC; prior state/configuration remain inactive and unchanged.
The [final qualification report](../reports/plan13-final-qualification-2026-09-15.md) records exact
receipts, current component identities and limits. This evidence closes the earlier pending
implementation qualification; historical status entries retain their original meaning.
