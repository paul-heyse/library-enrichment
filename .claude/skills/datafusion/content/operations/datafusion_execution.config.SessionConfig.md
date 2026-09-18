# `datafusion_execution::config::SessionConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.config.SessionConfig.json).

<a id="op-5db676088685c6e8b0496c08"></a>
## SessionConfig

`struct` · `datafusion_execution::config::SessionConfig` · datafusion-execution 55.1.0

```rust
struct SessionConfig
```

Source: `src/config.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Configuration options for [`SessionContext`].

Can be passed to [`SessionContext::new_with_config`] to customize the configuration of DataFusion.

Options can be set using namespaces keys with `.` as the separator, where the
namespace determines which configuration struct the value to routed to. All
built-in options are under the `datafusion` namespace.

For example, the key `datafusion.execution.batch_size` will set [ExecutionOptions::batch_size][datafusion_common::config::ExecutionOptions::batch_size](../operations/datafusion_common.config.ExecutionOptions.md#op-c25412ecbe58e58a4d83f3dd),
because [ConfigOptions::execution](../operations/datafusion_common.config.ConfigOptions.md#op-1a65b541b76cc5e25160a8a5) is [ExecutionOptions][datafusion_common::config::ExecutionOptions](../operations/datafusion_common.config.ExecutionOptions.md#op-15fab70a83be6ab4e79c51a4). Similarly, the key
`datafusion.execution.parquet.pushdown_filters` will set [ParquetOptions::pushdown_filters][datafusion_common::config::ParquetOptions::pushdown_filters](../operations/datafusion_common.config.ParquetOptions.md#op-7e2a1457b357de2df058a8da),
since [ExecutionOptions::parquet][datafusion_common::config::ExecutionOptions::parquet](../operations/datafusion_common.config.ExecutionOptions.md#op-6a4fde5c8353b19e8acd5f0c) is [ParquetOptions][datafusion_common::config::ParquetOptions](../operations/datafusion_common.config.ParquetOptions.md#op-434fa79e897be23d649f954c).

Some options have convenience methods. For example [SessionConfig::with_batch_size](../operations/datafusion_execution.config.SessionConfig.md#op-dd4f73e79a215db8cc5a8d29) is
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

You can also directly mutate the options via [SessionConfig::options_mut](../operations/datafusion_execution.config.SessionConfig.md#op-25f9a424ad10574715842ed6).
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
| `datafusion.catalog` | [CatalogOptions][datafusion_common::config::CatalogOptions](../operations/datafusion_common.config.CatalogOptions.md#op-cc8848630c802be1a4995eea) |
| `datafusion.execution` | [ExecutionOptions][datafusion_common::config::ExecutionOptions](../operations/datafusion_common.config.ExecutionOptions.md#op-15fab70a83be6ab4e79c51a4) |
| `datafusion.execution.parquet` | [ParquetOptions][datafusion_common::config::ParquetOptions](../operations/datafusion_common.config.ParquetOptions.md#op-434fa79e897be23d649f954c) |
| `datafusion.optimizer` | [OptimizerOptions][datafusion_common::config::OptimizerOptions](../operations/datafusion_common.config.OptimizerOptions.md#op-c1bca17ddd60fbb0925d4514) |
| `datafusion.sql_parser` | [SqlParserOptions][datafusion_common::config::SqlParserOptions](../operations/datafusion_common.config.SqlParserOptions.md#op-9136dabd1b661c04a50818cc) |
| `datafusion.explain` | [ExplainOptions][datafusion_common::config::ExplainOptions](../operations/datafusion_common.config.ExplainOptions.md#op-2ae1c989543f41b4e318de4d) |

## Custom configuration

Configuration options can be extended. See [SessionConfig::with_extension](../operations/datafusion_execution.config.SessionConfig.md#op-2b83eb4d3ae610e008f7b9f0) for details.

[`SessionContext`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html
[`SessionContext::new_with_config`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.new_with_config

<a id="op-85ea5598384eee56519a6862"></a>
## batch_size

`function` · `datafusion_execution::config::SessionConfig::batch_size` · datafusion-execution 55.1.0

```rust
fn batch_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get the currently configured batch size

