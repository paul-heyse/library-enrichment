# Reason about optimistic commits and errors after publication

Separate data preparation, optimistic log publication and post-commit work. An operation error does not establish that the table is unchanged.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| High-level operation with CommitProperties | Built-in operation already captures the needed read/write semantics | Customize metadata and retry policy without reproducing actions |
| CommitBuilder | You own valid protocol actions and their transactional meaning | You must supply correct operation/read-set semantics and recovery |
| Reload/reconcile after ambiguous error | The failure may have occurred after log publication | Observe table state before retrying data production |

## Contract

**failure phase.** The injected before_post_commit_hook error returned failure while a fresh observer saw the new version and appended rows. This proves local log visibility, not power-loss durability.
Claim `delta.commit.1`; runtime_observation; evidence: runtime.

**retry.** A stale append with max_retries(0) failed after an intervening commit; the default retry path appended successfully. A stale whole-table update conflicted with an intervening update.
Claim `delta.commit.2`; runtime_observation; evidence: runtime.

**backend.** The publication primitive is delegated to the chosen LogStore/backend. Do not assume every backend commits by the same atomic-rename protocol.
Claim `delta.commit.3`; source_observation; evidence: upstream, source.

## Implementation

- Record application/operation metadata sufficient for reconciliation and classify errors by the phase they can occur in.
- Use the returned version/table on success; on an ambiguous error, independently reload and compare committed state.
- Treat max_retries as a transaction policy, not a universal retry around arbitrary source side effects.

## Effects

- write files
- publish log version
- post-commit checkpoint/cleanup/hooks

## Errors

- Precondition failure, optimistic conflict, publication failure and post-commit failure can imply different visible states.

## Limits and unknowns

- Crash recovery, remote backend atomicity and distributed scheduling are not inferred from local-memory publication.

## Exact contracts

- [`deltalake_core::kernel::transaction::CommitBuilder::build`](../operations/deltalake_core.kernel.transaction.CommitBuilder.md#op-d53693381c75904ce089c98e) — `fn build(self, table_data: Option<&'a dyn TableReference>, log_store: LogStoreRef, operation: DeltaOperation) -> PreCommit<'a>`
- [`deltalake_core::kernel::transaction::CommitProperties::with_max_retries`](../operations/deltalake_core.kernel.transaction.CommitProperties.md#op-521f71362ddaf4ec207f580c) — `fn with_max_retries(self, max_retries: usize) -> Self`
- [`deltalake_core::operations::write::WriteBuilder::with_custom_execute_handler`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-a2d7c2a33a0ad647e936c3cb) — `fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self`
- [`deltalake_core::kernel::transaction::CommitProperties::with_metadata`](../operations/deltalake_core.kernel.transaction.CommitProperties.md#op-55f8ddb14dbf4d7449f3f2b1) — `fn with_metadata(self, metadata: impl IntoIterator<Item = (String, serde_json::Value)>) -> Self`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/kernel/transaction/mod.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: operation_error_can_follow_a_durable_commit, optimistic_conflict_and_retry_budget
