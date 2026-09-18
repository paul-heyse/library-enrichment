# Use transaction markers without assuming replay suppression

Application transaction markers are useful persisted coordination metadata. At this pin, repeating the same marker on sequential appends does not automatically suppress the second write.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| Application transactions + explicit recovery check | An application coordinates checkpoints with committed table state | Design race/retry handling; a pre-read alone is not atomic deduplication |
| Plain append | Every invocation is intended to add records | Repeated requests repeat data |
| Domain-key merge or externally coordinated ingestion | You need a stronger uniqueness/replay policy | Define key semantics and concurrent ownership explicitly |

## Contract

**observed replay.** Two sequential writes with the same application marker version 7 produced two rows and table version 1; transaction_version returned Some(7).
Claim `delta.replay.1`; runtime_observation; evidence: runtime.

**coordination.** A txn marker participates in transaction state and conflict logic, but the marker alone is not an exactly-once ingestion algorithm.
Claim `delta.replay.2`; source_observation; evidence: upstream, source.

**recovery.** After an ambiguous error, reload and inspect the application marker, commit metadata/history and data outcome before choosing whether to retry. Retention may limit the evidence available.
Claim `delta.replay.3`; source_observation; evidence: upstream, source.

**controlled interleaving.** A stale writer carrying the same application marker conflicted after the competing writer committed; this does not change the separately observed sequential replay behavior.
Claim `delta.replay.4`; runtime_observation; evidence: runtime.

## Implementation

- Persist a meaningful application batch identity in commit properties and define how it maps to source offsets.
- Specify concurrent ownership, conflict handling and reconciliation, then test duplicate delivery and failures after publication.
- Do not advertise with_commit_properties(Default::default()) as idempotency.

## Effects

- persist coordination metadata
- may append duplicate rows if retried

## Errors

- A repeated marker and a retryable storage/transaction failure are different conditions.

## Limits and unknowns

- No distributed exactly-once protocol or external checkpoint service is qualified by the local tests.

## Exact contracts

- [`deltalake_core::kernel::transaction::CommitProperties::with_application_transaction`](../operations/deltalake_core.kernel.transaction.CommitProperties.md#op-98963111c785f5e16e3f10e0) — `fn with_application_transaction(self, txn: Transaction) -> Self`
- [`deltalake_core::kernel::transaction::CommitProperties::with_application_transactions`](../operations/deltalake_core.kernel.transaction.CommitProperties.md#op-5120171ce9751caba37fe1a8) — `fn with_application_transactions(self, txn: Vec<Transaction>) -> Self`
- [`deltalake_core::kernel::snapshot::EagerSnapshot::transaction_version`](../operations/deltalake_core.kernel.snapshot.EagerSnapshot.md#op-3d21216ceac03842b7a8cbcd) — `async fn transaction_version(&self, log_store: &dyn LogStore, app_id: impl ToString) -> DeltaResult<Option<i64>>`
- [`deltalake_core::operations::write::WriteBuilder::with_commit_properties`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-e9145ea9173ea093f8f03b9c) — `fn with_commit_properties(self, commit_properties: CommitProperties) -> Self`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/kernel/transaction/mod.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: repeated_transaction_marker_does_not_suppress_sequential_append, competing_marker_writers_conflict_and_cdf_enablement_is_historical
