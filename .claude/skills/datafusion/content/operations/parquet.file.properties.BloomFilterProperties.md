# `parquet::file::properties::BloomFilterProperties`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.BloomFilterProperties.json).

<a id="op-d8c6090fa6f2cfe781d4070c"></a>
## BloomFilterProperties

`struct` · `parquet::file::properties::BloomFilterProperties` · parquet 59.3.0

```rust
struct BloomFilterProperties
```

Source: `src/file/properties.rs:1439`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Controls the bloom filter to be computed by the writer.

The bloom filter is initially sized for `ndv` distinct values at the given `fpp`, then
automatically folded down after all values are inserted to achieve optimal size while
maintaining the target `fpp`. See [`Sbbf::fold_to_target_fpp`] for details on the
folding algorithm.

# Example

```rust
# use parquet::{
#    file::properties::{BloomFilterProperties, WriterProperties},
#    schema::types::ColumnPath,
# };
// Build a BloomFilterProperties via the builder, then apply it to one column.
let bf = BloomFilterProperties::builder()
    .with_fpp(0.01)
    .with_max_ndv(10_000)
    .build();

let props = WriterProperties::builder()
    .set_column_bloom_filter_properties(ColumnPath::from("user_id"), bf.clone())
    .build();

assert_eq!(
    props.bloom_filter_properties(&ColumnPath::from("user_id")),
    Some(&bf)
);
```

[`Sbbf::fold_to_target_fpp`]: crate::bloom_filter::Sbbf::fold_to_target_fpp

<a id="op-fc030a452ac061a92476a2ac"></a>
## builder

`function` · `parquet::file::properties::BloomFilterProperties::builder` · parquet 59.3.0

```rust
fn builder() -> BloomFilterPropertiesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterProperties", "path": "BloomFilterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 1], "end": [1498, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1456`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a new [`BloomFilterPropertiesBuilder`](../operations/parquet.file.properties.BloomFilterPropertiesBuilder.md#op-32e29ff964a10174bac103fc) for constructing
[`BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c) with custom values.

<a id="op-686fb5cf1e83a59c7429251d"></a>
## clone

`function` · `parquet::file::properties::BloomFilterProperties::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BloomFilterProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterProperties", "path": "BloomFilterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 17], "end": [1438, 22], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/properties.rs:1438`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c821567b4933ef431c5c4e06"></a>
## default

`function` · `parquet::file::properties::BloomFilterProperties::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterProperties", "path": "BloomFilterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1444, 1], "end": [1451, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/properties.rs:1445`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-419979450b3d3ac95d4964b1"></a>
## eq

`function` · `parquet::file::properties::BloomFilterProperties::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &BloomFilterProperties) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterProperties", "path": "BloomFilterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 24], "end": [1438, 33], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/properties.rs:1438`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-476d59c0bd7f3af60a2eaee6"></a>
## fmt

`function` · `parquet::file::properties::BloomFilterProperties::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterProperties", "path": "BloomFilterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 10], "end": [1438, 15], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/properties.rs:1438`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8be17fbc04d515b7d1b56e5c"></a>
## fpp

`function` · `parquet::file::properties::BloomFilterProperties::fpp` · parquet 59.3.0

```rust
fn fpp(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterProperties", "path": "BloomFilterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 1], "end": [1498, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1470`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

False positive probability. This should be always between 0 and 1 exclusive. Defaults to [`DEFAULT_BLOOM_FILTER_FPP`](../operations/parquet.file.properties.DEFAULT_BLOOM_FILTER_FPP.md#op-6a638a0205a9f6e75faeba63).

You should set this value by calling [`WriterPropertiesBuilder::set_bloom_filter_fpp`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-3ca60c9b3704cc6b94959616).

The bloom filter data structure is a trade of between disk and memory space versus fpp, the
smaller the fpp, the more memory and disk space is required, thus setting it to a reasonable value
e.g. 0.1, 0.05, or 0.001 is recommended.

This value also serves as the target FPP for bloom filter folding: after all values
are inserted, the filter is folded down to the smallest size that still meets this FPP.

<a id="op-39b748ff76f16fec3d6a46aa"></a>
## ndv

`function` · `parquet::file::properties::BloomFilterProperties::ndv` · parquet 59.3.0

```rust
fn ndv(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterProperties", "path": "BloomFilterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 1], "end": [1498, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1495`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Maximum expected number of distinct values. Defaults to [`DEFAULT_BLOOM_FILTER_NDV`](../operations/parquet.file.properties.DEFAULT_BLOOM_FILTER_NDV.md#op-9c8e92e4f942656d39a72c51).

You should set this value by calling [`WriterPropertiesBuilder::set_bloom_filter_max_ndv`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-57fa972053a9b4d5a03eefc5).

When not explicitly set via the builder, this defaults to
[`max_row_group_row_count`](WriterProperties::max_row_group_row_count) (resolved at
build time). The bloom filter is initially sized for this many distinct values at the
given `fpp`, then folded down after insertion to achieve optimal size. A good heuristic
is to set this to the expected number of rows in the row group. If fewer distinct values
are actually written, the filter will be automatically compacted via folding.

Thus the only negative side of overestimating this value is that the bloom filter
will use more memory during writing than necessary, but it will not affect the final
bloom filter size on disk.

If you wish to reduce memory usage during writing and are able to make a reasonable estimate
of the number of distinct values in a row group, it is recommended to set this value explicitly
rather than relying on the default dynamic sizing based on `max_row_group_row_count`.
If you do set this value explicitly it is probably best to set it for each column
individually via [`WriterPropertiesBuilder::set_column_bloom_filter_max_ndv`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-23e425f8136d525313b1e884) rather than globally,
since different columns may have different numbers of distinct values.
