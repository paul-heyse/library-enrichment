---
id: ADR-0050
title: Preserve exact retention through native Delta maintenance
status: accepted
date: 2026-09-16
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-16, DM-42]
design: [§17]
review: not-required: Narrow maintenance corrections within the accepted Plan 17 retention architecture.
evidence: Implemented
supersedes: []
superseded-by: null
revisit: An exact upstream release protects retained deletion vectors, bounds complete log inventory and preserves supplied maintenance commit properties.
verification: maintenance_log_inventory_refuses_before_delete_and_keeps_exact_history, native vacuum protection tests and Plan 17 Q10.
---

# ADR-0050: Preserve exact retention through native Delta maintenance

## Context

Plan 17 requires exact durable protection before loading providers or reclaiming data.
The pinned native VACUUM protection set omits deletion vectors, log cleanup discards listing
errors, and maintenance stages reconstruct default commit properties. These gaps prevent
the service's single retention policy from governing native maintenance end to end.

## Scope

Patch the existing delta-rs core vendor at `58f07cd62bfbce3649a7e1c87c696288068ae184`.
Keep DataFusion 55.1.0, Arrow 59.3.0 and the pinned Buoyant kernel. This is an implementation
correction within §17; it does not relax retention or qualify the complete maintenance route.

## Drivers

Protect every dependency of retained versions. Require complete bounded observations before
deletion. Preserve the supplied session, runtime, retry and cleanup policy across native stages.

## Options

- Native defaults leave retention and boundedness gaps.
- Application implementations duplicate native vacuum, log and transaction machinery.
- Narrow native corrections preserve the provider architecture and are selected.

## Decision

Collect each active Add's data path and external deletion-vector object in the existing
current/retained-version protection helper. Convert the Delta descriptor through the kernel's
checked constructor and `absolute_path`, then use native object-store path conversion. Reject
malformed or foreign-root descriptors before deletion. Inline vectors require no object path.
Root containment is a service maintenance policy; it is not a Delta protocol restriction.

Log cleanup must finish a bounded entry/metadata inventory before constructing the delete
stream. Listing errors, invalid timestamps and exceeded budgets refuse deletion. Preserve
the native checkpoint/version/cutoff selection algorithm. A deletion error remains an error
even after partial removal; no atomic rollback is claimed.

Preserve the supplied `CommitProperties` in VACUUM START and every OPTIMIZE commit. Do not
override the configured retry budget. OPTIMIZE advances its snapshot after each own commit;
that is its next predecessor. The durable maintenance run records whole-operation completion;
a transaction marker in a partial native commit cannot stand in for that record.

### Consequences

The application keeps native maintenance builders. The vendor patch and its provenance must
remain until an exact upstream replacement passes the affected routes. The complete service
still needs writer/CDF/export/control enrollment, recovery and destructive-maintenance qualification.

### Compensating controls

Complete dependency closure and maintenance generation fencing remain mandatory. A live CDF
window defers table vacuum. Published evidence/results have no automatic expiry. Automatic
post-commit log cleanup is disabled; explicit native cleanup receives the selected safe floor.
R-48 tracks retirement of this patch. The native worker identity includes vendored source.

## Evidence

Retrieved 2026-09-16 from exact local primary sources, independently inspected in
[maintenance-upstream-check.md](../design_review/reviews/evidence/combined-schema-runtime-plan-2026-09-16/maintenance-upstream-check.md).

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Existing protection collects only data paths | [Delta vacuum](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L72) | 2026-09-16 | `.map_ok(\|file\| file.object_store_path())` |
| Kernel resolves vector locations | [Kernel descriptor](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L257) | 2026-09-16 | `pub fn absolute_path` |
| Cleanup materializes a fallible inventory | [Delta checkpoints](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/checkpoints.rs#L123) | 2026-09-16 | `collect::<Vec<_>>()` |
| OPTIMIZE replaces caller retry policy | [Delta optimize](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L988) | 2026-09-16 | `.with_max_retries(DEFAULT_RETRIES + commits_made)` |

## Verification

`maintenance_log_inventory_refuses_before_delete_and_keeps_exact_history` exercises actual
Delta checkpoint/cleanup and protected-version readback. Native vacuum oracles must cover
current/historical vectors, same-root encoded paths, malformed/outside-root descriptors and
actual unrelated-file reclamation. Commit-policy qualification must include stale predecessors.
Plan 17 Q10 remains `not_run` until its complete obligations execute.

## Boundaries preserved

All §B1–§B13 remain binding. Rust owns policy and physical mechanisms; DataFusion selects
relations and Delta owns commits. Exact publication, evidence distinctions, repository
immutability and the thin Python adapter remain unchanged.

## More information

[Plan 17](../plans/17-schema-governed-unified-runtime-hard-pivot.md),
[ADR register](register.md), `scripts/vendor-provenance.py`.

## Status history

- 2026-09-16 — accepted within the authorized hard pivot; implemented, full qualification open.
