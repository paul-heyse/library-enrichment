# Choose time travel, restore, checkpoints or log maintenance

Time travel reads an old state, restore publishes a new state, and checkpoints accelerate state reconstruction. These operations solve different lifecycle tasks.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| load_version | You want an historical read without changing head | Referenced files must remain available |
| restore | Current head should reflect an earlier state | Creates a new commit; missing files and protocol constraints matter |
| create_checkpoint | You need a checkpoint for log reconstruction | Does not compact user data or restore removed bytes |
| history | Inspect recorded commit metadata for diagnostics/recovery | History availability is limited by retained log state |

## Contract

**restore.** Restoring v0 from a table at v1 produced v2 with the original row; v1 still selected the two-row historical state in the fixture.
Claim `delta.history.1`; runtime_observation; evidence: runtime.

**checkpoint.** Creating a checkpoint preserved the current relation in the local fixture.
Claim `delta.history.2`; runtime_observation; evidence: runtime.

**separation.** Data compaction, log checkpointing, log cleanup and vacuum have different inputs and physical effects; select the intended layer.
Claim `delta.history.3`; source_observation; evidence: upstream, source.

## Implementation

- Record version before/after restore, and test historical data availability independently.
- Use history and application commit metadata for diagnosis without treating them as an indefinitely retained audit log.

## Effects

- select state
- publish restore version
- write checkpoint metadata

## Errors

- An old version may have loadable metadata but unavailable files; restoration has its own missing-file policy.

## Limits and unknowns

- Every checkpoint format and cleanup/retention combination is not covered by the fixture.

## Exact contracts

- [`deltalake_core::table::DeltaTable::history`](../operations/deltalake_core.table.DeltaTable.md#op-80e9ed24a9f0a3008cd9a869) — `async fn history(&self, limit: Option<usize>) -> Result<impl Iterator<Item = CommitInfo> + use<>, DeltaTableError>`
- [`deltalake_core::table::DeltaTable::restore`](../operations/deltalake_core.table.DeltaTable.md#op-ccc2e63ad429a29aab780d31) — `fn restore(self) -> RestoreBuilder`
- [`deltalake_core::table::DeltaTable::load_version`](../operations/deltalake_core.table.DeltaTable.md#op-bd07223c753e46cf67ac2d8a) — `async fn load_version(&mut self, version: Version) -> Result<(), DeltaTableError>`
- [`deltalake_core::operations::restore::RestoreBuilder::with_version_to_restore`](../operations/deltalake_core.operations.restore.RestoreBuilder.md#op-45dc160b98acde43d1355b28) — `fn with_version_to_restore(self, version: Version) -> Self`
- [`deltalake_core::protocol::checkpoints::create_checkpoint`](../operations/deltalake_core.protocol.checkpoints.create_checkpoint.md#op-e2ce7c6abdadd100ddb3dbb1) — `async fn create_checkpoint(table: &DeltaTable, operation_id: Option<uuid::Uuid>) -> DeltaResult<()>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/restore.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: restore_creates_new_version_and_checkpoint_preserves_rows
