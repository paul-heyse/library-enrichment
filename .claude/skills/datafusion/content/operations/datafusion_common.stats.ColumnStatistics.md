# `datafusion_common::stats::ColumnStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.stats.ColumnStatistics.json).

<a id="op-0684a7f4d4e937976b526c06"></a>
## ColumnStatistics

`struct` · `datafusion_common::stats::ColumnStatistics` · datafusion-common 55.1.0

```rust
struct ColumnStatistics
```

Source: `src/stats.rs:1070`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Statistics for a column within a relation

<a id="op-a25116c85e74060ba1b12979"></a>
## byte_size

`struct_field` · `datafusion_common::stats::ColumnStatistics::byte_size` · datafusion-common 55.1.0

```rust
byte_size: Precision<usize>
```

Source: `src/stats.rs:1103`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Estimated size of this column's data in bytes for the output.

Note that this is not the same as the total bytes that may be scanned,
processed, etc.

E.g. we may read 1GB of data from a Parquet file but the Arrow data
the node produces may be 2GB; it's this 2GB that is tracked here.

Currently this is accurately calculated for primitive types only.
For complex types (like Utf8, List, Struct, etc), this value may be
absent or inexact (e.g. estimated from the size of the data in the source Parquet files).

This value is automatically scaled when operations like limits or
filters reduce the number of rows (see [`Statistics::with_fetch`](../operations/datafusion_common.stats.Statistics.md#op-766dfbe142bf075a2c6281c4)).

<a id="op-f0fdd39da3a28a4297dccd2f"></a>
## clone

`function` · `datafusion_common::stats::ColumnStatistics::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ColumnStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 10], "end": [1069, 15], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/stats.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3195385a4e5f946bd48579b5"></a>
## default

`function` · `datafusion_common::stats::ColumnStatistics::default` · datafusion-common 55.1.0

```rust
fn default() -> ColumnStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 39], "end": [1069, 46], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/stats.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1670aa97872b620ba05d16fa"></a>
## distinct_count

`struct_field` · `datafusion_common::stats::ColumnStatistics::distinct_count` · datafusion-common 55.1.0

```rust
distinct_count: Precision<usize>
```

Source: `src/stats.rs:1088`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of distinct values

<a id="op-d33bca71728f3fc8ab2742d8"></a>
## eq

`function` · `datafusion_common::stats::ColumnStatistics::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ColumnStatistics) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 24], "end": [1069, 33], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/stats.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a15782166891460e8c8fceb"></a>
## fmt

`function` · `datafusion_common::stats::ColumnStatistics::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 17], "end": [1069, 22], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stats.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-690f7eb0d17a691db721d463"></a>
## heap_size

`function` · `datafusion_common::stats::ColumnStatistics::heap_size` · datafusion-common 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "crate::ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [126, 2], "filename": "src/heap_size.rs"}, "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}, "trait_path": "datafusion_common::heap_size::DFHeapSize"}`

Source: `src/heap_size.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbf4ad78bb2ad42c4b4df8ac"></a>
## is_singleton

`function` · `datafusion_common::stats::ColumnStatistics::is_singleton` · datafusion-common 55.1.0

```rust
fn is_singleton(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1108`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Column contains a single non null value (e.g constant).

<a id="op-6a99e78a5f97ba00dd15b060"></a>
## max_value

`struct_field` · `datafusion_common::stats::ColumnStatistics::max_value` · datafusion-common 55.1.0

```rust
max_value: Precision<ScalarValue>
```

Source: `src/stats.rs:1074`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maximum value of column

<a id="op-a8094a2babe8220951790cb6"></a>
## min_value

`struct_field` · `datafusion_common::stats::ColumnStatistics::min_value` · datafusion-common 55.1.0

```rust
min_value: Precision<ScalarValue>
```

Source: `src/stats.rs:1076`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Minimum value of column

<a id="op-5b8509eb27131030b4d8433c"></a>
## new_unknown

`function` · `datafusion_common::stats::ColumnStatistics::new_unknown` · datafusion-common 55.1.0

