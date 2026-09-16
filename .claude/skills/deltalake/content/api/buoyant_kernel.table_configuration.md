# `buoyant_kernel::table_configuration`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.table_configuration.json`](../model/buoyant_kernel.table_configuration.json)

## ExpectedStatsSchemas

`struct` · `buoyant_kernel::table_configuration::ExpectedStatsSchemas`

Also reachable as `delta_kernel::table_configuration::ExpectedStatsSchemas`

```rust
struct ExpectedStatsSchemas
```

**Fields**: `physical`

**Derives**: Clone, Debug

Expected schema for file statistics, using physical column names.

Wrapped in a struct so it can be extended with a logical-name variant if needed.

---

## TableConfiguration

`struct` · `buoyant_kernel::table_configuration::TableConfiguration`

Also reachable as `delta_kernel::table_configuration::TableConfiguration`

```rust
struct TableConfiguration
```

**Implements**: `deltalake_core::kernel::arrow::engine_ext::SnapshotExt`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (15)

```rust
fn build_expected_stats_schemas(&self, required_physical_columns: Option<&[ColumnName]>, requested_physical_columns: Option<&[ColumnName]>) -> DeltaResult<ExpectedStatsSchemas>
fn column_mapping_mode(&self) -> ColumnMappingMode
fn ensure_operation_supported(&self, operation: Operation) -> DeltaResult<()>
fn is_catalog_managed(&self) -> bool
fn is_feature_enabled(&self, feature: &TableFeature) -> bool
fn is_feature_supported(&self, feature: &TableFeature) -> bool
fn logical_schema(&self) -> SchemaRef
fn metadata(&self) -> &Metadata
fn partition_columns(&self) -> &[String]
fn physical_schema(&self) -> SchemaRef
fn protocol(&self) -> &Protocol
fn table_properties(&self) -> &TableProperties
fn table_root(&self) -> &Url
fn try_new(metadata: Metadata, protocol: Protocol, table_root: Url, version: Version) -> DeltaResult<Self>
fn version(&self) -> Version
```

Holds all the configuration for a table at a specific version. This includes the supported
reader and writer features, table properties, schema, version, and table root. This can be used
to check whether a table supports a feature or has it enabled. For example, deletion vector
support can be checked with [`TableConfiguration::is_feature_supported`] and deletion
vector write enablement can be checked with [`TableConfiguration::is_feature_enabled`].

[`TableConfiguration`] performs checks upon construction with `TableConfiguration::try_new`
to validate that Metadata and Protocol are correctly formatted and mutually compatible.
After construction, call `ensure_operation_supported` to verify that the kernel supports the
required operations for the table's protocol features.

---
