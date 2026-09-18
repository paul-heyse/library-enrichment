# `parquet::file::statistics::ValueStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.statistics.ValueStatistics.json).

<a id="op-74731c7409f1ae7805102feb"></a>
## ValueStatistics

`struct` · `parquet::file::statistics::ValueStatistics` · parquet 59.3.0

```rust
struct ValueStatistics<T>
```

Source: `src/file/statistics.rs:508`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Typed statistics for one column chunk

See [`Statistics`](../operations/parquet.file.statistics.Statistics.md#op-ba51f82bfe4dce01512b0440) for more details

<a id="op-420cc8289084a4a2547b1910"></a>
## clone

`function` · `parquet::file::statistics::ValueStatistics::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ValueStatistics<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [507, 10], "end": [507, 15], "filename": "src/file/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/statistics.rs:507`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70a180340c73beff92d71dbd"></a>
## distinct_count

`function` · `parquet::file::statistics::ValueStatistics::distinct_count` · parquet 59.3.0

```rust
fn distinct_count(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:610`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns optional value of number of distinct values occurring.

<a id="op-69f84f454c06ea82dfda1fc1"></a>
## eq

`function` · `parquet::file::statistics::ValueStatistics::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ValueStatistics<T>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [507, 21], "end": [507, 30], "filename": "src/file/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/statistics.rs:507`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a2fb9265a36ba0800e8ad6a"></a>
## fmt

`function` · `parquet::file::statistics::ValueStatistics::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::data_type::private::ParquetValueType", "path": "ParquetValueType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [679, 2], "filename": "src/file/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/file/statistics.rs:652`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b864c540e4d4fadc5764feec"></a>
## fmt

`function` · `parquet::file::statistics::ValueStatistics::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::data_type::private::ParquetValueType", "path": "ParquetValueType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 1], "end": [697, 2], "filename": "src/file/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/statistics.rs:682`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6ffbdf485b781a786292dff"></a>
## is_min_max_backwards_compatible

`function` · `parquet::file::statistics::ValueStatistics::is_min_max_backwards_compatible` · parquet 59.3.0

```rust
fn is_min_max_backwards_compatible(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:634`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Old versions of parquet stored statistics in `min` and `max` fields, ordered
using signed comparison. This resulted in an undefined ordering for unsigned
quantities, such as booleans and unsigned integers.

These fields were therefore deprecated in favour of `min_value` and `max_value`,
which have a type-defined sort order.

However, not all readers have been updated. For backwards compatibility, this method
returns `true` if the statistics within this have a signed sort order, that is
compatible with being stored in the deprecated `min` and `max` fields

<a id="op-9fd4e49e1978b2b88e9b7dbb"></a>
## max_bytes_opt

`function` · `parquet::file::statistics::ValueStatistics::max_bytes_opt` · parquet 59.3.0

```rust
fn max_bytes_opt(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::data_type::AsBytes", "path": "AsBytes"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [649, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:646`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns max value as bytes of the statistics, if max value is known.

<a id="op-5e3bad1973bf6a4e718e92b2"></a>
## max_is_exact

`function` · `parquet::file::statistics::ValueStatistics::max_is_exact` · parquet 59.3.0

```rust
fn max_is_exact(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:600`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Whether or not max value is set, and is an exact value.

<a id="op-c2d9df46ca4d552ca5bd2b3a"></a>
## max_opt

`function` · `parquet::file::statistics::ValueStatistics::max_opt` · parquet 59.3.0

```rust
fn max_opt(&self) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:589`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns max value of the statistics, if known.

<a id="op-ba8e8b37dbb763bcac4a0a10"></a>
## min_bytes_opt

`function` · `parquet::file::statistics::ValueStatistics::min_bytes_opt` · parquet 59.3.0

```rust
fn min_bytes_opt(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::data_type::AsBytes", "path": "AsBytes"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [649, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:641`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns min value as bytes of the statistics, if min value is known.

<a id="op-f9d64f05d13b80b86e00527b"></a>
## min_is_exact

`function` · `parquet::file::statistics::ValueStatistics::min_is_exact` · parquet 59.3.0

```rust
fn min_is_exact(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:605`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Whether or not min value is set, and is an exact value.

<a id="op-0c5d3b6d7d5ad9a87940994d"></a>
## min_opt

`function` · `parquet::file::statistics::ValueStatistics::min_opt` · parquet 59.3.0

```rust
fn min_opt(&self) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:584`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns min value of the statistics, if known.

<a id="op-763a56ea332bf8249e1c178c"></a>
## new

`function` · `parquet::file::statistics::ValueStatistics::new` · parquet 59.3.0

```rust
fn new(min: Option<T>, max: Option<T>, distinct_count: Option<u64>, null_count: Option<u64>, is_min_max_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:530`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new typed statistics.

<a id="op-5a1b2dede8198a5e638862a7"></a>
## null_count_opt

`function` · `parquet::file::statistics::ValueStatistics::null_count_opt` · parquet 59.3.0

```rust
fn null_count_opt(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:615`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns null count.

<a id="op-5f9907c467398bfde38bd3d0"></a>
## with_backwards_compatible_min_max

`function` · `parquet::file::statistics::ValueStatistics::with_backwards_compatible_min_max` · parquet 59.3.0

```rust
fn with_backwards_compatible_min_max(self, backwards_compatible: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:576`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set whether to write the deprecated `min` and `max` fields
for compatibility with older parquet writers

This should only be enabled if the field is signed,
see [`Self::is_min_max_backwards_compatible`](../operations/parquet.file.statistics.ValueStatistics.md#op-d6ffbdf485b781a786292dff)

<a id="op-bf5bc1c8ff19a040aa9ac072"></a>
## with_max_is_exact

`function` · `parquet::file::statistics::ValueStatistics::with_max_is_exact` · parquet 59.3.0

```rust
fn with_max_is_exact(self, is_max_value_exact: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:564`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set whether the stored `max` field represents the exact
maximum, or just a bound on the maximum value.

see [`Self::max_is_exact`](../operations/parquet.file.statistics.ValueStatistics.md#op-5e3bad1973bf6a4e718e92b2)

<a id="op-cbb5cdbb2590bf7b7356ff01"></a>
## with_min_is_exact

`function` · `parquet::file::statistics::ValueStatistics::with_min_is_exact` · parquet 59.3.0

```rust
fn with_min_is_exact(self, is_min_value_exact: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [637, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:553`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set whether the stored `min` field represents the exact
minimum, or just a bound on the minimum value.

see [`Self::min_is_exact`](../operations/parquet.file.statistics.ValueStatistics.md#op-f9d64f05d13b80b86e00527b)
