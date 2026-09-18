# `datafusion_expr_common::statistics::StatisticsRequest`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.StatisticsRequest.json).

<a id="op-5cdfa28c892e72d3ef275234"></a>
## StatisticsRequest

`enum` · `datafusion_expr_common::statistics::StatisticsRequest` · datafusion-expr-common 55.1.0

```rust
enum StatisticsRequest
```

Source: `src/statistics.rs:1722`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A statistic a caller would like a provider to supply, if it can do so
cheaply.

A small, query-aware extension to the existing `Statistics` model: instead
of "give me everything you have for every column", a caller can ask for a
specific list of stats by name. `StatisticsRequest` is just that vocabulary
— DataFusion itself does not populate or consume it. It exists so a request
can be threaded from a `TableScan` (see `TableScan::statistics_requests`)
through `ScanArgs::statistics_requests` to a `TableProvider`, which is enough
for a query-aware statistics feature to be implemented outside of DataFusion.

Each variant maps onto a field of [`datafusion_common::Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) /
[`datafusion_common::ColumnStatistics`](../operations/datafusion_common.stats.ColumnStatistics.md#op-0684a7f4d4e937976b526c06), so a provider that already
populates one can answer the request trivially.

The per-column variants hold an `Arc<Column>` rather than an owned
[`Column`](../operations/datafusion_common.column.Column.md#op-099cc6d1a52c20c065bf8bc6) (which carries owned strings) so cloning a request — and the
`BTreeSet<StatisticsRequest>` stored on `TableScan`, which is cloned with
the plan during optimization — stays cheap.

<a id="op-6bd5dd8f30da00af17d623d5"></a>
## ByteSize

`variant` · `datafusion_expr_common::statistics::StatisticsRequest::ByteSize` · datafusion-expr-common 55.1.0

```rust
ByteSize
```

Source: `src/statistics.rs:1735`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Encoded/output byte size of `column`.

<a id="op-3d8deb2ef18c774314f9f781"></a>
## DistinctCount

`variant` · `datafusion_expr_common::statistics::StatisticsRequest::DistinctCount` · datafusion-expr-common 55.1.0

```rust
DistinctCount
```

Source: `src/statistics.rs:1730`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Number of distinct values in `column` (exact or estimated).

<a id="op-e8895a8b2f0a327e09943aac"></a>
## Max

`variant` · `datafusion_expr_common::statistics::StatisticsRequest::Max` · datafusion-expr-common 55.1.0

```rust
Max
```

Source: `src/statistics.rs:1726`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Largest non-null value of `column`.

<a id="op-110cf18c8875e789065f7016"></a>
## Min

`variant` · `datafusion_expr_common::statistics::StatisticsRequest::Min` · datafusion-expr-common 55.1.0

```rust
Min
```

Source: `src/statistics.rs:1724`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Smallest non-null value of `column`.

<a id="op-278236785728257dd4157e30"></a>
## NullCount

`variant` · `datafusion_expr_common::statistics::StatisticsRequest::NullCount` · datafusion-expr-common 55.1.0

```rust
NullCount
```

Source: `src/statistics.rs:1728`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Number of NULLs in `column`.

<a id="op-5c17f5f8551ba57b6d7d1fb3"></a>
## RowCount

`variant` · `datafusion_expr_common::statistics::StatisticsRequest::RowCount` · datafusion-expr-common 55.1.0

```rust
RowCount
```

Source: `src/statistics.rs:1737`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Number of rows in the container (table / file).

<a id="op-07a38998dcc9762740f70c33"></a>
## Sum

`variant` · `datafusion_expr_common::statistics::StatisticsRequest::Sum` · datafusion-expr-common 55.1.0

```rust
Sum
```

Source: `src/statistics.rs:1733`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Sum of values in `column` (numerics, widened per
`ColumnStatistics::sum_value`).

<a id="op-7a5d99833ec1b1f3b5ba36b6"></a>
## TotalByteSize

`variant` · `datafusion_expr_common::statistics::StatisticsRequest::TotalByteSize` · datafusion-expr-common 55.1.0

```rust
TotalByteSize
```

Source: `src/statistics.rs:1739`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Total byte size of the container's output.

<a id="op-f8bdf180fe6857c307edbd6d"></a>
## clone

`function` · `datafusion_expr_common::statistics::StatisticsRequest::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> StatisticsRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::StatisticsRequest", "path": "StatisticsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1721, 17], "end": [1721, 22], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:1721`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-809ef1b17466d776cfc1b798"></a>
## cmp

`function` · `datafusion_expr_common::statistics::StatisticsRequest::cmp` · datafusion-expr-common 55.1.0

```rust
fn cmp(&self, other: &StatisticsRequest) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::StatisticsRequest", "path": "StatisticsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1721, 51], "end": [1721, 54], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/statistics.rs:1721`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ec81d801582d2fa94a9e6cf"></a>
## eq

`function` · `datafusion_expr_common::statistics::StatisticsRequest::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &StatisticsRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::StatisticsRequest", "path": "StatisticsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1721, 24], "end": [1721, 33], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/statistics.rs:1721`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c181cdb304f1ca1cf99e166b"></a>
## fmt

`function` · `datafusion_expr_common::statistics::StatisticsRequest::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::StatisticsRequest", "path": "StatisticsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1721, 10], "end": [1721, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:1721`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5911121fa84c72f23d9da401"></a>
## partial_cmp

`function` · `datafusion_expr_common::statistics::StatisticsRequest::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &StatisticsRequest) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::StatisticsRequest", "path": "StatisticsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1721, 39], "end": [1721, 49], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/statistics.rs:1721`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
