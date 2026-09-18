# Choose full overwrite, predicate replacement or merge

Use Overwrite plus replace_where for predicate-scoped replacement. Validate incoming rows against that predicate; use merge when changes depend on matching source and target keys.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| Overwrite + replace_where | Incoming data fully replaces the selected logical slice | Incoming rows outside the predicate are invalid |
| Full Overwrite | The entire current table relation is being replaced | Old data files may remain physically present until retention cleanup |
| Merge | Insert/update/delete decisions depend on source-target matching | Requires explicit key/null/duplicate/clause semantics |

## Contract

**predicate.** The probe rejected an out-of-predicate row without advancing the observed version. Replacing id >= 2 with id 20 preserved id 1; full overwrite then left only id 9.
Claim `delta.replace.1`; runtime_observation; evidence: runtime.

**schema.** SchemaMode::Overwrite is allowed with SaveMode::Overwrite, not append. Existing column-mapped tables reject schema evolution on the examined write precondition path.
Claim `delta.replace.2`; source_observation; evidence: upstream, source.

**partitions.** Partition metadata is part of table layout; do not treat a new with_partition_columns value as an independent per-write reshuffle option. Inspect the existing partition schema and the exact overwrite path.
Claim `delta.replace.3`; source_observation; evidence: upstream, source.

## Implementation

- Express the logical slice in the replacement predicate and assert all incoming rows belong to it.
- Test retained rows and resulting metadata/version, not only the number of new files.

## Effects

- remove existing logical data
- add replacement files
- publish log version

## Errors

- Predicate, append-only/protocol, partition or schema validation can reject an operation before publication.

## Limits and unknowns

- All partition-layout changes and every mapped-schema overwrite combination are not covered by the local probe.

## Exact contracts

- [`deltalake_core::operations::write::WriteBuilder::with_replace_where`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-fe70b99393fab335b2454bd5) — `fn with_replace_where(self, predicate: impl Into<Expression>) -> Self`
- [`deltalake_core::operations::write::WriteBuilder::with_save_mode`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-a2d377816dc436cf1c93443b) — `fn with_save_mode(self, save_mode: SaveMode) -> Self`
- [`deltalake_core::operations::write::WriteBuilder::with_partition_columns`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-3c7a11e509dfe629ee2c3f13) — `fn with_partition_columns(self, partition_columns: impl IntoIterator<Item = impl Into<String>>) -> Self`
- [`deltalake_core::operations::write::WriteBuilder::with_schema_mode`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-290b8fe3fd386ffca8f17f0b) — `fn with_schema_mode(self, schema_mode: SchemaMode) -> Self`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/write/mod.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: save_modes_replace_where_and_validation_before_commit
