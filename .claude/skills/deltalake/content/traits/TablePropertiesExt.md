# TablePropertiesExt

`deltalake_core::table::config::TablePropertiesExt`

```rust
trait TablePropertiesExt
```

Also reachable as `deltalake::table::config::TablePropertiesExt`

Prose: [`api/deltalake_core.table.config.md`](../api/deltalake_core.table.config.md#tablepropertiesext) · records: [`model/deltalake_core.table.config.json`](../model/deltalake_core.table.config.json)

## Required

Every implementation must supply these.

```rust
fn append_only(&self) -> bool
fn checkpoint_interval(&self) -> NonZero<u64>
fn deleted_file_retention_duration(&self) -> Duration
fn enable_change_data_feed(&self) -> bool
fn enable_expired_log_cleanup(&self) -> bool
fn get_constraints(&self) -> Vec<Constraint>
fn isolation_level(&self) -> IsolationLevel
fn log_retention_duration(&self) -> Duration
fn num_indexed_cols(&self) -> DataSkippingNumIndexedCols
fn target_file_size(&self) -> NonZero<u64>
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::table_properties::TableProperties`

## Documentation

Convenience accessors for reading well-known Delta table properties with their defaults
applied, layered on top of the raw [`TableProperties`] parsed from table metadata.
