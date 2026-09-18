# `datafusion_common::stats::Statistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.stats.Statistics.json).

<a id="op-06d2580fbefe2068a74aafb6"></a>
## Statistics

`struct` · `datafusion_common::stats::Statistics` · datafusion-common 55.1.0

```rust
struct Statistics
```

Source: `src/stats.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Statistics for a relation
Fields are optional and can be inexact because the sources
sometimes provide approximate estimates for performance reasons
and the transformations output are not always predictable.

<a id="op-56556c1ae2dd05cfea5ae5ea"></a>
## add_column_statistics

`function` · `datafusion_common::stats::Statistics::add_column_statistics` · datafusion-common 55.1.0

```rust
fn add_column_statistics(self, column_stats: ColumnStatistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add a column to the column statistics

<a id="op-5f54a2667df3c486226ef600"></a>
## calculate_total_byte_size

`function` · `datafusion_common::stats::Statistics::calculate_total_byte_size` · datafusion-common 55.1.0

```rust
fn calculate_total_byte_size(&mut self, schema: &Schema)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates `total_byte_size` based on the schema and `num_rows`.
If any of the columns has non-primitive width, or `num_rows` is unknown,
the previous `total_byte_size` is kept but downgraded to inexact rather
than discarded.

<a id="op-a40918eba20547045e49e180"></a>
## clone

`function` · `datafusion_common::stats::Statistics::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Statistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [367, 17], "end": [367, 22], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/stats.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98ce2884971eff5d6260bd5d"></a>
## column_statistics

`struct_field` · `datafusion_common::stats::Statistics::column_statistics` · datafusion-common 55.1.0

```rust
column_statistics: Vec<ColumnStatistics>
```

Source: `src/stats.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Statistics on a column level.

It must contains a [`ColumnStatistics`](../operations/datafusion_common.stats.ColumnStatistics.md#op-0684a7f4d4e937976b526c06) for each field in the schema of
the table to which the [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) refer.

<a id="op-05d5c7bfc1de1927e923c535"></a>
## default

`function` · `datafusion_common::stats::Statistics::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [416, 2], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/stats.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a new [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) instance with all fields set to unknown
and no columns.

<a id="op-b8565351d2636076a9abf412"></a>
## eq

`function` · `datafusion_common::stats::Statistics::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Statistics) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [367, 24], "end": [367, 33], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/stats.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bb98cfe090580d8dc170559"></a>
## fmt

`function` · `datafusion_common::stats::Statistics::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1013, 1], "end": [1066, 2], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/stats.rs:1014`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9d1f2fe94cc13b3ac8fe1b0"></a>
## fmt

`function` · `datafusion_common::stats::Statistics::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [367, 10], "end": [367, 15], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stats.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f36e4d91db482802fb62f0a"></a>
## heap_size

`function` · `datafusion_common::stats::Statistics::heap_size` · datafusion-common 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "crate::Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [91, 2], "filename": "src/heap_size.rs"}, "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}, "trait_path": "datafusion_common::heap_size::DFHeapSize"}`

Source: `src/heap_size.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ff73fd5da728d5c13155c9f"></a>
## new_unknown

`function` · `datafusion_common::stats::Statistics::new_unknown` · datafusion-common 55.1.0

```rust
fn new_unknown(schema: &Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:421`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) instance for the given schema by assigning
unknown statistics to each column in the schema.

<a id="op-c82d0e404348edec60ecdd15"></a>
## num_rows

`struct_field` · `datafusion_common::stats::Statistics::num_rows` · datafusion-common 55.1.0

```rust
num_rows: Precision<usize>
```

