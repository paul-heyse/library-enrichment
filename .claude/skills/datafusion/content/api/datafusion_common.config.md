# `datafusion_common::config`

Crate `datafusion-common` · 39 public items · structured records in [`model/datafusion_common.config.json`](../model/datafusion_common.config.json)

## DATAFUSION_FFI_CONFIG_NAMESPACE

`constant` · `datafusion_common::config::DATAFUSION_FFI_CONFIG_NAMESPACE`

Also reachable as `datafusion::config::DATAFUSION_FFI_CONFIG_NAMESPACE`

```rust
const DATAFUSION_FFI_CONFIG_NAMESPACE: &str = "datafusion_ffi"
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.DATAFUSION_FFI_CONFIG_NAMESPACE.md).


This namespace is reserved for interacting with Foreign Function Interface
(FFI) based configuration extensions.

---

## ConfigFileType

`enum` · `datafusion_common::config::ConfigFileType`

Also reachable as `datafusion::config::ConfigFileType`

```rust
enum ConfigFileType
```

**Variants**: `CSV`, `PARQUET`, `JSON`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigFileType.md).


These file types have special built in behavior for configuration.
Use TableOptions::Extensions for configuring other file types.

---

## Dialect

`enum` · `datafusion_common::config::Dialect`

Also reachable as `datafusion::config::Dialect`

```rust
enum Dialect
```

**Variants**: `Generic`, `MySQL`, `PostgreSQL`, `Hive`, `SQLite`, `Snowflake`, `Redshift`, `MsSQL`, `ClickHouse`, `BigQuery`, `Ansi`, `DuckDB`, `Databricks`, `Spark`

**Implements**: `core::convert::AsRef`, `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn available() -> &'static str
fn metadata() -> &'static [DialectInfo]
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.Dialect.md).


This is the SQL dialect used by DataFusion's parser.
This mirrors [sqlparser::dialect::Dialect](https://docs.rs/sqlparser/latest/sqlparser/dialect/trait.Dialect.html)
trait in order to offer an easier API and avoid adding the `sqlparser` dependency

---

## MapKeyDedupPolicy

`enum` · `datafusion_common::config::MapKeyDedupPolicy`

Also reachable as `datafusion::config::MapKeyDedupPolicy`

```rust
enum MapKeyDedupPolicy
```

**Variants**: `Exception`, `LastWin`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.MapKeyDedupPolicy.md).


Policy for handling duplicate keys in Spark-compatible map-construction
functions (`map_from_arrays`, `map_from_entries`, `str_to_map`). Mirrors
Spark's [`spark.sql.mapKeyDedupPolicy`](https://github.com/apache/spark/blob/cf3a34e19dfcf70e2d679217ff1ba21302212472/sql/catalyst/src/main/scala/org/apache/spark/sql/internal/SQLConf.scala#L4961).

---

## OutputFormat

`enum` · `datafusion_common::config::OutputFormat`

Also reachable as `datafusion::config::OutputFormat`

```rust
enum OutputFormat
```

**Variants**: `CSV`, `JSON`, `PARQUET`, `AVRO`, `ARROW`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.OutputFormat.md).


---

## SpillCompression

`enum` · `datafusion_common::config::SpillCompression`

Also reachable as `datafusion::config::SpillCompression`

```rust
enum SpillCompression
```

**Variants**: `Zstd`, `Lz4Frame`, `Uncompressed`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.SpillCompression.md).


---

## default_config_transform

`function` · `datafusion_common::config::default_config_transform`

Also reachable as `datafusion::config::default_config_transform`

