# `parquet::file::statistics::Statistics`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.statistics.Statistics.json).

<a id="op-ba51f82bfe4dce01512b0440"></a>
## Statistics

`enum` · `parquet::file::statistics::Statistics` · parquet 59.3.0

```rust
enum Statistics
```

Source: `src/file/statistics.rs:337`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Strongly typed statistics for a column chunk within a row group.

This structure is a natively typed, in memory representation of the thrift
`Statistics` structure in a Parquet file footer. The statistics stored in
this structure can be used by query engines to skip decoding pages while
reading parquet data.

Page level statistics are stored separately, in [ColumnIndexMetaData].

[ColumnIndexMetaData]: crate::file::page_index::column_index::ColumnIndexMetaData

<a id="op-b3c2ec7ff2859be858f3ff33"></a>
## Boolean

`variant` · `parquet::file::statistics::Statistics::Boolean` · parquet 59.3.0

```rust
Boolean
```

Source: `src/file/statistics.rs:339`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Statistics for Boolean column

<a id="op-30b9ff87aea68c8eee982a8d"></a>
## ByteArray

`variant` · `parquet::file::statistics::Statistics::ByteArray` · parquet 59.3.0

```rust
ByteArray
```

Source: `src/file/statistics.rs:351`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Statistics for ByteArray column

<a id="op-c5f4795d17de1d5010993faf"></a>
## Double

`variant` · `parquet::file::statistics::Statistics::Double` · parquet 59.3.0

```rust
Double
```

Source: `src/file/statistics.rs:349`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Statistics for Double column

<a id="op-97df68057aaa195b62022fcc"></a>
## FixedLenByteArray

`variant` · `parquet::file::statistics::Statistics::FixedLenByteArray` · parquet 59.3.0

```rust
FixedLenByteArray
```