Source: `src/stats.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The number of rows estimated to be scanned.

<a id="op-e11e520a85190d3bb2c510d8"></a>
## project

`function` · `datafusion_common::stats::Statistics::project` · datafusion-common 55.1.0

```rust
fn project(self, projection: Option<&impl AsRef<[usize]>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Project the statistics to the given column indices.

For example, if we had statistics for columns `{"a", "b", "c"}`,
projecting to `vec![2, 1]` would return statistics for columns `{"c",
"b"}`.

<a id="op-529ee6f1a172d140fb3a88bf"></a>
## to_inexact

`function` · `datafusion_common::stats::Statistics::to_inexact` · datafusion-common 55.1.0

```rust
fn to_inexact(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If the exactness of a [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) instance is lost, this function relaxes
the exactness of all information by converting them [`Precision::Inexact`](../operations/datafusion_common.stats.Precision.md#op-5d87555abc98461943c3ed02).

<a id="op-0ffa48cba0a5a726d7470790"></a>
## total_byte_size

`struct_field` · `datafusion_common::stats::Statistics::total_byte_size` · datafusion-common 55.1.0

```rust
total_byte_size: Precision<usize>
```

Source: `src/stats.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The total bytes of the output data.

Note that this is not the same as the total bytes that may be scanned,
processed, etc.
E.g. we may read 1GB of data from a Parquet file but the Arrow data
the node produces may be 2GB; it's this 2GB that is tracked here.

<a id="op-511f54c67f4f786903849795"></a>
## try_merge_iter

`function` · `datafusion_common::stats::Statistics::try_merge_iter` · datafusion-common 55.1.0

```rust
fn try_merge_iter<'a, I>(items: I, schema: &Schema) -> Result<Statistics> where I: IntoIterator<Item = &'a Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:714`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Summarize zero or more statistics into a single `Statistics` instance.

The method assumes that all statistics are for the same schema.
If not, maybe you can call `SchemaMapper::map_column_statistics` to make them consistent.

This method uses [`NdvFallback::Max`](../operations/datafusion_common.stats.NdvFallback.md#op-894f13b7f6fa8b6caeb809f8) when `distinct_count` overlap
can not be estimated from column bounds.

Returns an error if the statistics do not match the specified schemas.

# Example
```
# use datafusion_common::{ColumnStatistics, ScalarValue, Statistics};
# use arrow::datatypes::{Field, Schema, DataType};
# use datafusion_common::stats::Precision;
let stats1 = Statistics::default()
    .with_num_rows(Precision::Exact(10))
    .add_column_statistics(
        ColumnStatistics::new_unknown()
            .with_min_value(Precision::Exact(ScalarValue::from(1)))
            .with_max_value(Precision::Exact(ScalarValue::from(100)))
            .with_sum_value(Precision::Exact(ScalarValue::from(500))),
    );

let stats2 = Statistics::default()
    .with_num_rows(Precision::Exact(20))
    .add_column_statistics(
        ColumnStatistics::new_unknown()
            .with_min_value(Precision::Exact(ScalarValue::from(5)))
            .with_max_value(Precision::Exact(ScalarValue::from(200)))
            .with_sum_value(Precision::Exact(ScalarValue::from(1000))),
    );

let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let merged = Statistics::try_merge_iter(
    &[stats1, stats2],
    &schema,
).unwrap();

assert_eq!(merged.num_rows, Precision::Exact(30));
assert_eq!(merged.column_statistics[0].min_value,
    Precision::Exact(ScalarValue::from(1)));
assert_eq!(merged.column_statistics[0].max_value,
    Precision::Exact(ScalarValue::from(200)));
assert_eq!(merged.column_statistics[0].sum_value,
    Precision::Exact(ScalarValue::Int64(Some(1500))));
```

<a id="op-4a89a1e185c4c13b36d7c33b"></a>
## try_merge_iter_with_ndv_fallback

`function` · `datafusion_common::stats::Statistics::try_merge_iter_with_ndv_fallback` · datafusion-common 55.1.0

```rust
fn try_merge_iter_with_ndv_fallback<'a, I>(items: I, schema: &Schema, ndv_fallback: NdvFallback) -> Result<Statistics> where I: IntoIterator<Item = &'a Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Same as [`Statistics::try_merge_iter`](../operations/datafusion_common.stats.Statistics.md#op-511f54c67f4f786903849795), but lets callers choose the
fallback used when `distinct_count` overlap can not be estimated.

<a id="op-e299d9af60771698675838c8"></a>
## unknown_column

`function` · `datafusion_common::stats::Statistics::unknown_column` · datafusion-common 55.1.0

```rust
fn unknown_column(schema: &Schema) -> Vec<ColumnStatistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns an unbounded `ColumnStatistics` for each field in the schema.

<a id="op-766dfbe142bf075a2c6281c4"></a>
## with_fetch

`function` · `datafusion_common::stats::Statistics::with_fetch` · datafusion-common 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>, skip: usize, n_partitions: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:548`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the statistics after applying `fetch` and `skip` operations.

Here, `self` denotes per-partition statistics. Use the `n_partitions`
parameter to compute global statistics in a multi-partition setting.

<a id="op-560a2d72c9b036c802c788a1"></a>
## with_num_rows

`function` · `datafusion_common::stats::Statistics::with_num_rows` · datafusion-common 55.1.0

```rust
fn with_num_rows(self, num_rows: Precision<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the number of rows

<a id="op-687e614568d38b8262f4ad23"></a>
## with_total_byte_size

`function` · `datafusion_common::stats::Statistics::with_total_byte_size` · datafusion-common 55.1.0

```rust
fn with_total_byte_size(self, total_byte_size: Precision<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [795, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the total size, in bytes