```rust
fn default_config_transform<T>(input: &str) -> Result<T> where T: FromStr, <T as FromStr>::Err: Sync + Send + Error + 'static
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.default_config_transform.md).


Default transformation to parse a [`ConfigField`] for a string.

This uses [`FromStr`] to parse the data.

---

## CatalogOptions

`struct` · `datafusion_common::config::CatalogOptions`

Also reachable as `datafusion::config::CatalogOptions`

```rust
struct CatalogOptions
```

**Fields**: `create_default_catalog_and_schema`, `default_catalog`, `default_schema`, `information_schema`, `location`, `format`, `has_header`, `newlines_in_values`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.CatalogOptions.md).


Options related to catalog and directory scanning

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

---

## ColumnDecryptionProperties

`struct` · `datafusion_common::config::ColumnDecryptionProperties`

Also reachable as `datafusion::config::ColumnDecryptionProperties`

```rust
struct ColumnDecryptionProperties
```

**Fields**: `column_key_as_hex`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ColumnDecryptionProperties.md).


---

## ColumnEncryptionProperties

`struct` · `datafusion_common::config::ColumnEncryptionProperties`

Also reachable as `datafusion::config::ColumnEncryptionProperties`

```rust
struct ColumnEncryptionProperties
```

**Fields**: `column_key_as_hex`, `column_metadata_as_hex`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ColumnEncryptionProperties.md).


---

## ConfigEntry

`struct` · `datafusion_common::config::ConfigEntry`

Also reachable as `datafusion::config::ConfigEntry`

```rust
struct ConfigEntry
```

**Fields**: `key`, `value`, `description`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigEntry.md).


A key value pair, with a corresponding description

---

## ConfigFileDecryptionProperties

`struct` · `datafusion_common::config::ConfigFileDecryptionProperties`

Also reachable as `datafusion::config::ConfigFileDecryptionProperties`, `datafusion_common::encryption::ConfigFileDecryptionProperties`

```rust
struct ConfigFileDecryptionProperties
```

**Fields**: `footer_key_as_hex`, `column_decryption_properties`, `aad_prefix_as_hex`, `footer_signature_verification`

**Implements**: `core::convert::TryFrom`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(f: &Arc<FileDecryptionProperties>) -> Result<Self>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigFileDecryptionProperties.md).


---

## ConfigFileEncryptionProperties

`struct` · `datafusion_common::config::ConfigFileEncryptionProperties`

Also reachable as `datafusion::config::ConfigFileEncryptionProperties`, `datafusion_common::encryption::ConfigFileEncryptionProperties`

```rust
struct ConfigFileEncryptionProperties
```

**Fields**: `encrypt_footer`, `footer_key_as_hex`, `footer_key_metadata_as_hex`, `column_encryption_properties`, `aad_prefix_as_hex`, `store_aad_prefix`

**Implements**: `core::convert::From`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(f: &Arc<FileEncryptionProperties>) -> Self
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigFileEncryptionProperties.md).


---

## ConfigMinTwoUsize

`struct` · `datafusion_common::config::ConfigMinTwoUsize`

Also reachable as `datafusion::config::ConfigMinTwoUsize`

```rust
struct ConfigMinTwoUsize
```

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
const fn get(self) -> usize
fn try_new(value: usize) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> Result<()>
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigMinTwoUsize.md).


A `usize` configuration value that rejects 0 and 1 when set from strings.

Use this for options whose consumer divides the value in half to size an
internal buffer (e.g. a bounded channel capacity): values below 2 would
round down to a zero-capacity buffer and panic. Invalid values return a
configuration error through [`ConfigField`] instead.

---

## ConfigNonZeroUsize

`struct` · `datafusion_common::config::ConfigNonZeroUsize`

Also reachable as `datafusion::config::ConfigNonZeroUsize`

```rust
struct ConfigNonZeroUsize
```

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
const fn get(self) -> usize
fn try_new(value: usize) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> Result<()>
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigNonZeroUsize.md).


A `usize` configuration value that rejects zero when set from strings.

Use this for options where zero is never a meaningful runtime value.
Invalid values return a configuration error through [`ConfigField`].

---

## ConfigOptions

`struct` · `datafusion_common::config::ConfigOptions`

Also reachable as `datafusion::config::ConfigOptions`

```rust
struct ConfigOptions
```

**Fields**: `catalog`, `execution`, `optimizer`, `sql_parser`, `explain`, `extensions`, `format`, `spark`

**Implements**: `core::convert::TryFrom`, `datafusion_common::config::ConfigField`, `datafusion_ffi::config::ExtensionOptionsFFIProvider`

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn entries(&self) -> Vec<ConfigEntry>
fn from_env() -> Result<Self>
fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self>
fn generate_config_markdown() -> String
fn new() -> Self
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn with_extensions(self, extensions: Extensions) -> Self
```

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> Result<()>
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, _key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigOptions.md).


