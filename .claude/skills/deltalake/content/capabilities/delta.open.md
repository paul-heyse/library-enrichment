# Open, load and select a table version

Separate a table handle from its loaded snapshot. Select the required version before building a provider; a provider created from an already loaded table retains that snapshot.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| DeltaTableBuilder::load / open helpers | You need an existing table snapshot | Performs log access; protocol/storage errors remain possible |
| DeltaTableBuilder::build | You need a configured handle before loading or creating | The handle has no loaded version |
| load_version / load_with_datetime | A reproducible historical view is required | Historical data must still exist; loading log state alone does not prove files are readable |

## Contract

**state.** A built unloaded handle has version None. A loaded snapshot is a specific version; refreshing one clone does not refresh an existing provider.
Claim `delta.open.1`; source_observation; evidence: upstream, source.

**version.** In the local probe, a provider built from an unloaded handle with table_version(0) selects v0. For a loaded handle, its snapshot takes precedence over the builder's version option.
Claim `delta.open.2`; runtime_observation; evidence: runtime.

**time travel.** Time travel selects historical state without a new commit. Restore instead commits a new current version referencing historical state.
Claim `delta.open.3`; runtime_observation; evidence: runtime.

## Implementation

- Choose URL, storage options and required version explicitly at the boundary where freshness is decided.
- Check version() and snapshot state before producing a provider; rebuild the provider after refreshing if fresh results are required.

## Effects

- read log and metadata
- select snapshot; no table commit

## Errors

- Load errors concern URL, log, protocol or snapshot selection. Missing data can surface later while consuming a scan.

## Limits and unknowns

- Timestamp selection at every retention/protocol boundary is not exhaustively tested.

## Exact contracts

- [`deltalake_core::table::DeltaTable::try_from_url`](../operations/deltalake_core.table.DeltaTable.md#op-f74e66012b666ef0fd0eb264) — `async fn try_from_url(uri: Url) -> DeltaResult<Self>`
- [`deltalake_core::table::DeltaTable::load_version`](../operations/deltalake_core.table.DeltaTable.md#op-bd07223c753e46cf67ac2d8a) — `async fn load_version(&mut self, version: Version) -> Result<(), DeltaTableError>`
- [`deltalake_core::table::DeltaTable::load_with_datetime`](../operations/deltalake_core.table.DeltaTable.md#op-fbd40a4dc41d75936671eb6f) — `async fn load_with_datetime(&mut self, datetime: DateTime<Utc>) -> Result<(), DeltaTableError>`
- [`deltalake_core::table::builder::DeltaTableBuilder::build`](../operations/deltalake_core.table.builder.DeltaTableBuilder.md#op-ce34f4c8678c321fcc13b373) — `fn build(self) -> DeltaResult<DeltaTable>`
- [`deltalake_core::table::builder::DeltaTableBuilder::load`](../operations/deltalake_core.table.builder.DeltaTableBuilder.md#op-7e06e6511ff81a276fce5093) — `async fn load(self) -> DeltaResult<DeltaTable>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/table/builder.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: unloaded_handle_explicit_version_and_partition_reconstruction, restore_creates_new_version_and_checkpoint_preserves_rows