<a id="op-bbfb27e46202a7e53733f3d0"></a>
## clone

`function` · `datafusion_execution::config::SessionConfig::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> SessionConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 10], "end": [92, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-397095a87912adb63313b997"></a>
## coalesce_batches

`function` · `datafusion_execution::config::SessionConfig::coalesce_batches` · datafusion-execution 55.1.0

```rust
fn coalesce_batches(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns true if record batches will be examined between each operator
and small batches will be coalesced into larger batches.

<a id="op-40b373b8d45a3402e686ae0f"></a>
## collect_statistics

`function` · `datafusion_execution::config::SessionConfig::collect_statistics` · datafusion-execution 55.1.0

```rust
fn collect_statistics(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Are statistics collected during execution?

<a id="op-c6b2643ccd864e6932fdf367"></a>
## create_default_catalog_and_schema

`function` · `datafusion_execution::config::SessionConfig::create_default_catalog_and_schema` · datafusion-execution 55.1.0

```rust
fn create_default_catalog_and_schema(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Should the context create the default catalog and schema?

<a id="op-916ed20b25ef32195635bf1a"></a>
## default

`function` · `datafusion_execution::config::SessionConfig::default` · datafusion-execution 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [112, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a82b906a003e96e81bd472a8"></a>
## enforce_batch_size_in_joins

`function` · `datafusion_execution::config::SessionConfig::enforce_batch_size_in_joins` · datafusion-execution 55.1.0

```rust
fn enforce_batch_size_in_joins(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns true if the joins will be enforced to output batches of the configured size

<a id="op-e8f12e9599373c0e07625dbf"></a>
## fmt

`function` · `datafusion_execution::config::SessionConfig::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 17], "end": [92, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a70e383f6da3e0066aea7712"></a>
## from

`function` · `datafusion_execution::config::SessionConfig::from` · datafusion-execution 55.1.0

```rust
fn from(options: ConfigOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [615, 1], "end": [623, 2], "filename": "src/config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/config.rs:616`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a85eeda5faef1963693d6aef"></a>
## from_env

`function` · `datafusion_execution::config::SessionConfig::from_env` · datafusion-execution 55.1.0

```rust
fn from_env() -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create an execution config with config options read from the environment

See [`ConfigOptions::from_env`] for details on how environment variables
are mapped to config options.

Unresolved upstream links (retained, not inferred): ``ConfigOptions::from_env``.

<a id="op-f7447488d79b4f348bbf8cfd"></a>
## from_string_hash_map

`function` · `datafusion_execution::config::SessionConfig::from_string_hash_map` · datafusion-execution 55.1.0

```rust
fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create new ConfigOptions struct, taking values from a string hash map.

<a id="op-fdf4014d9856366f443125bf"></a>
## get_extension

`function` · `datafusion_execution::config::SessionConfig::get_extension` · datafusion-execution 55.1.0

```rust
fn get_extension<T>(&self) -> Option<Arc<T>> where T: Send + Sync + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:607`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get extension, if any for the specified type `T` exists.

See [`with_extension`](Self::with_extension) on how to add attach extensions.

<a id="op-63d4b3894b12340f67d1e73a"></a>
## information_schema

`function` · `datafusion_execution::config::SessionConfig::information_schema` · datafusion-execution 55.1.0

```rust
fn information_schema(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Is the information schema enabled?

<a id="op-faa997fe524e378614ad51b6"></a>
## new

`function` · `datafusion_execution::config::SessionConfig::new` · datafusion-execution 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create an execution config with default setting

<a id="op-31bd2ad63c30cb041ab40a88"></a>
## options

`function` · `datafusion_execution::config::SessionConfig::options` · datafusion-execution 55.1.0

```rust
fn options(&self) -> &Arc<ConfigOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return a handle to the configuration options.

Can be used to read the current configuration.

```
use datafusion_execution::config::SessionConfig;

let config = SessionConfig::new();
assert!(config.options().execution.batch_size.get() > 0);
```

<a id="op-25f9a424ad10574715842ed6"></a>
## options_mut

`function` · `datafusion_execution::config::SessionConfig::options_mut` · datafusion-execution 55.1.0

```rust
fn options_mut(&mut self) -> &mut ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return a mutable handle to the configuration options.

Can be used to set configuration options.

```
use datafusion_common::config::ConfigNonZeroUsize;
use datafusion_execution::config::SessionConfig;

let mut config = SessionConfig::new();
config.options_mut().execution.batch_size = ConfigNonZeroUsize::try_new(1024)?;
assert_eq!(config.options().execution.batch_size.get(), 1024);
# datafusion_common::Result::<()>::Ok(())
```

<a id="op-07408a101f5b5b5012b76d50"></a>
## parquet_bloom_filter_pruning

`function` · `datafusion_execution::config::SessionConfig::parquet_bloom_filter_pruning` · datafusion-execution 55.1.0

```rust
fn parquet_bloom_filter_pruning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns true if bloom filter should be used to skip parquet row groups

<a id="op-79367bf89aefe449b143c796"></a>
## parquet_page_index_pruning

`function` · `datafusion_execution::config::SessionConfig::parquet_page_index_pruning` · datafusion-execution 55.1.0

```rust
fn parquet_page_index_pruning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:378`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns true if page index should be used to skip parquet data pages

<a id="op-b825074f0ebe70d91fc9deeb"></a>
## parquet_pruning

`function` · `datafusion_execution::config::SessionConfig::parquet_pruning` · datafusion-execution 55.1.0

```rust
fn parquet_pruning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns true if pruning predicate should be used to skip parquet row groups

<a id="op-519c429be6a372c585ec7599"></a>
## prefer_existing_sort

`function` · `datafusion_execution::config::SessionConfig::prefer_existing_sort` · datafusion-execution 55.1.0

```rust
fn prefer_existing_sort(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Prefer existing sort (true) or maximize parallelism (false). See
[prefer_existing_sort] for more details

[prefer_existing_sort]: datafusion_common::config::OptimizerOptions::prefer_existing_sort

<a id="op-eb7ad3940f5100b9b61044c8"></a>
## repartition_aggregations

`function` · `datafusion_execution::config::SessionConfig::repartition_aggregations` · datafusion-execution 55.1.0

```rust
fn repartition_aggregations(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Are aggregates repartitioned during execution?

<a id="op-6f5d28ff05fa7e4c860bdde9"></a>
## repartition_joins

`function` · `datafusion_execution::config::SessionConfig::repartition_joins` · datafusion-execution 55.1.0

```rust
fn repartition_joins(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Are joins repartitioned during execution?

<a id="op-98f38f30e89fe4b4e2420ccd"></a>
## repartition_sorts

`function` · `datafusion_execution::config::SessionConfig::repartition_sorts` · datafusion-execution 55.1.0

```rust
fn repartition_sorts(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Do we execute sorts in a per-partition fashion and merge afterwards,
or do we coalesce partitions first and sort globally?

<a id="op-be5d179adb474a6268280a7b"></a>
## repartition_window_functions

`function` · `datafusion_execution::config::SessionConfig::repartition_window_functions` · datafusion-execution 55.1.0

```rust
fn repartition_window_functions(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Are window functions repartitioned during execution?

<a id="op-4dd563a237cb91b48d9cca33"></a>
## round_robin_repartition

`function` · `datafusion_execution::config::SessionConfig::round_robin_repartition` · datafusion-execution 55.1.0

```rust
fn round_robin_repartition(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns true if the physical plan optimizer will try to
add round robin repartition to increase parallelism to leverage more CPU cores.

<a id="op-53bff127d4d312a11cc97e4e"></a>
## set

`function` · `datafusion_execution::config::SessionConfig::set` · datafusion-execution 55.1.0

```rust
fn set(self, key: &str, value: &ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set a configuration option

<a id="op-6c0f5f4054240c82d97a456a"></a>
## set_bool

`function` · `datafusion_execution::config::SessionConfig::set_bool` · datafusion-execution 55.1.0

```rust
fn set_bool(self, key: &str, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set a boolean configuration option

<a id="op-f42c0bee58aaff8d4dd422ab"></a>
## set_extension

`function` · `datafusion_execution::config::SessionConfig::set_extension` · datafusion-execution 55.1.0

```rust
fn set_extension<T>(&mut self, ext: Arc<T>) where T: Send + Sync + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:597`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set extension. Pretty much the same as [`with_extension`](Self::with_extension), but take
mutable reference instead of owning it. Useful if you want to add another extension after
the [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08) is created.

# Example
```
use datafusion_execution::config::SessionConfig;
use std::sync::Arc;

// application-specific extension types
struct Ext1(u8);
struct Ext2(u8);
struct Ext3(u8);

let ext1a = Arc::new(Ext1(10));
let ext1b = Arc::new(Ext1(11));
let ext2 = Arc::new(Ext2(2));

let mut cfg = SessionConfig::default();

// will only remember the last Ext1
cfg.set_extension(Arc::clone(&ext1a));
cfg.set_extension(Arc::clone(&ext1b));
cfg.set_extension(Arc::clone(&ext2));

let ext1_received = cfg.get_extension::<Ext1>().unwrap();
assert!(!Arc::ptr_eq(&ext1_received, &ext1a));
assert!(Arc::ptr_eq(&ext1_received, &ext1b));

let ext2_received = cfg.get_extension::<Ext2>().unwrap();
assert!(Arc::ptr_eq(&ext2_received, &ext2));

assert!(cfg.get_extension::<Ext3>().is_none());
```

<a id="op-8982599eace022c069349d6b"></a>
## set_str

`function` · `datafusion_execution::config::SessionConfig::set_str` · datafusion-execution 55.1.0

```rust
fn set_str(self, key: &str, value: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set a generic `str` configuration option

<a id="op-2ae65732c6e81fc86658f226"></a>
## set_u64

`function` · `datafusion_execution::config::SessionConfig::set_u64` · datafusion-execution 55.1.0

```rust
fn set_u64(self, key: &str, value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set a generic `u64` configuration option

<a id="op-c21988cd04c2f2de37104f67"></a>
## set_usize

`function` · `datafusion_execution::config::SessionConfig::set_usize` · datafusion-execution 55.1.0

```rust
fn set_usize(self, key: &str, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set a generic `usize` configuration option

<a id="op-7325a391db73f6f30c8eb784"></a>
## spill_compression

`function` · `datafusion_execution::config::SessionConfig::spill_compression` · datafusion-execution 55.1.0

```rust
fn spill_compression(&self) -> SpillCompression
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Compression codec for spill file

<a id="op-e39e4c42904d882ea3f46e53"></a>
## target_partitions

`function` · `datafusion_execution::config::SessionConfig::target_partitions` · datafusion-execution 55.1.0

```rust
fn target_partitions(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get [`target_partitions`]

[`target_partitions`]: datafusion_common::config::ExecutionOptions::target_partitions

<a id="op-16f1254c29f319c6918a2c98"></a>
## to_props

`function` · `datafusion_execution::config::SessionConfig::to_props` · datafusion-execution 55.1.0

```rust
fn to_props(&self) -> HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Convert configuration options to name-value pairs with values
converted to strings.

Note that this method will eventually be deprecated and
replaced by [`options`].

[`options`]: Self::options

<a id="op-a13180b6ef31ceb1dadf4a6d"></a>
## with_allow_symmetric_joins_without_pruning

`function` · `datafusion_execution::config::SessionConfig::with_allow_symmetric_joins_without_pruning` · datafusion-execution 55.1.0

```rust
fn with_allow_symmetric_joins_without_pruning(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the allowing unordered symmetric hash join

<a id="op-dd4f73e79a215db8cc5a8d29"></a>
## with_batch_size

`function` · `datafusion_execution::config::SessionConfig::with_batch_size` · datafusion-execution 55.1.0

```rust
fn with_batch_size(self, n: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Customize batch size

<a id="op-b24446f1693173d27eaec6d2"></a>
## with_coalesce_batches

`function` · `datafusion_execution::config::SessionConfig::with_coalesce_batches` · datafusion-execution 55.1.0

```rust
fn with_coalesce_batches(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the coalescence of small batches into larger batches

<a id="op-fffcc6a2e04e1a5344fba62f"></a>
## with_collect_statistics

`function` · `datafusion_execution::config::SessionConfig::with_collect_statistics` · datafusion-execution 55.1.0

```rust
fn with_collect_statistics(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the collection of statistics after listing files

<a id="op-d696b3c2d29dfeb3c6b9c08a"></a>
## with_create_default_catalog_and_schema

`function` · `datafusion_execution::config::SessionConfig::with_create_default_catalog_and_schema` · datafusion-execution 55.1.0

```rust
fn with_create_default_catalog_and_schema(self, create: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Controls whether the default catalog and schema will be automatically created

<a id="op-0a32a4151ba79d466d800aed"></a>
## with_default_catalog_and_schema

`function` · `datafusion_execution::config::SessionConfig::with_default_catalog_and_schema` · datafusion-execution 55.1.0

```rust
fn with_default_catalog_and_schema(self, catalog: impl Into<String>, schema: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Selects a name for the default catalog and schema

<a id="op-e36fe2ed63f5945effb56573"></a>
## with_enable_ansi_mode

`function` · `datafusion_execution::config::SessionConfig::with_enable_ansi_mode` · datafusion-execution 55.1.0

```rust
fn with_enable_ansi_mode(self, enable_ansi_mode: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:489`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Toggle SQL ANSI mode for expressions, casting, and error handling

<a id="op-b2fecef9f2629bc99694d5af"></a>
## with_enable_sort_pushdown

`function` · `datafusion_execution::config::SessionConfig::with_enable_sort_pushdown` · datafusion-execution 55.1.0

```rust
fn with_enable_sort_pushdown(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:425`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables sort pushdown optimization, and currently only
applies to Parquet data source.

<a id="op-71b329402e18c34ec3814672"></a>
## with_enable_subquery_sort_elimination

`function` · `datafusion_execution::config::SessionConfig::with_enable_subquery_sort_elimination` · datafusion-execution 55.1.0

```rust
fn with_enable_subquery_sort_elimination(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables elimination of `ORDER BY` clauses in subqueries
when they are not required by order-sensitive operators.

<a id="op-d218abc45d0a21b0e9ef6e53"></a>
## with_enforce_batch_size_in_joins

`function` · `datafusion_execution::config::SessionConfig::with_enforce_batch_size_in_joins` · datafusion-execution 55.1.0

```rust
fn with_enforce_batch_size_in_joins(self, enforce_batch_size_in_joins: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:474`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the enforcement of batch size in joins

<a id="op-2b83eb4d3ae610e008f7b9f0"></a>
## with_extension

`function` · `datafusion_execution::config::SessionConfig::with_extension` · datafusion-execution 55.1.0

```rust
fn with_extension<T>(self, ext: Arc<T>) -> Self where T: Send + Sync + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:555`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Add extensions.

Extensions can be used to attach extra data to the session config -- e.g. tracing information or caches.
Extensions are opaque and the types are unknown to DataFusion itself, which makes them extremely flexible. [^1]

Extensions are stored within an [`Arc`] so they do NOT require [`Clone`]. The are immutable. If you need to
modify their state over their lifetime -- e.g. for caches -- you need to establish some form of interior mutability.

Extensions are indexed by their type `T`. If multiple values of the same type are provided, only the last one
will be kept.

You may use [`get_extension`](Self::get_extension) to retrieve extensions.

# Example
```
use datafusion_execution::config::SessionConfig;
use std::sync::Arc;

// application-specific extension types
struct Ext1(u8);
struct Ext2(u8);
struct Ext3(u8);

let ext1a = Arc::new(Ext1(10));
let ext1b = Arc::new(Ext1(11));
let ext2 = Arc::new(Ext2(2));

let cfg = SessionConfig::default()
    // will only remember the last Ext1
    .with_extension(Arc::clone(&ext1a))
    .with_extension(Arc::clone(&ext1b))
    .with_extension(Arc::clone(&ext2));

let ext1_received = cfg.get_extension::<Ext1>().unwrap();
assert!(!Arc::ptr_eq(&ext1_received, &ext1a));
assert!(Arc::ptr_eq(&ext1_received, &ext1b));

let ext2_received = cfg.get_extension::<Ext2>().unwrap();
assert!(Arc::ptr_eq(&ext2_received, &ext2));

assert!(cfg.get_extension::<Ext3>().is_none());
```

[^1]: Compare that to [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4) which only supports [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) payloads.

Unresolved upstream links (retained, not inferred): ``Clone``, ``Arc``.

<a id="op-a23f7e7a9a4708e0204845da"></a>
## with_information_schema

`function` · `datafusion_execution::config::SessionConfig::with_information_schema` · datafusion-execution 55.1.0

```rust
fn with_information_schema(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the inclusion of `information_schema` virtual tables

<a id="op-525ab17615d84d3d332a90f5"></a>
## with_option_extension

`function` · `datafusion_execution::config::SessionConfig::with_option_extension` · datafusion-execution 55.1.0

```rust
fn with_option_extension<T: ConfigExtension>(self, extension: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Insert new [ConfigExtension](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c)

<a id="op-a8cfd21d7ccab1083ea81572"></a>
## with_parquet_bloom_filter_pruning

`function` · `datafusion_execution::config::SessionConfig::with_parquet_bloom_filter_pruning` · datafusion-execution 55.1.0

```rust
fn with_parquet_bloom_filter_pruning(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the use of bloom filter for parquet readers to skip row groups

<a id="op-31dfd00918796a9831dbc23f"></a>
## with_parquet_page_index_pruning

`function` · `datafusion_execution::config::SessionConfig::with_parquet_page_index_pruning` · datafusion-execution 55.1.0

```rust
fn with_parquet_page_index_pruning(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the use of page index for parquet readers to skip parquet data pages

<a id="op-1e63b529ad0f30d45b13cf73"></a>
## with_parquet_pruning

`function` · `datafusion_execution::config::SessionConfig::with_parquet_pruning` · datafusion-execution 55.1.0

```rust
fn with_parquet_pruning(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the use of pruning predicate for parquet readers to skip row groups

<a id="op-bddd4359cf419e9d45de291c"></a>
## with_prefer_existing_sort

`function` · `datafusion_execution::config::SessionConfig::with_prefer_existing_sort` · datafusion-execution 55.1.0

```rust
fn with_prefer_existing_sort(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Prefer existing sort (true) or maximize parallelism (false). See
[prefer_existing_sort] for more details

[prefer_existing_sort]: datafusion_common::config::OptimizerOptions::prefer_existing_sort

<a id="op-2197eb053ba8c04b115db1a3"></a>
## with_prefer_existing_union

`function` · `datafusion_execution::config::SessionConfig::with_prefer_existing_union` · datafusion-execution 55.1.0

```rust
fn with_prefer_existing_union(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Prefer existing union (true). See [prefer_existing_union] for more details

[prefer_existing_union]: datafusion_common::config::OptimizerOptions::prefer_existing_union

<a id="op-2e618632743e16ff8c90c8be"></a>
## with_repartition_aggregations

`function` · `datafusion_execution::config::SessionConfig::with_repartition_aggregations` · datafusion-execution 55.1.0

```rust
fn with_repartition_aggregations(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the use of repartitioning for aggregations to improve parallelism

<a id="op-6efeb1f01f5a64dea498a7b2"></a>
## with_repartition_file_min_size

`function` · `datafusion_execution::config::SessionConfig::with_repartition_file_min_size` · datafusion-execution 55.1.0

```rust
fn with_repartition_file_min_size(self, size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Sets minimum file range size for repartitioning scans

<a id="op-5c0ce2a1fff31d440da5f76c"></a>
## with_repartition_file_scans

`function` · `datafusion_execution::config::SessionConfig::with_repartition_file_scans` · datafusion-execution 55.1.0

```rust
fn with_repartition_file_scans(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the use of repartitioning for file scans

<a id="op-11511dcce37f8bdb3ecb9967"></a>
## with_repartition_joins

`function` · `datafusion_execution::config::SessionConfig::with_repartition_joins` · datafusion-execution 55.1.0

```rust
fn with_repartition_joins(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the use of repartitioning for joins to improve parallelism

<a id="op-44f53a13a22e387a47460b70"></a>
## with_repartition_sorts

`function` · `datafusion_execution::config::SessionConfig::with_repartition_sorts` · datafusion-execution 55.1.0

```rust
fn with_repartition_sorts(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the use of per-partition sorting to improve parallelism

<a id="op-e2566c96c7c97c1377b25cfc"></a>
## with_repartition_windows

`function` · `datafusion_execution::config::SessionConfig::with_repartition_windows` · datafusion-execution 55.1.0

```rust
fn with_repartition_windows(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the use of repartitioning for window functions to improve parallelism

<a id="op-63610a1ced624de7e4fc9410"></a>
## with_round_robin_repartition

`function` · `datafusion_execution::config::SessionConfig::with_round_robin_repartition` · datafusion-execution 55.1.0

```rust
fn with_round_robin_repartition(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Enables or disables the round robin repartition for increasing parallelism

<a id="op-a730183c433ce599c2f98cc1"></a>
## with_sort_in_place_threshold_bytes

`function` · `datafusion_execution::config::SessionConfig::with_sort_in_place_threshold_bytes` · datafusion-execution 55.1.0

```rust
fn with_sort_in_place_threshold_bytes(self, sort_in_place_threshold_bytes: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set the size of [`sort_in_place_threshold_bytes`] to control
how sort does things.

[`sort_in_place_threshold_bytes`]: datafusion_common::config::ExecutionOptions::sort_in_place_threshold_bytes

<a id="op-48495b5080a727bce2b84e5e"></a>
## with_sort_spill_reservation_bytes

`function` · `datafusion_execution::config::SessionConfig::with_sort_spill_reservation_bytes` · datafusion-execution 55.1.0

```rust
fn with_sort_spill_reservation_bytes(self, sort_spill_reservation_bytes: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set the size of [`sort_spill_reservation_bytes`] to control
memory pre-reservation

[`sort_spill_reservation_bytes`]: datafusion_common::config::ExecutionOptions::sort_spill_reservation_bytes

<a id="op-0bea37eed793bfd6c3084d5d"></a>
## with_spill_compression

`function` · `datafusion_execution::config::SessionConfig::with_spill_compression` · datafusion-execution 55.1.0

```rust
fn with_spill_compression(self, spill_compression: SpillCompression) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set the compression codec [`spill_compression`] used when spilling data to disk.

[`spill_compression`]: datafusion_common::config::ExecutionOptions::spill_compression

<a id="op-49e70ad2f18e3e31684328b4"></a>
## with_target_partitions

`function` · `datafusion_execution::config::SessionConfig::with_target_partitions` · datafusion-execution 55.1.0

```rust
fn with_target_partitions(self, n: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [613, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Customize [`target_partitions`]

[`target_partitions`]: datafusion_common::config::ExecutionOptions::target_partitions