Configuration options struct, able to store both built-in configuration and custom options

---

## CsvOptions

`struct` · `datafusion_common::config::CsvOptions`

Also reachable as `datafusion::config::CsvOptions`

```rust
struct CsvOptions
```

**Fields**: `has_header`, `delimiter`, `quote`, `terminator`, `escape`, `double_quote`, `quote_style`, `ignore_leading_whitespace`, `ignore_trailing_whitespace`, `newlines_in_values`, `compression`, `compression_level`, `schema_infer_max_rec`, `date_format`, `datetime_format`, `timestamp_format`, `timestamp_tz_format`, `time_format`, `null_value`, `null_regex`, `comment`, `truncated_rows`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (20)

```rust
fn delimiter(&self) -> u8
fn escape(&self) -> Option<u8>
fn has_header(&self) -> Option<bool>
fn quote(&self) -> u8
fn terminator(&self) -> Option<u8>
fn with_compression(self, compression_type_variant: CompressionTypeVariant) -> Self
fn with_compression_level(self, level: u32) -> Self
fn with_delimiter(self, delimiter: u8) -> Self
fn with_double_quote(self, double_quote: bool) -> Self
fn with_escape(self, escape: Option<u8>) -> Self
fn with_file_compression_type(self, compression: CompressionTypeVariant) -> Self
fn with_has_header(self, has_header: bool) -> Self
fn with_ignore_leading_whitespace(self, ignore_leading_whitespace: bool) -> Self
fn with_ignore_trailing_whitespace(self, ignore_trailing_whitespace: bool) -> Self
fn with_newlines_in_values(self, newlines_in_values: bool) -> Self
fn with_quote(self, quote: u8) -> Self
fn with_quote_style(self, quote_style: CsvQuoteStyle) -> Self
fn with_schema_infer_max_rec(self, max_rec: usize) -> Self
fn with_terminator(self, terminator: Option<u8>) -> Self
fn with_truncated_rows(self, allow: bool) -> Self
```

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.CsvOptions.md).


Options controlling CSV format

---

## DialectInfo

`struct` · `datafusion_common::config::DialectInfo`

Also reachable as `datafusion::config::DialectInfo`

```rust
struct DialectInfo
```

**Fields**: `dialect`, `canonical_name`, `display_name`, `aliases`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.DialectInfo.md).


Metadata for a SQL dialect supported by DataFusion configuration.

---

## EncryptionFactoryOptions

`struct` · `datafusion_common::config::EncryptionFactoryOptions`

Also reachable as `datafusion::config::EncryptionFactoryOptions`

```rust
struct EncryptionFactoryOptions
```

**Fields**: `options`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn to_extension_options<T: ExtensionOptions + Default>(&self) -> Result<T>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.EncryptionFactoryOptions.md).


Holds implementation-specific options for an encryption factory

---

## ExecutionOptions

`struct` · `datafusion_common::config::ExecutionOptions`

Also reachable as `datafusion::config::ExecutionOptions`

```rust
struct ExecutionOptions
```

**Fields**: `batch_size`, `perfect_hash_join_small_build_threshold`, `perfect_hash_join_min_key_density`, `coalesce_batches`, `collect_statistics`, `target_partitions`, `time_zone`, `parquet`, `planning_concurrency`, `skip_physical_aggregate_schema_check`, `enable_migration_aggregate`, `spill_compression`, `sort_spill_reservation_bytes`, `sort_in_place_threshold_bytes`, `sort_pushdown_buffer_capacity`, `max_spill_file_size_bytes`, `meta_fetch_concurrency`, `minimum_parallel_output_files`, `soft_max_rows_per_output_file`, `max_buffered_batches_per_output_file`, `listing_table_ignore_subdirectory`, `listing_table_factory_infer_partitions`, `enable_recursive_ctes`, `split_file_groups_by_statistics`, `keep_partition_by_columns`, `enable_file_stream_work_stealing`, `skip_partial_aggregation_probe_ratio_threshold`, `skip_partial_aggregation_probe_rows_threshold`, `use_row_number_estimates_to_optimize_partitioning`, `enforce_batch_size_in_joins`, `objectstore_writer_buffer_size`, `enable_ansi_mode`, `hash_join_buffering_capacity`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ExecutionOptions.md).


