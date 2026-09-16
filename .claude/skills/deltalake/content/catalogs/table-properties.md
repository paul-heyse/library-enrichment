# Table properties

24 properties are modelled as `TableProperty` variants. They are stored in
the table metadata as `delta.*` strings, so a property this build does not model is not
an error at write time -- it is simply ignored, which is the failure worth knowing about.

Set them with `DeltaTable::set_tbl_properties`, or at creation with
`CreateBuilder::with_configuration`. Read them back through `TablePropertiesExt`
(10 accessors), which is an extension trait on the kernel's
`TableProperties` -- so the accessors are invisible until you import it.

| Property variant |
|---|
| `AppendOnly` |
| `AutoOptimizeAutoCompact` |
| `AutoOptimizeOptimizeWrite` |
| `CheckpointInterval` |
| `CheckpointPolicy` |
| `CheckpointUseRunLengthEncoding` |
| `CheckpointWriteStatsAsJson` |
| `CheckpointWriteStatsAsStruct` |
| `ColumnMappingMode` |
| `DataSkippingNumIndexedCols` |
| `DataSkippingStatsColumns` |
| `DeletedFileRetentionDuration` |
| `EnableChangeDataFeed` |
| `EnableDeletionVectors` |
| `EnableExpiredLogCleanup` |
| `IsolationLevel` |
| `LogRetentionDuration` |
| `MinReaderVersion` |
| `MinWriterVersion` |
| `RandomPrefixLength` |
| `RandomizeFilePrefixes` |
| `SetTransactionRetentionDuration` |
| `TargetFileSize` |
| `TuneFileSizesForRewrites` |

## Reading them back

`TableConfig` accessors:

`append_only`, `checkpoint_interval`, `deleted_file_retention_duration`, `enable_change_data_feed`, `enable_expired_log_cleanup`, `get_constraints`, `isolation_level`, `log_retention_duration`, `num_indexed_cols`, `target_file_size`

Cross-check against upstream's own view in
[`../corpus/guides/feature-table.md`](../corpus/guides/feature-table.md).
Where the two disagree, this table is what the pinned commit compiles.
