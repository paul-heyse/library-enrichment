# `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.bloom_filter.BloomFilterStatistics.json).

<a id="op-e6d24f33931ffc9f2f664552"></a>
## BloomFilterStatistics

`struct` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics` · datafusion-datasource-parquet 55.1.0

```rust
struct BloomFilterStatistics
```

Source: `src/bloom_filter.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

In memory Parquet Split Block Bloom Filters (SBBF).

This structure implements [`PruningStatistics`](../operations/datafusion_common.pruning.PruningStatistics.md#op-a18da0087d8b285319f91952) and is used to prune
Parquet row groups and data pages based on the query predicate.

<a id="op-2bfd39ee3d102c2883ccc9ad"></a>
## clone

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::clone` · datafusion-datasource-parquet 55.1.0

```rust
fn clone(&self) -> BloomFilterStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 17], "end": [35, 22], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/bloom_filter.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f41458af8687fec566e31b60"></a>
## contained

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::contained` · datafusion-datasource-parquet 55.1.0

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [233, 2], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/bloom_filter.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Use bloom filters to determine if we are sure this column can not
possibly contain `values`

The `contained` API returns false if the bloom filters knows that *ALL*
of the values in a column are not present.

<a id="op-94b0ce8c0d5151784efe53f2"></a>
## default

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::default` · datafusion-datasource-parquet 55.1.0

```rust
fn default() -> BloomFilterStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 24], "end": [35, 31], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/bloom_filter.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38ad4784a727b1e94dcb5abc"></a>
## fmt

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/bloom_filter.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4ab15be33cc38e340a4bc1a"></a>
## insert

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::insert` · datafusion-datasource-parquet 55.1.0

```rust
fn insert(&mut self, column: impl Into<String>, sbbf: Sbbf, ty: Type, type_length: i32)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [168, 2], "filename": "src/bloom_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Add a Bloom filter for the specified column, along with the column's
Parquet physical [`Type`](../operations/parquet.basic.Type.md#op-796c3f25eb2f2865773fe11c) and type length from the column descriptor.

<a id="op-b989966e656a4d358de5d2ab"></a>
## max_values

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::max_values` · datafusion-datasource-parquet 55.1.0

```rust
fn max_values(&self, _column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [233, 2], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/bloom_filter.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27a297b8c44e7cddda229719"></a>
## min_values

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::min_values` · datafusion-datasource-parquet 55.1.0

```rust
fn min_values(&self, _column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [233, 2], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/bloom_filter.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eccb113996266f9061deb32c"></a>
## new

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [168, 2], "filename": "src/bloom_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create an empty [`BloomFilterStatistics`](../operations/datafusion_datasource_parquet.bloom_filter.BloomFilterStatistics.md#op-e6d24f33931ffc9f2f664552)

<a id="op-5551b855458bbf4d5f3c08c2"></a>
## null_counts

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::null_counts` · datafusion-datasource-parquet 55.1.0

```rust
fn null_counts(&self, _column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [233, 2], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/bloom_filter.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afbe6a287bdc30c3b70fa15f"></a>
## num_containers

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::num_containers` · datafusion-datasource-parquet 55.1.0

```rust
fn num_containers(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [233, 2], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/bloom_filter.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a4e845d9b2f17a8647e9e15"></a>
## row_counts

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::row_counts` · datafusion-datasource-parquet 55.1.0

```rust
fn row_counts(&self) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [233, 2], "filename": "src/bloom_filter.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/bloom_filter.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2852e6a8e529ed3b02f327db"></a>
## with_capacity

`function` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics::with_capacity` · datafusion-datasource-parquet 55.1.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics", "path": "BloomFilterStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [168, 2], "filename": "src/bloom_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create an empty [`BloomFilterStatistics`](../operations/datafusion_datasource_parquet.bloom_filter.BloomFilterStatistics.md#op-e6d24f33931ffc9f2f664552) with the specified capacity