Options related to query execution

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

---

## ExplainOptions

`struct` · `datafusion_common::config::ExplainOptions`

Also reachable as `datafusion::config::ExplainOptions`

```rust
struct ExplainOptions
```

**Fields**: `logical_plan_only`, `physical_plan_only`, `show_statistics`, `show_sizes`, `show_schema`, `format`, `tree_maximum_render_width`, `analyze_level`, `analyze_categories`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ExplainOptions.md).


Options controlling explain output

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

---

## Extensions

`struct` · `datafusion_common::config::Extensions`

Also reachable as `datafusion::config::Extensions`

```rust
struct Extensions
```

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn get<T: ConfigExtension>(&self) -> Option<&T>
fn get_mut<T: ConfigExtension>(&mut self) -> Option<&mut T>
fn insert<T: ConfigExtension>(&mut self, extension: T)
fn iter(&self) -> impl Iterator<Item = (&'static str, &Box<dyn ExtensionOptions>)>
fn new() -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.Extensions.md).


A type-safe container for [`ConfigExtension`]

---

## FormatOptions

`struct` · `datafusion_common::config::FormatOptions`

Also reachable as `datafusion::config::FormatOptions`

```rust
struct FormatOptions
```

**Fields**: `safe`, `null`, `date_format`, `datetime_format`, `timestamp_format`, `timestamp_tz_format`, `time_format`, `duration_format`, `types_info`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.FormatOptions.md).


Options controlling the format of output when printing record batches
Copies [`arrow::util::display::FormatOptions`]

---

## JsonOptions

`struct` · `datafusion_common::config::JsonOptions`

Also reachable as `datafusion::config::JsonOptions`

```rust
struct JsonOptions
```

**Fields**: `compression`, `compression_level`, `schema_infer_max_rec`, `newline_delimited`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.JsonOptions.md).


Options controlling JSON format

---

## MaxRowGroupBytes

`struct` · `datafusion_common::config::MaxRowGroupBytes`

Also reachable as `datafusion::config::MaxRowGroupBytes`

```rust
struct MaxRowGroupBytes
```

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn get(&self) -> usize
fn try_new(value: usize) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.MaxRowGroupBytes.md).


Target maximum size of a Parquet row group in bytes.

Wraps a `usize` so the "must be greater than zero" constraint (arrow-rs
panics on a zero byte limit) is validated when the config is set, rather
than when the writer properties are built.

---

## OptimizerOptions

`struct` · `datafusion_common::config::OptimizerOptions`

Also reachable as `datafusion::config::OptimizerOptions`

```rust
struct OptimizerOptions
```

**Fields**: `enable_distinct_aggregation_soft_limit`, `enable_round_robin_repartition`, `enable_topk_aggregation`, `enable_window_limits`, `enable_window_topn`, `enable_topk_repartition`, `enable_topk_dynamic_filter_pushdown`, `enable_physical_uncorrelated_scalar_subquery`, `enable_join_dynamic_filter_pushdown`, `enable_aggregate_dynamic_filter_pushdown`, `enable_dynamic_filter_pushdown`, `filter_null_join_keys`, `repartition_aggregations`, `repartition_file_min_size`, `repartition_joins`, `allow_symmetric_joins_without_pruning`, `repartition_file_scans`, `preserve_file_partitions`, `repartition_windows`, `repartition_sorts`, `subset_repartition_threshold`, `prefer_existing_sort`, `skip_failed_rules`, `max_passes`, `top_down_join_key_reordering`, `join_reordering`, `use_statistics_registry`, `prefer_hash_join`, `enable_piecewise_merge_join`, `hash_join_single_partition_threshold`, `hash_join_single_partition_threshold_rows`, `hash_join_inlist_pushdown_max_size`, `hash_join_inlist_pushdown_max_distinct_values`, `default_filter_selectivity`, `prefer_existing_union`, `expand_views_at_output`, `enable_sort_pushdown`, `enable_leaf_expression_pushdown`, `enable_unions_to_filter`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.OptimizerOptions.md).