Source: `src/file/statistics.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Statistics for FixedLenByteArray column

<a id="op-5dba7f0b48eeed3204e0960b"></a>
## Float

`variant` · `parquet::file::statistics::Statistics::Float` · parquet 59.3.0

```rust
Float
```

Source: `src/file/statistics.rs:347`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Statistics for Float column

<a id="op-305df50136f03cbf02665312"></a>
## Int32

`variant` · `parquet::file::statistics::Statistics::Int32` · parquet 59.3.0

```rust
Int32
```

Source: `src/file/statistics.rs:341`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Statistics for Int32 column

<a id="op-3027854b4f9f5bbae2504ace"></a>
## Int64

`variant` · `parquet::file::statistics::Statistics::Int64` · parquet 59.3.0

```rust
Int64
```

Source: `src/file/statistics.rs:343`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Statistics for Int64 column

<a id="op-a05d1cc87f0a4c76faa4188c"></a>
## Int96

`variant` · `parquet::file::statistics::Statistics::Int96` · parquet 59.3.0

```rust
Int96
```

Source: `src/file/statistics.rs:345`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Statistics for Int96 column

<a id="op-8a5cdebf1c4ea1d92564d8e8"></a>
## boolean

`function` · `parquet::file::statistics::Statistics::boolean` · parquet 59.3.0

```rust
fn boolean(min: Option<bool>, max: Option<bool>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:380`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for `Boolean` column type.

<a id="op-3cef1244a144384ba0e43548"></a>
## byte_array

`function` · `parquet::file::statistics::Statistics::byte_array` · parquet 59.3.0

```rust
fn byte_array(min: Option<ByteArray>, max: Option<ByteArray>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:392`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for `ByteArray` column type.

<a id="op-cda6107c804393efe10ff3e7"></a>
## clone

`function` · `parquet::file::statistics::Statistics::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Statistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 17], "end": [336, 22], "filename": "src/file/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/statistics.rs:336`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d58b0dc771ce7dd4fd13820d"></a>
## distinct_count_opt

`function` · `parquet::file::statistics::Statistics::distinct_count_opt` · parquet 59.3.0

```rust
fn distinct_count_opt(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:426`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns optional value of number of distinct values occurring.
When it is `None`, the value should be ignored.

<a id="op-8a4de1175074198e669ceb82"></a>
## double

`function` · `parquet::file::statistics::Statistics::double` · parquet 59.3.0

```rust
fn double(min: Option<f64>, max: Option<f64>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:390`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for `Double` column type.

<a id="op-89deb04fad4b38bafa91eb4f"></a>
## eq

`function` · `parquet::file::statistics::Statistics::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Statistics) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 24], "end": [336, 33], "filename": "src/file/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/statistics.rs:336`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f77e556f93de63a7bf95bb8"></a>
## fixed_len_byte_array

`function` · `parquet::file::statistics::Statistics::fixed_len_byte_array` · parquet 59.3.0

```rust
fn fixed_len_byte_array(min: Option<FixedLenByteArray>, max: Option<FixedLenByteArray>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:394`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for `FixedLenByteArray` column type.

<a id="op-535d41667bba5ed3e0b768eb"></a>
## float

`function` · `parquet::file::statistics::Statistics::float` · parquet 59.3.0

```rust
fn float(min: Option<f32>, max: Option<f32>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:388`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for `Float` column type.

<a id="op-47d27355075ffe3e351c9b42"></a>
## fmt

`function` · `parquet::file::statistics::Statistics::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 1], "end": [499, 2], "filename": "src/file/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/file/statistics.rs:487`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71f5e65074b35a9f9939c24c"></a>
## fmt

`function` · `parquet::file::statistics::Statistics::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 10], "end": [336, 15], "filename": "src/file/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/statistics.rs:336`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ac9b5e0f93ec1bd9f325c72"></a>
## from

`function` · `parquet::file::statistics::Statistics::from` · parquet 59.3.0

```rust
fn from(t: ValueStatistics<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::data_type::private::ParquetValueType", "path": "ParquetValueType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 1], "end": [360, 2], "filename": "src/file/statistics.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::file::statistics::ValueStatistics", "path": "ValueStatistics"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file/statistics.rs:357`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9965a536a2ba92ebd1c4c098"></a>
## int32

`function` · `parquet::file::statistics::Statistics::int32` · parquet 59.3.0

```rust
fn int32(min: Option<i32>, max: Option<i32>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:382`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for `Int32` column type.

<a id="op-03c091367c2e79dc819f260c"></a>
## int64

`function` · `parquet::file::statistics::Statistics::int64` · parquet 59.3.0

```rust
fn int64(min: Option<i64>, max: Option<i64>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:384`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for `Int64` column type.

<a id="op-62f6fa48f296700af9a1edcf"></a>
## int96

`function` · `parquet::file::statistics::Statistics::int96` · parquet 59.3.0

```rust
fn int96(min: Option<Int96>, max: Option<Int96>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:386`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for `Int96` column type.

<a id="op-6f5b8ced839359f7699fe0cb"></a>
## is_min_max_backwards_compatible

`function` · `parquet::file::statistics::Statistics::is_min_max_backwards_compatible` · parquet 59.3.0

```rust
fn is_min_max_backwards_compatible(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:420`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Old versions of parquet stored statistics in `min` and `max` fields, ordered
using signed comparison. This resulted in an undefined ordering for unsigned
quantities, such as booleans and unsigned integers.

These fields were therefore deprecated in favour of `min_value` and `max_value`,
which have a type-defined sort order.

However, not all readers have been updated. For backwards compatibility, this method
returns `true` if the statistics within this have a signed sort order, that is
compatible with being stored in the deprecated `min` and `max` fields

<a id="op-9c00627b0adf1b9b8ca4fc5d"></a>
## is_min_max_deprecated

`function` · `parquet::file::statistics::Statistics::is_min_max_deprecated` · parquet 59.3.0

```rust
fn is_min_max_deprecated(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:406`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if statistics have old `min` and `max` fields set.
This means that the column order is likely to be undefined, which, for old files
could mean a signed sort order of values.

Refer to [`ColumnOrder`](crate::basic::ColumnOrder) and
[`SortOrder`](crate::basic::SortOrder) for more information.

<a id="op-121ddd9c95312b4e099e6110"></a>
## max_bytes_opt

`function` · `parquet::file::statistics::Statistics::max_bytes_opt` · parquet 59.3.0

```rust
fn max_bytes_opt(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:467`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of bytes that represent max value, if max value is known.

<a id="op-9be8228ef30dbb508f16b015"></a>
## max_is_exact

`function` · `parquet::file::statistics::Statistics::max_is_exact` · parquet 59.3.0

```rust
fn max_is_exact(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:457`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if the max value is set, and is an exact max value.

<a id="op-c32b5a5031892d52391c4d4c"></a>
## min_bytes_opt

`function` · `parquet::file::statistics::Statistics::min_bytes_opt` · parquet 59.3.0

```rust
fn min_bytes_opt(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:462`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of bytes that represent min value, if min value is known.

<a id="op-a9ea445668cb2477b3a80968"></a>
## min_is_exact

`function` · `parquet::file::statistics::Statistics::min_is_exact` · parquet 59.3.0

```rust
fn min_is_exact(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:452`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if the min value is set, and is an exact min value.

<a id="op-335917d0fd05efa2cd1838aa"></a>
## new

`function` · `parquet::file::statistics::Statistics::new` · parquet 59.3.0

```rust
fn new<T: ParquetValueType>(min: Option<T>, max: Option<T>, distinct_count: Option<u64>, null_count: Option<u64>, is_deprecated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:364`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new statistics for a column type

<a id="op-593d5b1a4002b16915b7bd99"></a>
## null_count_opt

`function` · `parquet::file::statistics::Statistics::null_count_opt` · parquet 59.3.0

```rust
fn null_count_opt(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:447`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns number of null values for the column, if known.
Note that this includes all nulls when column is part of the complex type.

Note: Versions of this library prior to `58.1.0` returned `0` if the null count
was not available. This method now returns `None` in that case.

Also, versions of this library prior to `53.1.0` did not store a null count
statistic when the null count was `0`.

It is unsound to assume that missing nullcount stats mean the column contains no nulls,
but code that depends on the old behavior can restore it by defaulting to zero:

```no_run
# use parquet::file::statistics::Statistics;
# let statistics: Statistics = todo!();
let null_count = statistics.null_count_opt().unwrap_or(0);
```

<a id="op-1b0e627d13e31e50c0fb26df"></a>
## physical_type

`function` · `parquet::file::statistics::Statistics::physical_type` · parquet 59.3.0

```rust
fn physical_type(&self) -> Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::statistics::Statistics", "path": "Statistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [484, 2], "filename": "src/file/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/statistics.rs:472`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns physical type associated with statistics.
