---
id: ADR-0037
title: Separate research outcome from bounded result delivery
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-22, DM-30, DM-41, DM-42, DM-51]
design: [§6.3, §7.2, §7.3, §7.4, §8.2, §8.3]
review: docs/design_review/reviews/design_review_research-v2-contract_2026-09-15.md
evidence: Implemented
supersedes: [ADR-0030]
superseded-by: null
revisit: A supported result requires a new representation beyond bounded pages and artifacts.
verification: delivery_preserves_outcome_and_scope_and_indexes_independent_sections; overflow_delivery_recovers_from_read_only_bytes_without_regeneration; individual_large_values_are_complete_readable_and_content_addressed; schema-conformance
---

# ADR-0037: Separate research outcome from bounded result delivery

## Context

Error-shaped successful overflow and nested job envelopes obscure results. The user authorized one hard replacement without transitional readers.

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

Pydantic/FastMCP owns authored MCP presentation models, defaults and rendering composed from
generated Rust domain DTOs. It validates agent input before binding, lowers it into an explicit
native request, and validates native output before rendering every outcome. Rust owns evidence
meaning, coverage, enforced capacity, cursor identity, typed recovery prerequisites and durable
result references. The two interfaces need not have identical presentation schemas; lowering and
projection must preserve the domain contract. No native framework is added for MCP schema
composition, text-block layout or optional resource links. Normal typed Rust handles nonrelational
domain variants; they are not forced into Arrow arrays or UDFs.

The sole active wire contract is 2.0. Research status is independent of inline, page or artifact delivery. Preserve original outcome, coverage and a complete retrieval action. Root pagination is replaced by typed payload pages. Error records carry cause, stage and recovery actions. Terminal jobs expose their result outcome and direct descriptor. Every required result/index artifact is admitted before successful commit and included in export. Active conformance targets the new frozen bundle; original specification bytes remain historical. Start a fresh active state generation; do not decode old envelopes or journals.

### Consequences

All generated, runtime and human-facing consumers change together. Old contracts are not supported
by the new runtime. Independent source evidence and fixed policy boundaries remain authoritative.

### Compensating controls

Targeted negative/boundary tests, generated schema conformance, native ownership checks and
Plan 13's functional and deletion ledgers. No mocked acceptance or silent fallback.

## Evidence

This is an internal contract decision grounded in current source and the functional review.
Exact upstream interface receipts are recorded separately before their consumers are implemented.

[Exact-release verification and executed modeling probes](../architecture/plan13-upstream-verification.md)
establish the selected FastMCP 4.0.3 / Pydantic 2.13.5 composition boundary. This choice responds
to the user's 2026-09-15 request to consider Pydantic; it is an evaluated design decision, not
an assumption that all response logic belongs in Python.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Current implementation boundary | [Source](../../crates/enrichment-daemon/src/delivery.rs) | 2026-09-15 | `The complete answer is stored because it exceeds the inline budget.` |

## Verification

Required oracles: delivery_preserves_outcome_and_scope; schema-conformance; committed_overflow_delivery_recovers_without_writes. These names describe obligations, not executed results.
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

Alternative values are tagged inline/artifact; large value artifacts carry readable handles,
digests and sizes alongside the alternative's original FactSource. Indexed terminal result
recovery verifies retained bytes and selects header sections without parsing full data or
writing scratch. ADR-0030's precommit result obligation remains mandatory; its earlier delivery
representation is superseded. Current-source export/fault qualification passed in the independent
native and Python replays recorded in the final qualification section below.

## Status history

### Portable client error projection — 2026-09-15

The real Claude recovery journey retained MCP error text but omitted structured error content
from its client event trace. A generic job-status summary therefore stranded recovery despite a
correct native terminal record. `presentation.error_preview` now projects the native error code,
cause, stage, retryability and next action; terminal artifacts retain their complete read action.
The adapter measures the serialized MCP result against the fixed framing allowance and omits
long explanations when necessary. Structured output remains the validated Rust envelope.
This adds no new evidence or job authority and changes no frozen domain bytes.

`tests/contract/test_research_stdio.py` verifies a real failed acquisition and readable result;
`tests/unit/test_error_preview.py` covers a large escaped diagnostic and terminal artifact-write
failure without a read action. The corrected boundary receipt recorded 12 passed cases in
`.dev-state/plan13-final-adapter-boundary-corrected.log`. Final actual-client recovery qualification
is recorded separately in the execution ledger; the earlier failed trace remains preserved.

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