Options related to query optimization

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

---

## ParquetCdcOptions

`struct` · `datafusion_common::config::ParquetCdcOptions`

Also reachable as `datafusion::config::ParquetCdcOptions`

```rust
struct ParquetCdcOptions
```

**Fields**: `enabled`, `min_chunk_size`, `max_chunk_size`, `norm_level`

**Implements**: `core::convert::From`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn disabled() -> Self
fn enabled() -> Self
```

**via `core::convert::From`**

```rust
fn from(value: Option<&parquet::file::properties::CdcOptions>) -> Self
```

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ParquetCdcOptions.md).


Options for content-defined chunking (CDC) when writing parquet files.
Mirrors `parquet::file::properties::CdcOptions`.

Carried as a [`ParquetCdcOptions`] in [`ParquetOptions::content_defined_chunking`]
with an explicit `enabled` flag, so it can be toggled with dotted config
keys (`content_defined_chunking.enabled = true|false`) and the result is
independent of the order in which the keys are set.

---

## ParquetColumnOptions

`struct` · `datafusion_common::config::ParquetColumnOptions`

Also reachable as `datafusion::config::ParquetColumnOptions`

```rust
struct ParquetColumnOptions
```

**Fields**: `bloom_filter_enabled`, `encoding`, `dictionary_enabled`, `compression`, `statistics_enabled`, `bloom_filter_fpp`, `bloom_filter_ndv`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ParquetColumnOptions.md).


Options controlling parquet format for individual columns.

See [`ParquetOptions`] for more details

---

## ParquetEncryptionOptions

`struct` · `datafusion_common::config::ParquetEncryptionOptions`

Also reachable as `datafusion::config::ParquetEncryptionOptions`

```rust
struct ParquetEncryptionOptions
```

**Fields**: `file_decryption`, `file_encryption`, `factory_id`, `factory_options`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn configure_factory(&mut self, factory_id: &str, config: &impl ExtensionOptions)
```

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ParquetEncryptionOptions.md).


Options for configuring Parquet Modular Encryption

To use Parquet encryption, you must enable the `parquet_encryption` feature flag, as it is not activated by default.

---

## ParquetOptions

`struct` · `datafusion_common::config::ParquetOptions`

Also reachable as `datafusion::config::ParquetOptions`

```rust
struct ParquetOptions
```

**Fields**: `enable_page_index`, `pruning`, `skip_metadata`, `metadata_size_hint`, `pushdown_filters`, `reorder_filters`, `force_filter_selections`, `schema_force_view_types`, `binary_as_string`, `coerce_int96`, `coerce_int96_tz`, `bloom_filter_on_read`, `max_predicate_cache_size`, `max_in_list_size`, `data_pagesize_limit`, `write_batch_size`, `writer_version`, `skip_arrow_metadata`, `compression`, `dictionary_enabled`, `dictionary_page_size_limit`, `statistics_enabled`, `max_row_group_size`, `max_row_group_bytes`, `created_by`, `column_index_truncate_length`, `statistics_truncate_length`, `data_page_row_count_limit`, `encoding`, `bloom_filter_on_write`, `bloom_filter_fpp`, `bloom_filter_ndv`, `allow_single_file_parallelism`, `maximum_parallel_row_group_writers`, `maximum_buffered_record_batches_per_stream`, `content_defined_chunking`

**Implements**: `core::convert::TryFrom`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn into_writer_properties_builder(&self) -> Result<WriterPropertiesBuilder>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ParquetOptions.md).


Options for reading and writing parquet files

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

---

## SparkOptions

`struct` · `datafusion_common::config::SparkOptions`

Also reachable as `datafusion::config::SparkOptions`

```rust
struct SparkOptions
```

**Fields**: `map_key_dedup_policy`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.SparkOptions.md).


