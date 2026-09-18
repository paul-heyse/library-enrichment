# `datafusion_datasource::file::FileSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file.FileSource.json).

<a id="op-8c1b19f80ea0c73466216a63"></a>
## FileSource

`trait` · `datafusion_datasource::file::FileSource` · datafusion-datasource 55.1.0

```rust
trait FileSource: Any + Send + Sync
```

Source: `src/file.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

File format specific behaviors for [`DataSource`]

# Schema information
There are two important schemas for a [`FileSource`](../operations/datafusion_datasource.file.FileSource.md#op-8c1b19f80ea0c73466216a63):
1. [`Self::table_schema`](../operations/datafusion_datasource.file.FileSource.md#op-5d12d1eeea4656c1e1a3bf3e) -- the schema for the overall table
   (file data plus partition columns)
2. The logical output schema, comprised of [`Self::table_schema`](../operations/datafusion_datasource.file.FileSource.md#op-5d12d1eeea4656c1e1a3bf3e) with
   [`Self::projection`](../operations/datafusion_datasource.file.FileSource.md#op-0222a3b6073f25c90ced3f20) applied

See more details on specific implementations:
* [`ArrowSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.ArrowSource.html)
* [`AvroSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.AvroSource.html)
* [`CsvSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.CsvSource.html)
* [`JsonSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.JsonSource.html)
* [`ParquetSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.ParquetSource.html)

[`DataSource`]: crate::source::DataSource

<a id="op-54278e73514d98c4a5081400"></a>
## apply_expressions

`function` · `datafusion_datasource::file::FileSource::apply_expressions` · datafusion-datasource 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Source: `src/file.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Apply a function to all physical expressions used by this file source.

This includes:
- Filter predicates (which may contain dynamic filters)
- Projection expressions

The function `f` should be called once per expression unless the function returns
[`TreeNodeRecursion::Stop`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-fbbedd1f0409ac786df535a1) to stop iteration.

See [`ExecutionPlan::apply_expressions`] for more details and implementation examples.

[`ExecutionPlan::apply_expressions`]: datafusion_physical_plan::ExecutionPlan::apply_expressions

<a id="op-04948e3f4428189d061797ac"></a>
## create_file_opener

`function` · `datafusion_datasource::file::FileSource::create_file_opener` · datafusion-datasource 55.1.0

```rust
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition: usize) -> Result<Arc<dyn FileOpener>>
```

Source: `src/file.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a `dyn FileOpener` based on given parameters.

