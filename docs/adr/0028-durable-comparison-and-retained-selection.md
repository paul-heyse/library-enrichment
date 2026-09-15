---
id: ADR-0028
title: Compose cold comparisons and reuse exactly qualified retained execution
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-27, DM-30, DM-33, DM-35]
design: [§7.1, §9.2]
review: docs/design_review/reviews/design_review_native-architecture-completion_2026-09-14.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A supported comparison needs more than its two fixed acquisition prerequisites.
verification: cold_version_comparison_waits_for_exact_acquisitions; exact_derived_inspection_reuses_retained_observations; comparison_cancellation_preserves_another_subscriber
---

# ADR-0028: Compose cold comparisons and reuse exactly qualified retained execution

## Context

Plan 11 identified that cold version comparison still assumes synchronous resolution. The
resolver now schedules durable work and can return Pending. Explicit inspection from a parent
context also needs an exact rule for reuse of retained resolved-child observations.

## Scope

Amend design §7.1 and §9.2 within the existing nine tools, typed evidence relations, catalog and
concrete jobs. No generic workflow or second query/resolution path is introduced.

## Drivers

Bounded request latency, cancellation ownership, stable retained facts and truthful result scope.

## Options

- Block the original comparison request indefinitely: rejected; breaks bounded transport.
- Return a resolution job as if it were the comparison: rejected; its terminal payload is not
  the requested comparison and loses the second acquisition.
- One concrete comparison job with two owned acquisition interests: selected.
- Search arbitrary descendant contexts by symbol name: rejected; environment/query ambiguity.

## Decision

Version-form comparison schedules one closed Compare job. It resolves exactly two requested
versions through the existing retained resolver/acquisition job path, without holding a producer
permit while waiting. It owns a separate interest in each acquisition. Cancelling the comparison
removes only those interests; another subscriber keeps its work. If the comparison is the last
subscriber it waits for acquisition cleanup. Then the comparison pins one catalog generation and
executes the same native read as an explicitly pinned comparison. Pending returns the comparison
job identity; terminal delivery contains its actual comparison or an explicit failure.

A comparison is a derived read, not a producer observation. Its result commits in its atomic
terminal journal; it does not invent a producer run or a catalog evidence publication. Restart
never repeats unfinished acquisition automatically. An interrupted comparison remains failed;
explicit resubmission reuses any independently committed acquisitions. A terminal journal result
survives restart, and result overflow follows the shared bounded delivery contract.

Retained inspection stays within the supplied context. ExecuteOnMiss may reuse a direct derived
context only when its release, declared environment constraints, static source closure and exact
query match, and its retained producer image/helper/containment identity satisfies the requested
execution selection. No current policy qualification is required for a read of already retained
facts; requested execution policy still controls starting any new producer. If several qualified
children match, present the alternatives and require an explicit child context; do not choose
by age or merge environments. Advancing time alone never invalidates an exact retained result.

### Consequences

A concrete query job is added to the closed journal variants. No new public tool or observation
kind is added. Two simultaneous cold acquisitions may occupy two records plus their parent;
queue rejection is explicit. A comparison interrupted before its terminal journal can require
an explicit retry, but committed documentation is reused.

### Compensating controls

Bounded journal/queue/query resources, per-subscriber tokens, exact context predicates, common
native comparison, and negative ambiguity/cancellation/restart tests.

## Evidence

This is an internal composition decision, not a new upstream capability claim. The primary
project contracts and inspected code establish the existing mechanisms.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Native evidence remains the core | [Frozen execution brief](../blueprint/AGENT_HANDOFF.md) | 2026-09-14 | “Use a Rust-owned core and immutable Arrow/Parquet evidence snapshots, with DataFusion for structured retrieval.” |
| Closed jobs already govern retained observations | [ADR-0026](0026-retained-execution-observations.md) | 2026-09-14 | “one `execution_observations` relation and concrete durable job variants” |
| Cold resolution has a pending route | [Resolver job](../../crates/enrichment-daemon/src/ops/resolve_job.rs) | 2026-09-14 | `verify::pending(record.data(Some(token)))` |

## Verification

`cold_version_comparison_waits_for_exact_acquisitions`,
`comparison_cancellation_preserves_another_subscriber`, and
`exact_derived_inspection_reuses_retained_observations` must prove cold/warm/restart behavior,
exact final payloads, independent cancellation and ambiguous-child refusal. Their current-source
logs belong in the Plan 11 execution ledger; their names here are requirements, not passed claims.

## Boundaries preserved

§B1–§B13 remain: Rust owns identities/policy/state/query/publication; Python stays thin; source
and environment isolation, epistemic distinctions, native evidence, separate Context7 and the
nine-tool transport contract remain intact. No legacy evidence migration or age expiry.

## More information

[Plan 11](../plans/11-arrow-datafusion-completion-and-legacy-removal.md), findings N1/N10 and W0/W2.

## Status history

- 2026-09-14 — proposed; implementation and scoped review in progress.

- 2026-09-14 — accepted after scoped native architecture review; remaining actual producer,
  client and final performance qualification stays in Plan 12.
