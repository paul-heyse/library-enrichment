# Operations

24 operation builders. Every one implements `IntoFuture`, so the shape is
always the same: take a builder from an entry point, chain `with_*` calls, then await it.
Nothing runs until the await.

```rust
let (table, metrics) = table.delete().with_predicate(expr).await?;
```

A builder with no `with_*` call still runs -- with defaults chosen for safety rather
than for your workload. The configuration column is where the capability is.

| Operation | Constructed by | Configuration | Defined in |
|---|---|---:|---|
| `AddColumnBuilder` | `DeltaTable::add_columns` | 3 | [`deltalake-core`](../api/deltalake_core.operations.add_column.md#addcolumnbuilder) |
| `AddTableFeatureBuilder` | `DeltaTable::add_feature` | 5 | [`deltalake-core`](../api/deltalake_core.operations.add_feature.md#addtablefeaturebuilder) |
| `ConstraintBuilder` | `DeltaTable::add_constraint` | 5 | [`deltalake-core`](../api/deltalake_core.operations.constraints.md#constraintbuilder) |
| `ConvertToDeltaBuilder` | — | 12 | [`deltalake-core`](../api/deltalake_core.operations.convert_to_delta.md#converttodeltabuilder) |
| `CreateBuilder` | `DeltaTable::create` | 15 | [`deltalake-core`](../api/deltalake_core.operations.create.md#createbuilder) |
| `DeleteBuilder` | `DeltaTable::delete`, `MergeBuilder::when_matched_delete`, `MergeBuilder::when_not_matched_by_source_delete` | 6 | [`deltalake-core`](../api/deltalake_core.operations.delete.md#deletebuilder) |
| `DropColumnNotNullBuilder` | `DeltaTable::drop_column_not_null` | 3 | [`deltalake-core`](../api/deltalake_core.operations.drop_column_not_null.md#dropcolumnnotnullbuilder) |
| `DropConstraintBuilder` | `DeltaTable::drop_constraints` | 4 | [`deltalake-core`](../api/deltalake_core.operations.drop_constraints.md#dropconstraintbuilder) |
| `FileSystemCheckBuilder` | `DeltaTable::filesystem_check` | 3 | [`deltalake-core`](../api/deltalake_core.operations.filesystem_check.md#filesystemcheckbuilder) |
| `GenerateBuilder` | `DeltaTable::generate` | 0 | [`deltalake-core`](../api/deltalake_core.operations.generate.md#generatebuilder) |
| `LoadBuilder` | `DeltaTable::scan_table` | 2 | [`deltalake-core`](../api/deltalake_core.operations.load.md#loadbuilder) |
| `MergeBuilder` | `DeltaTable::merge` | 10 | [`deltalake-core`](../api/deltalake_core.operations.merge.md#mergebuilder) |
| `OptimizeBuilder` | `DeltaTable::optimize` | 11 | [`deltalake-core`](../api/deltalake_core.operations.optimize.md#optimizebuilder) |
| `PostCommit` | — | 0 | [`deltalake-core`](../api/deltalake_core.kernel.transaction.md#postcommit) |
| `PreCommit` | `CommitBuilder::build` | 0 | [`deltalake-core`](../api/deltalake_core.kernel.transaction.md#precommit) |
| `PreparedCommit` | `PreCommit::into_prepared_commit_future` | 0 | [`deltalake-core`](../api/deltalake_core.kernel.transaction.md#preparedcommit) |
| `RestoreBuilder` | `DeltaTable::restore` | 6 | [`deltalake-core`](../api/deltalake_core.operations.restore.md#restorebuilder) |
| `SetTablePropertiesBuilder` | `DeltaTable::set_tbl_properties` | 4 | [`deltalake-core`](../api/deltalake_core.operations.set_tbl_properties.md#settablepropertiesbuilder) |
| `TableProviderBuilder` | `DeltaScan::builder`, `DeltaTable::table_provider` | 9 | [`deltalake-core`](../api/deltalake_core.delta_datafusion.table_provider.md#tableproviderbuilder) |
| `UpdateBuilder` | `DeltaTable::update`, `MergeBuilder::when_matched_update`, `MergeBuilder::when_not_matched_by_source_update` | 8 | [`deltalake-core`](../api/deltalake_core.operations.update.md#updatebuilder) |
| `UpdateFieldMetadataBuilder` | `DeltaTable::update_field_metadata` | 4 | [`deltalake-core`](../api/deltalake_core.operations.update_field_metadata.md#updatefieldmetadatabuilder) |
| `UpdateTableMetadataBuilder` | `DeltaTable::update_table_metadata` | 3 | [`deltalake-core`](../api/deltalake_core.operations.update_table_metadata.md#updatetablemetadatabuilder) |
| `VacuumBuilder` | `DeltaTable::vacuum` | 8 | [`deltalake-core`](../api/deltalake_core.operations.vacuum.md#vacuumbuilder) |
| `WriteBuilder` | `DeltaTable::write` | 18 | [`deltalake-core`](../api/deltalake_core.operations.write.md#writebuilder) |

## Reachable but not nameable

These builders are returned by a public method but live in a private module with no
re-export. You can call and await one; you cannot `use` it or name it in a signature.
rustdoc omits the impls of such a type, so their configuration count above reads 0
whatever the source says -- read the source before concluding one takes no options.

- `deltalake_core::operations::load::LoadBuilder`

## Configuration by operation

**`AddColumnBuilder`** — Add new columns and/or nested fields to a table

`with_commit_properties` `with_custom_execute_handler` `with_fields`

**`AddTableFeatureBuilder`** — Enable table features for a table

`with_allow_protocol_versions_increase` `with_commit_properties` `with_custom_execute_handler` `with_feature` `with_features`

**`ConstraintBuilder`** — Build a constraint to add to a table

`with_commit_properties` `with_constraint` `with_constraints` `with_custom_execute_handler` `with_session_state`

**`ConvertToDeltaBuilder`** — Build an operation to convert a Parquet table to a [`DeltaTable`] in place

`with_comment` `with_commit_properties` `with_configuration` `with_configuration_property` `with_custom_execute_handler` `with_location` `with_log_store` `with_partition_schema` `with_partition_strategy` `with_save_mode` `with_storage_options` `with_table_name`

**`CreateBuilder`** — Build an operation to create a new [DeltaTable]

`with_actions` `with_column` `with_columns` `with_comment` `with_commit_properties` `with_configuration` `with_configuration_property` `with_custom_execute_handler` `with_location` `with_log_store` `with_partition_columns` `with_raise_if_key_not_exists` `with_save_mode` `with_storage_options` `with_table_name`

**`DeleteBuilder`** — Delete Records from the Delta Table. See this module's documentation for more information

`with_commit_properties` `with_custom_execute_handler` `with_predicate` `with_session_fallback_policy` `with_session_state` `with_writer_properties`

**`DropColumnNotNullBuilder`** — Drop the `NOT NULL` constraint on a top-level column, making it nullable.

`with_column` `with_commit_properties` `with_custom_execute_handler`

**`DropConstraintBuilder`** — Remove constraints from the table

`with_commit_properties` `with_constraint` `with_custom_execute_handler` `with_raise_if_not_exists`

**`FileSystemCheckBuilder`** — Audit the Delta Table's active files with the underlying file system. See this module's documentation for more information

`with_commit_properties` `with_custom_execute_handler` `with_dry_run`

**`LoadBuilder`** — no summary

`with_columns` `with_session_state`

**`MergeBuilder`** — Merge records into a Delta Table.

`with_commit_properties` `with_custom_execute_handler` `with_merge_schema` `with_safe_cast` `with_session_fallback_policy` `with_session_state` `with_source_alias` `with_streaming` `with_target_alias` `with_writer_properties`

**`OptimizeBuilder`** — Optimize a Delta table with given options

`with_commit_properties` `with_custom_execute_handler` `with_filters` `with_max_concurrent_tasks` `with_min_commit_interval` `with_preserve_insertion_order` `with_session_fallback_policy` `with_session_state` `with_target_size` `with_type` `with_writer_properties`

**`RestoreBuilder`** — Restore a Delta table with given version See this module's documentation for more information

`with_commit_properties` `with_custom_execute_handler` `with_datetime_to_restore` `with_ignore_missing_files` `with_protocol_downgrade_allowed` `with_version_to_restore`

**`SetTablePropertiesBuilder`** — Remove constraints from the table

`with_commit_properties` `with_custom_execute_handler` `with_properties` `with_raise_if_not_exists`

**`TableProviderBuilder`** — Builder for a datafusion [TableProvider] for a Delta table

`with_adds` `with_eager_snapshot` `with_file_column` `with_file_paths` `with_file_selection` `with_log_store` `with_session` `with_snapshot` `with_table_version`

**`UpdateBuilder`** — Updates records in the Delta Table. See this module's documentation for more information

`with_commit_properties` `with_custom_execute_handler` `with_predicate` `with_safe_cast` `with_session_fallback_policy` `with_session_state` `with_update` `with_writer_properties`

**`UpdateFieldMetadataBuilder`** — Update a field's metadata in a schema. If the key does not exists, the entry is inserted.

`with_commit_properties` `with_custom_execute_handler` `with_field_name` `with_metadata`

**`UpdateTableMetadataBuilder`** — Update table metadata operation

`with_commit_properties` `with_custom_execute_handler` `with_update`

**`VacuumBuilder`** — Vacuum a Delta table with the given options See this module's documentation for more information

`with_commit_properties` `with_custom_execute_handler` `with_dry_run` `with_enforce_retention_duration` `with_keep_versions` `with_mode` `with_retention_period` `with_scan_concurrency`

**`WriteBuilder`** — Write data into a DeltaTable

`with_cast_safety` `with_commit_properties` `with_configuration` `with_custom_execute_handler` `with_description` `with_input_batches` `with_input_execution_plan` `with_input_plan` `with_partition_columns` `with_replace_where` `with_save_mode` `with_schema_mode` `with_session_fallback_policy` `with_session_state` `with_table_name` `with_target_file_size` `with_write_batch_size` `with_writer_properties`