Note: File sources with a native morsel implementation should return an
error from this method and implementing [`Self::create_morselizer`](../operations/datafusion_datasource.file.FileSource.md#op-e372eb95110decaac1ab248e) instead.

<a id="op-e372eb95110decaac1ab248e"></a>
## create_morselizer

`function` · `datafusion_datasource::file::FileSource::create_morselizer` · datafusion-datasource 55.1.0

```rust
fn create_morselizer(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition: usize) -> Result<Box<dyn Morselizer>>
```

Source: `src/file.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a `dyn Morselizer` based on given parameters.

The default implementation preserves existing behavior by adapting the
legacy [`FileOpener`](../operations/datafusion_datasource.file_stream.FileOpener.md#op-de9e825a0c7367b18e03d6f1) API into a [`Morselizer`](../operations/datafusion_datasource.morsel.Morselizer.md#op-4dc8c6c12cccc0db0eec6703).

It is preferred to implement the [`Morselizer`](../operations/datafusion_datasource.morsel.Morselizer.md#op-4dc8c6c12cccc0db0eec6703) API directly by
implementing this method.

<a id="op-2e0e153662dee3e71f2fbd3f"></a>
## file_type

`function` · `datafusion_datasource::file::FileSource::file_type` · datafusion-datasource 55.1.0

```rust
fn file_type(&self) -> &str
```

Source: `src/file.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

String representation of file source such as "csv", "json", "parquet"

<a id="op-09dcb4160e6584441ae99415"></a>
## filter

`function` · `datafusion_datasource::file::FileSource::filter` · datafusion-datasource 55.1.0

```rust
fn filter(&self) -> Option<Arc<dyn PhysicalExpr>>
```

Source: `src/file.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the filter expression that will be applied *during* the file scan.

These expressions are in terms of the unprojected [`Self::table_schema`](../operations/datafusion_datasource.file.FileSource.md#op-5d12d1eeea4656c1e1a3bf3e).

<a id="op-25ec6a31dece3afe968c8bd1"></a>
## fmt_extra

`function` · `datafusion_datasource::file::FileSource::fmt_extra` · datafusion-datasource 55.1.0

```rust
fn fmt_extra(&self, _t: DisplayFormatType, _f: &mut Formatter<'_>) -> fmt::Result
```

Source: `src/file.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Format FileType specific information

<a id="op-bf9bc0bd742e7e6e34b6d452"></a>
## metrics

`function` · `datafusion_datasource::file::FileSource::metrics` · datafusion-datasource 55.1.0

```rust
fn metrics(&self) -> &ExecutionPlanMetricsSet
```

Source: `src/file.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return execution plan metrics

<a id="op-0222a3b6073f25c90ced3f20"></a>
## projection

`function` · `datafusion_datasource::file::FileSource::projection` · datafusion-datasource 55.1.0

```rust
fn projection(&self) -> Option<&ProjectionExprs>
```

Source: `src/file.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the projection that will be applied to the output stream on top
of [`Self::table_schema`](../operations/datafusion_datasource.file.FileSource.md#op-5d12d1eeea4656c1e1a3bf3e).

Note you can use [`ProjectionExprs::project_schema`] on the table
schema to get the effective output schema of this source.

Unresolved upstream links (retained, not inferred): ``ProjectionExprs::project_schema``.

<a id="op-970b3f8908b16de658e1f7ed"></a>
## reorder_files

`function` · `datafusion_datasource::file::FileSource::reorder_files` · datafusion-datasource 55.1.0

```rust
fn reorder_files(&self, files: Vec<PartitionedFile>) -> Vec<PartitionedFile>
```

Source: `src/file.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Reorder files in the shared work queue to optimize query performance.

For example, TopK queries benefit from reading files with the best
statistics first, so the dynamic filter threshold tightens quickly.

The default implementation returns files unchanged (no reordering).

<a id="op-31968499ac7eeb39834010b2"></a>
## repartitioned

`function` · `datafusion_datasource::file::FileSource::repartitioned` · datafusion-datasource 55.1.0

```rust
fn repartitioned(&self, target_partitions: usize, repartition_file_min_size: usize, output_ordering: Option<LexOrdering>, config: &FileScanConfig) -> Result<Option<FileScanConfig>>
```

Source: `src/file.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

If supported by the [`FileSource`](../operations/datafusion_datasource.file.FileSource.md#op-8c1b19f80ea0c73466216a63), redistribute files across partitions
according to their size. Allows custom file formats to implement their
own repartitioning logic.

The default implementation uses [`FileGroupPartitioner`](../operations/datafusion_datasource.file_groups.FileGroupPartitioner.md#op-7b24c3a42bf8f20212e4a135). See that
struct for more details.

<a id="op-83110f8216b0e926a2816eea"></a>
## schema_adapter_factory

`function` · `datafusion_datasource::file::FileSource::schema_adapter_factory` · datafusion-datasource 55.1.0

```rust
fn schema_adapter_factory(&self) -> Option<Arc<dyn SchemaAdapterFactory>>
```

Source: `src/file.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Returns the current schema adapter factory if set.

`SchemaAdapterFactory` has been removed. Use `PhysicalExprAdapterFactory` instead.
See `upgrading.md` for more details.

<a id="op-01963ce766d3917b03f28775"></a>
## supports_repartitioning

`function` · `datafusion_datasource::file::FileSource::supports_repartitioning` · datafusion-datasource 55.1.0

```rust
fn supports_repartitioning(&self) -> bool
```

Source: `src/file.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns whether this file source supports repartitioning files by byte ranges.

When this returns `true`, files can be split into multiple partitions
based on byte offsets for parallel reading.

When this returns `false`, files cannot be repartitioned (e.g., CSV files
with `newlines_in_values` enabled cannot be split because record boundaries
cannot be determined by byte offset alone).

The default implementation returns `true`. File sources that cannot support
repartitioning should override this method.

<a id="op-5d12d1eeea4656c1e1a3bf3e"></a>
## table_schema

`function` · `datafusion_datasource::file::FileSource::table_schema` · datafusion-datasource 55.1.0

```rust
fn table_schema(&self) -> &table_schema::TableSchema
```

Source: `src/file.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the table schema for the overall table (including partition columns, if any)

This method returns the unprojected schema: the full schema of the data
without [`Self::projection`](../operations/datafusion_datasource.file.FileSource.md#op-0222a3b6073f25c90ced3f20) applied.

The output schema of this `FileSource` is this TableSchema
with [`Self::projection`](../operations/datafusion_datasource.file.FileSource.md#op-0222a3b6073f25c90ced3f20) applied.

Use [`ProjectionExprs::project_schema`] to get the projected schema
after applying the projection.

Unresolved upstream links (retained, not inferred): ``ProjectionExprs::project_schema``.

<a id="op-712b05c66a4b7e2a866163fe"></a>
## try_pushdown_filters

`function` · `datafusion_datasource::file::FileSource::try_pushdown_filters` · datafusion-datasource 55.1.0

```rust
fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn FileSource>>>
```

Source: `src/file.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Try to push down filters into this FileSource.

`filters` must be in terms of the unprojected table schema (file schema
plus partition columns), before any projection is applied.

Any filters that this FileSource chooses to evaluate itself should be
returned as `PushedDown::Yes` in the result, along with a FileSource
instance that incorporates those filters. Such filters are logically
applied "during" the file scan, meaning they may refer to columns not
included in the final output projection.

Filters that cannot be pushed down should be marked as `PushedDown::No`,
and will be evaluated by an execution plan after the file source.

See [`ExecutionPlan::handle_child_pushdown_result`] for more details.

[`ExecutionPlan::handle_child_pushdown_result`]: datafusion_physical_plan::ExecutionPlan::handle_child_pushdown_result

<a id="op-7fc5dc0ef450b149553f4e72"></a>
## try_pushdown_projection

`function` · `datafusion_datasource::file::FileSource::try_pushdown_projection` · datafusion-datasource 55.1.0

```rust
fn try_pushdown_projection(&self, _projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
```

Source: `src/file.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Try to push down a projection into this FileSource.

`FileSource` implementations that support projection pushdown should
override this method and return a new `FileSource` instance with the
projection incorporated.

If a `FileSource` does accept a projection it is expected to handle
the projection in it's entirety, including partition columns.
For example, the `FileSource` may translate that projection into a
file format specific projection (e.g. Parquet can push down struct field access,
some other file formats like Vortex can push down computed expressions into un-decoded data)
and also need to handle partition column projection (generally done by replacing partition column
references with literal values derived from each files partition values).

Not all FileSource's can handle complex expression pushdowns. For example,
a CSV file source may only support simple column selections. In such cases,
the `FileSource` can use [`SplitProjection`] and [`ProjectionOpener`]
to split the projection into a pushdownable part and a non-pushdownable part.
These helpers also handle partition column projection.

[`SplitProjection`]: crate::projection::SplitProjection
[`ProjectionOpener`]: crate::projection::ProjectionOpener

<a id="op-140ec04e9bfe3fe1e1e09b41"></a>
## try_pushdown_sort

`function` · `datafusion_datasource::file::FileSource::try_pushdown_sort` · datafusion-datasource 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr], eq_properties: &EquivalenceProperties) -> Result<SortOrderPushdownResult<Arc<dyn FileSource>>>
```

Source: `src/file.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Try to create a new FileSource that can produce data in the specified sort order.

This method attempts to optimize data retrieval to match the requested ordering.
It receives both the requested ordering and equivalence properties that describe
the output data from this file source.

# Parameters
* `order` - The requested sort ordering from the query
* `eq_properties` - Equivalence properties of the data that will be produced by this
  file source. These properties describe the ordering, constant columns, and other
  relationships in the output data, allowing the implementation to determine if
  optimizations like reversed scanning can help satisfy the requested ordering.
  This includes information about:
  - The file's natural ordering (from output_ordering in FileScanConfig)
  - Constant columns (e.g., from filters like `ticker = 'AAPL'`)
  - Monotonic functions (e.g., `extract_year_month(timestamp)`)
  - Other equivalence relationships

# Examples

## Example 1: Simple reverse
```text
File ordering: [a ASC, b DESC]
Requested:     [a DESC]
Reversed file: [a DESC, b ASC]
Result: Satisfies request (prefix match) → Inexact
```

## Example 2: Monotonic function
```text
File ordering: [extract_year_month(ts) ASC, ts ASC]
Requested:     [ts DESC]
Reversed file: [extract_year_month(ts) DESC, ts DESC]
Result: Through monotonicity, satisfies [ts DESC] → Inexact
```

# Returns
* `Exact` - Created a source that guarantees perfect ordering
* `Inexact` - Created a source optimized for ordering (e.g., reversed row groups) but not perfectly sorted
* `Unsupported` - Cannot optimize for this ordering

# Deprecation / migration notes
- [`Self::try_reverse_output`](../operations/datafusion_datasource.file.FileSource.md#op-cc25f64751ac02f8fa05460b) was renamed to this method and deprecated since `53.0.0`.
  Per DataFusion's deprecation guidelines, it will be removed in `59.0.0` or later
  (6 major versions or 6 months, whichever is longer).
- New implementations should override [`Self::try_pushdown_sort`](../operations/datafusion_datasource.file.FileSource.md#op-140ec04e9bfe3fe1e1e09b41) directly.
- For backwards compatibility, the default implementation of
  [`Self::try_pushdown_sort`](../operations/datafusion_datasource.file.FileSource.md#op-140ec04e9bfe3fe1e1e09b41) delegates to the deprecated
  [`Self::try_reverse_output`](../operations/datafusion_datasource.file.FileSource.md#op-cc25f64751ac02f8fa05460b) until it is removed. After that point, the
  default implementation will return [`SortOrderPushdownResult::Unsupported`](../operations/datafusion_physical_plan.sort_pushdown.SortOrderPushdownResult.md#op-2741d2a02b139c9a94076044).

<a id="op-cc25f64751ac02f8fa05460b"></a>
## try_reverse_output

`function` · `datafusion_datasource::file::FileSource::try_reverse_output` · datafusion-datasource 55.1.0

```rust
fn try_reverse_output(&self, _order: &[PhysicalSortExpr], _eq_properties: &EquivalenceProperties) -> Result<SortOrderPushdownResult<Arc<dyn FileSource>>>
```

Source: `src/file.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Renamed to [`Self::try_pushdown_sort`](../operations/datafusion_datasource.file.FileSource.md#op-140ec04e9bfe3fe1e1e09b41).

<a id="op-83152ed75bb6a4e441648c13"></a>
## try_to_proto

`function` · `datafusion_datasource::file::FileSource::try_to_proto` · datafusion-datasource 55.1.0

```rust
fn try_to_proto(&self, _base: &FileScanConfig, _ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Source: `src/file.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Serialize this file source into a full [`PhysicalPlanNode`] (a
`DataSourceExec` wrapping the `FileScanConfig`), if it knows how.

`base` is the shared [`FileScanConfig`] this source is wrapped in; the
format-agnostic parts (file groups, schema, statistics, ordering,
projection, …) are encoded via
[`FileScanConfig::try_to_proto`](crate::file_scan_config::FileScanConfig::try_to_proto),
and the concrete source appends its format-specific fields (e.g. CSV
delimiter/quote) around it.

* `Ok(None)` (the default) — this source has no proto hook yet; the
  caller falls back to the central downcast chain in `datafusion-proto`.
* `Ok(Some(node))` — fully serialized; the caller must not fall back.

[`PhysicalPlanNode`]: datafusion_proto_models::protobuf::PhysicalPlanNode
[`FileScanConfig`]: crate::file_scan_config::FileScanConfig

<a id="op-f16c55d10a8ff97c8f4cac12"></a>
## with_batch_size

`function` · `datafusion_datasource::file::FileSource::with_batch_size` · datafusion-datasource 55.1.0

```rust
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

Source: `src/file.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Initialize new type with batch size configuration

<a id="op-d8ed3228c2ec0899c5db3e4f"></a>
## with_schema_adapter_factory

`function` · `datafusion_datasource::file::FileSource::with_schema_adapter_factory` · datafusion-datasource 55.1.0

```rust
fn with_schema_adapter_factory(&self, _factory: Arc<dyn SchemaAdapterFactory>) -> Result<Arc<dyn FileSource>>
```

Source: `src/file.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Set optional schema adapter factory.

`SchemaAdapterFactory` has been removed. Use `PhysicalExprAdapterFactory` instead.
See `upgrading.md` for more details.