Options controlling DataFusion's Spark-compatibility layer (functions
under `datafusion/spark`). Keys here mirror their `spark.sql.*`
equivalents in Apache Spark.

---

## SqlParserOptions

`struct` · `datafusion_common::config::SqlParserOptions`

Also reachable as `datafusion::config::SqlParserOptions`

```rust
struct SqlParserOptions
```

**Fields**: `parse_float_as_decimal`, `enable_ident_normalization`, `enable_options_value_normalization`, `dialect`, `support_varchar_with_length`, `map_string_types_to_utf8view`, `collect_spans`, `recursion_limit`, `default_null_ordering`, `enable_subquery_sort_elimination`

**Implements**: `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.SqlParserOptions.md).


Options related to SQL parser

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

---

## TableOptions

`struct` · `datafusion_common::config::TableOptions`

Also reachable as `datafusion::config::TableOptions`

```rust
struct TableOptions
```

**Fields**: `csv`, `parquet`, `json`, `current_format`, `extensions`

**Implements**: `core::convert::TryFrom`, `datafusion_common::config::ConfigField`, `datafusion_ffi::config::ExtensionOptionsFFIProvider`

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn alter_with_string_hash_map(&mut self, settings: &HashMap<String, String>) -> Result<()>
fn combine_with_session_config(&self, config: &ConfigOptions) -> Self
fn default_from_session_config(config: &ConfigOptions) -> Self
fn entries(&self) -> Vec<ConfigEntry>
fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self>
fn new() -> Self
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn set_config_format(&mut self, format: ConfigFileType)
fn with_extensions(self, extensions: Extensions) -> Self
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, _key_prefix: &str, _description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.TableOptions.md).


Represents the configuration options available for handling different table formats within a data processing application.
This struct encompasses options for various file formats including CSV, Parquet, and JSON, allowing for flexible configuration
of parsing and writing behaviors specific to each format. Additionally, it supports extending functionality through custom extensions.

---

## TableParquetOptions

`struct` · `datafusion_common::config::TableParquetOptions`

Also reachable as `datafusion::config::TableParquetOptions`

```rust
struct TableParquetOptions
```

**Fields**: `global`, `column_specific_options`, `key_value_metadata`, `crypto`

**Implements**: `core::convert::TryFrom`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn arrow_schema(&mut self, schema: &Arc<Schema>)
fn entries(&TableParquetOptions) -> Vec<ConfigEntry>
fn new() -> Self
fn with_skip_arrow_metadata(self, skip: bool) -> Self
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.TableParquetOptions.md).


Options that control how Parquet files are read, including global options
that apply to all columns and optional column-specific overrides

Closely tied to `ParquetWriterOptions` (see `crate::file_options::parquet_writer::ParquetWriterOptions` when the "parquet" feature is enabled).
Properties not included in [`TableParquetOptions`] may not be configurable at the external API
(e.g. sorting_columns).

---

## ConfigExtension

`trait` · `datafusion_common::config::ConfigExtension`

Also reachable as `datafusion::config::ConfigExtension`

```rust
trait ConfigExtension: ExtensionOptions
```

**Implementors** (2)

- `datafusion_ffi::config::extension_options::FFI_ExtensionOptions`
- `datafusion_ffi::tests::config::ExternalConfig`

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigExtension.md).


[`ConfigExtension`] provides a mechanism to store third-party configuration
within DataFusion [`ConfigOptions`]

This mechanism can be used to pass configuration to user defined functions
or optimizer passes

# Example
```
use datafusion_common::{
    config::ConfigExtension, config::ConfigOptions, extensions_options,
};
// Define a new configuration struct using the `extensions_options` macro
extensions_options! {
   /// My own config options.
   pub struct MyConfig {
       /// Should "foo" be replaced by "bar"?
       pub foo_to_bar: bool, default = true

       /// How many "baz" should be created?
       pub baz_count: usize, default = 1337
   }
}

impl ConfigExtension for MyConfig {
    const PREFIX: &'static str = "my_config";
}

// set up config struct and register extension
let mut config = ConfigOptions::default();
config.extensions.insert(MyConfig::default());

