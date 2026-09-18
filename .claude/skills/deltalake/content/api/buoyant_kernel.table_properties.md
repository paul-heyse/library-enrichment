# `buoyant_kernel::table_properties`

Crate `buoyant_kernel` · 7 public items · structured records in [`model/buoyant_kernel.table_properties.json`](../model/buoyant_kernel.table_properties.json)

## DEFAULT_NUM_INDEXED_COLS

`constant` · `buoyant_kernel::table_properties::DEFAULT_NUM_INDEXED_COLS`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_properties.DEFAULT_NUM_INDEXED_COLS.md)

Also reachable as `delta_kernel::table_properties::DEFAULT_NUM_INDEXED_COLS`

```rust
const DEFAULT_NUM_INDEXED_COLS: u64 = 32
```

Default number of leaf columns to collect statistics on when `dataSkippingNumIndexedCols`
is not specified.

---

## DELTA_PROPERTY_PREFIX

`constant` · `buoyant_kernel::table_properties::DELTA_PROPERTY_PREFIX`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_properties.DELTA_PROPERTY_PREFIX.md)

Also reachable as `delta_kernel::table_properties::DELTA_PROPERTY_PREFIX`

```rust
const DELTA_PROPERTY_PREFIX: &str = "delta."
```

Prefix for delta table properties (e.g., `delta.enableChangeDataFeed`, `delta.appendOnly`).

---

## CheckpointPolicy

`enum` · `buoyant_kernel::table_properties::CheckpointPolicy`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_properties.CheckpointPolicy.md)

Also reachable as `delta_kernel::table_properties::CheckpointPolicy`

```rust
enum CheckpointPolicy
```

**Variants**: `Classic`, `V2`

**Implements**: `core::convert::TryFrom`, `core::str::traits::FromStr`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &str) -> ::core::result::Result<CheckpointPolicy, <Self as ::core::convert::TryFrom>::Error>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<CheckpointPolicy, <Self as ::core::str::FromStr>::Err>
```

The checkpoint policy applied when writing checkpoints

---

## DataSkippingNumIndexedCols

`enum` · `buoyant_kernel::table_properties::DataSkippingNumIndexedCols`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_properties.DataSkippingNumIndexedCols.md)

Also reachable as `delta_kernel::table_properties::DataSkippingNumIndexedCols`

```rust
enum DataSkippingNumIndexedCols
```

**Variants**: `AllColumns`, `NumColumns`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &str) -> Result<Self, Self::Error>
```

---

## IsolationLevel

`enum` · `buoyant_kernel::table_properties::IsolationLevel`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_properties.IsolationLevel.md)

Also reachable as `delta_kernel::table_properties::IsolationLevel`

```rust
enum IsolationLevel
```

**Variants**: `Serializable`, `WriteSerializable`, `SnapshotIsolation`

**Implements**: `core::convert::TryFrom`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &str) -> ::core::result::Result<IsolationLevel, <Self as ::core::convert::TryFrom>::Error>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<IsolationLevel, <Self as ::core::str::FromStr>::Err>
```

The isolation level applied during transaction

---

## ParquetCompressionCodec

`enum` · `buoyant_kernel::table_properties::ParquetCompressionCodec`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_properties.ParquetCompressionCodec.md)

Also reachable as `delta_kernel::table_properties::ParquetCompressionCodec`

```rust
enum ParquetCompressionCodec
```

**Variants**: `Zstd`, `Uncompressed`, `Snappy`, `Gzip`, `Lz4`, `Lz4Raw`

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &str) -> ::core::result::Result<ParquetCompressionCodec, <Self as ::core::convert::TryFrom>::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<ParquetCompressionCodec, <Self as ::core::str::FromStr>::Err>
```

Compression codec for newly written Parquet data files, controlled by the
`delta.parquet.compression.codec` table property.

Per the Delta protocol, parsing is case-insensitive, and `none` is accepted as an alias for
`uncompressed`. When the property is absent, writers SHOULD default to [`Self::Zstd`].

See [Table Properties] in the Delta protocol.

[Table Properties]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#table-properties

---

## TableProperties

`struct` · `buoyant_kernel::table_properties::TableProperties`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_properties.TableProperties.md)

Also reachable as `delta_kernel::table_properties::TableProperties`

```rust
struct TableProperties
```

**Fields**: `append_only`, `auto_compact`, `optimize_write`, `checkpoint_interval`, `checkpoint_write_stats_as_json`, `checkpoint_write_stats_as_struct`, `column_mapping_mode`, `column_mapping_max_column_id`, `data_skipping_num_indexed_cols`, `data_skipping_stats_columns`, `deleted_file_retention_duration`, `enable_change_data_feed`, `enable_deletion_vectors`, `enable_type_widening`, `enable_iceberg_compat_v1`, `enable_iceberg_compat_v2`, `enable_iceberg_compat_v3`, `isolation_level`, `log_retention_duration`, `enable_expired_log_cleanup`, `randomize_file_prefixes`, `random_prefix_length`, `set_transaction_retention_duration`, `target_file_size`, `tune_file_sizes_for_rewrites`, `checkpoint_policy`, `enable_row_tracking`, `row_tracking_suspended`, `materialized_row_id_column_name`, `materialized_row_commit_version_column_name`, `parquet_format_version`, `parquet_compression_codec`, `enable_in_commit_timestamps`, `in_commit_timestamp_enablement_version`, `in_commit_timestamp_enablement_timestamp`, `unknown_properties`

**Implements**: `core::convert::From`, `deltalake_core::table::config::TablePropertiesExt`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn compression_codec_or_default(&self) -> ParquetCompressionCodec
fn random_prefix_length(&self) -> NonZero<usize>
fn should_randomize_file_prefixes(&self) -> bool
fn should_write_stats_as_json(&self) -> bool
fn should_write_stats_as_struct(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(unparsed: I) -> Self
```

Delta table properties. These are parsed from the 'configuration' map in the most recent
'Metadata' action of a table.

Reference: <https://github.com/delta-io/delta/blob/master/spark/src/main/scala/org/apache/spark/sql/delta/DeltaConfig.scala>

---
