---
id: ADR-0030
title: Admit complete delivery artifacts before committing successful jobs
status: superseded
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-27, DM-30, DM-33, DM-35]
design: [§7.3, §8.2, §8.3]
review: docs/design_review/reviews/design_review_native-architecture-completion_2026-09-14.md
evidence: Proposed
supersedes: []
superseded-by: ADR-0037
revisit: A supported operation needs a complete delivery document larger than 32 MiB.
verification: delivery_failure_prevents_job_catalog_commit; committed_overflow_delivery_recovers_without_writes; exported_job_delivery_has_complete_artifact_closure
---

# ADR-0030: Admit complete delivery artifacts before committing successful jobs

## Context

Plan 11 W2 requires required result artifacts to be admitted before successful publication.
The shared encoder bounds replies and journals, but `Jobs::finish` still performs overflow writes
after a catalog commit. Disk exhaustion at that point can leave a committed active journal.
Inspect's separate 1.5 MiB rejection also prevents using the common large-result contract.

## Scope

Amend delivery, catalog publication and concrete terminal-job recovery in design §7.3, §8.2
and §8.3. Resolve, Inspect and Verify keep one publication coordinator. Compare remains a derived
read committed through its atomic terminal journal, as proposed by ADR-0028. No evidence
migration or historical-layout reader is introduced; the catalog format changes explicitly.

## Drivers

A successful publication must have a complete, bounded, retrievable answer. Restart must use
admitted bytes without executing producers or needing a new delivery-artifact write.

## Options

- Write overflow after commit: rejected; an artifact I/O failure can strand committed work.
- Reject every response larger than the journal: rejected; conflates transport and result budgets.
- Relabel a committed success as a failed producer: rejected; contradicts the catalog evidence.
- Prepare a bounded immutable delivery artifact before the catalog selection: selected.

## Decision

Every concrete producer-job publication requires an immutable canonical complete answer,
excluding transport request identity, at most 32 MiB. Its complete context and snapshot identity
are supplied after the candidate manifest is known and before the catalog root is committed.
One bounded preparation callback in the existing publication coordinator performs this step;
it is not a scheduling or workflow facility. A rebase prepares a new answer for the new candidate.

The catalog job-publication row carries a typed artifact descriptor for that delivery document,
separate from the producer's result/input/attempt closure. A presentation artifact is never
invented as a producer input. Its content identity, exact scope and bound are admitted before
selection. File and directory durability precede the catalog root barrier. Output-size or
artifact-write failure prevents that job publication.

The prepared journal envelope is held through commit. Replies fitting the journal stay inline;
larger replies reference the already durable artifact. Restart validates the catalog closure,
then reads a bounded inline answer or constructs a small handle envelope directly from the
committed descriptor. It does not regenerate, re-execute or write a replacement result artifact.
Export includes these typed delivery references and their verified bytes. Ordinary read replies
and comparison results continue to use the same size/overflow encoder and paging surface.

### Consequences

Even a small published producer-job answer has a durable artifact. This adds bounded write and
hash work, which W7 must measure. Unselected artifacts from failed/rebased candidates are ordinary
unreferenced owned evidence, reclaimable only through explicit cleanup. No clock-based expiry is
introduced. The catalog schema and fresh development state version must advance together.

### Compensating controls

Mandatory delivery descriptors, precommit byte/hash/scope admission, fixed result/journal bounds,
lease-protected publication/export, fresh-state version checks and targeted disk/crash tests.

## Evidence

This is an internal publication decision. It makes no new upstream capability claim.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Publication must validate evidence before visibility | [Living design §8.2](../design/DESIGN.md#82-single-flight-and-publication) | 2026-09-14 | “Readers never observe partially written tables.” |
| Plan requires precommit delivery admission | [Plan 11 W2](../plans/11-arrow-datafusion-completion-and-legacy-removal.md#w2--finish-durable-resolution-retained-execution-and-delivery) | 2026-09-14 | “Admit required result artifacts before successful job publication.” |
| Existing journal finish may create a result artifact | [Job finish](../../crates/enrichment-daemon/src/jobs.rs) | 2026-09-14 | `let result = bound_delivery(&self.root, result)?;` |

## Verification

`delivery_failure_prevents_job_catalog_commit`,
`committed_overflow_delivery_recovers_without_writes`, and
`exported_job_delivery_has_complete_artifact_closure` are required new oracles. They must cover
Resolve, Inspect and Verify, escaped Unicode, over-limit output, unwritable/full storage, restart
between catalog and terminal journal, and read-only exported closure. Existing comparison
cancellation and terminal-overflow tests remain required. These names are obligations, not
claims of completed execution. Current-source logs belong in the Plan 11 execution ledger.

## Boundaries preserved

§B1–§B13 remain intact: Rust owns identity/state/policy/publication, facts remain typed Arrow and
Parquet with native DataFusion retrieval, Python remains an adapter/worker, epistemic classes and
execution isolation are unchanged. The separate bounded presentation artifact introduces no
query authority, storage engine, generic job language or public SQL/shell tool.

## More information

[Plan 11 execution ledger](../plans/11-arrow-datafusion-execution-ledger.md),
[ADR-0023](0023-relational-catalog-publication.md),
[ADR-0026](0026-retained-execution-observations.md),
[ADR-0028](0028-durable-comparison-and-retained-selection.md), and
[ADR-0029](0029-bounded-inspection-projections.md).

## Status history

- 2026-09-14 — proposed; implementation, failure oracles and scoped review remain required.

- 2026-09-14 — accepted after scoped native architecture review; remaining actual producer,
  client and final performance qualification stays in Plan 12.
- 2026-09-15 — superseded by ADR-0037.