// overwrite config default
config.set("my_config.baz_count", "42").unwrap();

// check config state
let my_config = config.extensions.get::<MyConfig>().unwrap();
assert!(my_config.foo_to_bar,);
assert_eq!(my_config.baz_count, 42,);
```

# Note:
Unfortunately associated constants are not currently object-safe, and so this
extends the object-safe [`ExtensionOptions`]

---

## ConfigField

`trait` · `datafusion_common::config::ConfigField`

Also reachable as `datafusion::config::ConfigField`

```rust
trait ConfigField
```

**Implementors** (36)

- `alloc::string::String`
- `core::option::Option`
- `datafusion_common::config::CatalogOptions`
- `datafusion_common::config::ColumnDecryptionProperties`
- `datafusion_common::config::ColumnEncryptionProperties`
- `datafusion_common::config::ConfigFileDecryptionProperties`
- `datafusion_common::config::ConfigFileEncryptionProperties`
- `datafusion_common::config::ConfigMinTwoUsize`
- `datafusion_common::config::ConfigNonZeroUsize`
- `datafusion_common::config::ConfigOptions`
- `datafusion_common::config::CsvOptions`
- `datafusion_common::config::Dialect`
- `datafusion_common::config::EncryptionFactoryOptions`
- `datafusion_common::config::ExecutionOptions`
- `datafusion_common::config::ExplainOptions`
- `datafusion_common::config::FormatOptions`
- `datafusion_common::config::JsonOptions`
- `datafusion_common::config::MapKeyDedupPolicy`
- `datafusion_common::config::OptimizerOptions`
- `datafusion_common::config::ParquetCdcOptions`
- `datafusion_common::config::ParquetColumnOptions`
- `datafusion_common::config::ParquetEncryptionOptions`
- `datafusion_common::config::ParquetOptions`
- `datafusion_common::config::SparkOptions`
- `datafusion_common::config::SpillCompression`
- `datafusion_common::config::SqlParserOptions`
- `datafusion_common::config::TableOptions`
- `datafusion_common::config::TableParquetOptions`
- `datafusion_common::format::ExplainAnalyzeCategories`
- `datafusion_common::format::ExplainFormat`
- `datafusion_common::format::MetricType`
- `datafusion_common::parquet_config::DFParquetWriterVersion`
- `datafusion_common::parsers::CompressionTypeVariant`
- `datafusion_common::parsers::CsvQuoteStyle`
- `datafusion_ffi::tests::config::ExternalConfig`
- `std::collections::hash::map::HashMap`

**Methods** (3)

```rust
fn reset(&mut self, key: &str) -> Result<()>
fn set(&mut self, key: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ConfigField.md).


A trait implemented by `config_namespace` and for field types that provides
the ability to walk and mutate the configuration tree

---

## ExtensionOptions

`trait` · `datafusion_common::config::ExtensionOptions`

Also reachable as `datafusion::config::ExtensionOptions`

```rust
trait ExtensionOptions: Send + Sync + fmt::Debug + 'static
```

**Implementors** (2)

- `datafusion_ffi::config::extension_options::FFI_ExtensionOptions`
- `datafusion_ffi::tests::config::ExternalConfig`

**Methods** (5)

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn cloned(&self) -> Box<dyn ExtensionOptions>
fn entries(&self) -> Vec<ConfigEntry>
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.ExtensionOptions.md).


An object-safe API for storing arbitrary configuration.

See [`ConfigExtension`] for user defined configuration

---

## OutputFormatExt

`trait` · `datafusion_common::config::OutputFormatExt`

Also reachable as `datafusion::config::OutputFormatExt`

```rust
trait OutputFormatExt: Display
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.OutputFormatExt.md).


---

## Visit

`trait` · `datafusion_common::config::Visit`

Also reachable as `datafusion::config::Visit`

```rust
trait Visit
```

**Methods** (2)

```rust
fn none(&mut self, key: &str, description: &'static str)
fn some<V: Display>(&mut self, key: &str, value: V, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.config.Visit.md).


An implementation trait used to recursively walk configuration

---