```rust
fn new_unknown() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1119`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ColumnStatistics`](../operations/datafusion_common.stats.ColumnStatistics.md#op-0684a7f4d4e937976b526c06) instance having all [`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448) parameters.

<a id="op-86dece8d73aa45425664e487"></a>
## null_count

`struct_field` · `datafusion_common::stats::ColumnStatistics::null_count` · datafusion-common 55.1.0

```rust
null_count: Precision<usize>
```

Source: `src/stats.rs:1072`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of null values on column

<a id="op-97bfde544b15f0785ab2a60a"></a>
## sum_value

`struct_field` · `datafusion_common::stats::ColumnStatistics::sum_value` · datafusion-common 55.1.0

```rust
sum_value: Precision<ScalarValue>
```

Source: `src/stats.rs:1086`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sum value of a column.

For integral columns, values should be kept in SUM-compatible widened
types (`Int8/Int16/Int32 -> Int64`, `UInt8/UInt16/UInt32 -> UInt64`) to
reduce overflow risk during statistics propagation.

Callers should prefer [`ColumnStatistics::with_sum_value`](../operations/datafusion_common.stats.ColumnStatistics.md#op-0528b09fb6cc1796d1f14621) for setting
this field and [`Precision<ScalarValue>::add_for_sum`](../operations/datafusion_common.stats.Precision.md#op-0e066186dd7732034421f898) /
[`Precision<ScalarValue>::cast_to_sum_type`](../operations/datafusion_common.stats.Precision.md#op-e08d62f219feb17e175161b4) for sum arithmetic.

<a id="op-0224220c36488bd56d187980"></a>
## to_inexact

`function` · `datafusion_common::stats::ColumnStatistics::to_inexact` · datafusion-common 55.1.0

```rust
fn to_inexact(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1182`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If the exactness of a [`ColumnStatistics`](../operations/datafusion_common.stats.ColumnStatistics.md#op-0684a7f4d4e937976b526c06) instance is lost, this
function relaxes the exactness of all information by converting them
[`Precision::Inexact`](../operations/datafusion_common.stats.Precision.md#op-5d87555abc98461943c3ed02).

<a id="op-b377276b62c398a760dbe523"></a>
## with_byte_size

`function` · `datafusion_common::stats::ColumnStatistics::with_byte_size` · datafusion-common 55.1.0

```rust
fn with_byte_size(self, byte_size: Precision<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1174`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the scan byte size
This should initially be set to the total size of the column.

<a id="op-901f4f851c3c1f7fe1507a46"></a>
## with_distinct_count

`function` · `datafusion_common::stats::ColumnStatistics::with_distinct_count` · datafusion-common 55.1.0

```rust
fn with_distinct_count(self, distinct_count: Precision<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1167`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the distinct count

<a id="op-2e92b626d8a67da6418bca09"></a>
## with_max_value

`function` · `datafusion_common::stats::ColumnStatistics::with_max_value` · datafusion-common 55.1.0

```rust
fn with_max_value(self, max_value: Precision<ScalarValue>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1137`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the max value

<a id="op-9f78c7e3fbbb2a7edfb02c4b"></a>
## with_min_value

`function` · `datafusion_common::stats::ColumnStatistics::with_min_value` · datafusion-common 55.1.0

```rust
fn with_min_value(self, min_value: Precision<ScalarValue>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1143`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the min value

<a id="op-760ad145d17906091436b3e4"></a>
## with_null_count

`function` · `datafusion_common::stats::ColumnStatistics::with_null_count` · datafusion-common 55.1.0

```rust
fn with_null_count(self, null_count: Precision<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1131`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the null count

<a id="op-0528b09fb6cc1796d1f14621"></a>
## with_sum_value

`function` · `datafusion_common::stats::ColumnStatistics::with_sum_value` · datafusion-common 55.1.0

```rust
fn with_sum_value(self, sum_value: Precision<ScalarValue>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::ColumnStatistics", "path": "ColumnStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1191, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:1149`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the sum value
