# `datafusion_execution::config`

Crate `datafusion-execution` · 1 public items · structured records in [`model/datafusion_execution.config.json`](../model/datafusion_execution.config.json)

## SessionConfig

`struct` · `datafusion_execution::config::SessionConfig`

Also reachable as `datafusion::execution::context::SessionConfig`, `datafusion::prelude::SessionConfig`

```rust
struct SessionConfig
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Debug, Default

**Methods** (59)

```rust
fn batch_size(&self) -> usize
fn coalesce_batches(&self) -> bool
fn collect_statistics(&self) -> bool
fn create_default_catalog_and_schema(&self) -> bool
fn enforce_batch_size_in_joins(&self) -> bool
fn from_env() -> Result<Self>
fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self>
fn get_extension<T>(&self) -> Option<Arc<T>> where T: Send + Sync + 'static
fn information_schema(&self) -> bool
fn new() -> Self
fn options(&self) -> &Arc<ConfigOptions>
fn options_mut(&mut self) -> &mut ConfigOptions
fn parquet_bloom_filter_pruning(&self) -> bool
fn parquet_page_index_pruning(&self) -> bool
fn parquet_pruning(&self) -> bool
fn prefer_existing_sort(&self) -> bool
fn repartition_aggregations(&self) -> bool
fn repartition_joins(&self) -> bool
fn repartition_sorts(&self) -> bool
fn repartition_window_functions(&self) -> bool
fn round_robin_repartition(&self) -> bool
fn set(self, key: &str, value: &ScalarValue) -> Self
fn set_bool(self, key: &str, value: bool) -> Self
fn set_extension<T>(&mut self, ext: Arc<T>) where T: Send + Sync + 'static
fn set_str(self, key: &str, value: &str) -> Self
fn set_u64(self, key: &str, value: u64) -> Self
fn set_usize(self, key: &str, value: usize) -> Self
fn spill_compression(&self) -> SpillCompression
fn target_partitions(&self) -> usize
fn to_props(&self) -> HashMap<String, String>
fn with_allow_symmetric_joins_without_pruning(self, enabled: bool) -> Self
fn with_batch_size(self, n: usize) -> Self
fn with_coalesce_batches(self, enabled: bool) -> Self
fn with_collect_statistics(self, enabled: bool) -> Self
fn with_create_default_catalog_and_schema(self, create: bool) -> Self
fn with_default_catalog_and_schema(self, catalog: impl Into<String>, schema: impl Into<String>) -> Self
fn with_enable_ansi_mode(self, enable_ansi_mode: bool) -> Self
fn with_enable_sort_pushdown(self, enabled: bool) -> Self
fn with_enable_subquery_sort_elimination(self, enabled: bool) -> Self
fn with_enforce_batch_size_in_joins(self, enforce_batch_size_in_joins: bool) -> Self
fn with_extension<T>(self, ext: Arc<T>) -> Self where T: Send + Sync + 'static
fn with_information_schema(self, enabled: bool) -> Self
fn with_option_extension<T: ConfigExtension>(self, extension: T) -> Self
fn with_parquet_bloom_filter_pruning(self, enabled: bool) -> Self
fn with_parquet_page_index_pruning(self, enabled: bool) -> Self
fn with_parquet_pruning(self, enabled: bool) -> Self
fn with_prefer_existing_sort(self, enabled: bool) -> Self
fn with_prefer_existing_union(self, enabled: bool) -> Self
fn with_repartition_aggregations(self, enabled: bool) -> Self
fn with_repartition_file_min_size(self, size: usize) -> Self
fn with_repartition_file_scans(self, enabled: bool) -> Self
fn with_repartition_joins(self, enabled: bool) -> Self
fn with_repartition_sorts(self, enabled: bool) -> Self
fn with_repartition_windows(self, enabled: bool) -> Self
fn with_round_robin_repartition(self, enabled: bool) -> Self
fn with_sort_in_place_threshold_bytes(self, sort_in_place_threshold_bytes: usize) -> Self
fn with_sort_spill_reservation_bytes(self, sort_spill_reservation_bytes: usize) -> Self
fn with_spill_compression(self, spill_compression: SpillCompression) -> Self
fn with_target_partitions(self, n: usize) -> Self
```

**via `core::convert::From`**

```rust
fn from(options: ConfigOptions) -> Self
```

Configuration options for [`SessionContext`].

Can be passed to [`SessionContext::new_with_config`] to customize the configuration of DataFusion.

Options can be set using namespaces keys with `.` as the separator, where the
namespace determines which configuration struct the value to routed to. All
built-in options are under the `datafusion` namespace.

For example, the key `datafusion.execution.batch_size` will set [ExecutionOptions::batch_size][datafusion_common::config::ExecutionOptions::batch_size],
because [ConfigOptions::execution] is [ExecutionOptions][datafusion_common::config::ExecutionOptions]. Similarly, the key
`datafusion.execution.parquet.pushdown_filters` will set [ParquetOptions::pushdown_filters][datafusion_common::config::ParquetOptions::pushdown_filters],
since [ExecutionOptions::parquet][datafusion_common::config::ExecutionOptions::parquet] is [ParquetOptions][datafusion_common::config::ParquetOptions].

Some options have convenience methods. For example [SessionConfig::with_batch_size] is
shorthand for setting `datafusion.execution.batch_size`.

```
use datafusion_common::ScalarValue;
use datafusion_execution::config::SessionConfig;

let config = SessionConfig::new()
    .set(
        "datafusion.execution.batch_size",
        &ScalarValue::UInt64(Some(1234)),
    )
    .set_bool("datafusion.execution.parquet.pushdown_filters", true);

assert_eq!(config.batch_size(), 1234);
assert_eq!(config.options().execution.batch_size.get(), 1234);
assert_eq!(config.options().execution.parquet.pushdown_filters, true);
```

You can also directly mutate the options via [SessionConfig::options_mut].
So the following is equivalent to the above:

```
# use datafusion_execution::config::SessionConfig;
# use datafusion_common::config::ConfigNonZeroUsize;
#
let mut config = SessionConfig::new();
config.options_mut().execution.batch_size = ConfigNonZeroUsize::try_new(1234)?;
config.options_mut().execution.parquet.pushdown_filters = true;
#
# assert_eq!(config.batch_size(), 1234);
# assert_eq!(config.options().execution.batch_size.get(), 1234);
# assert_eq!(config.options().execution.parquet.pushdown_filters, true);
# datafusion_common::Result::<()>::Ok(())
```

## Built-in options

| Namespace | Config struct |
| --------- | ------------- |
| `datafusion.catalog` | [CatalogOptions][datafusion_common::config::CatalogOptions] |
| `datafusion.execution` | [ExecutionOptions][datafusion_common::config::ExecutionOptions] |
| `datafusion.execution.parquet` | [ParquetOptions][datafusion_common::config::ParquetOptions] |
| `datafusion.optimizer` | [OptimizerOptions][datafusion_common::config::OptimizerOptions] |
| `datafusion.sql_parser` | [SqlParserOptions][datafusion_common::config::SqlParserOptions] |
| `datafusion.explain` | [ExplainOptions][datafusion_common::config::ExplainOptions] |

## Custom configuration

Configuration options can be extended. See [SessionConfig::with_extension] for details.

[`SessionContext`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html
[`SessionContext::new_with_config`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.new_with_config

---
