# `datafusion_session::table::ScanArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.table.ScanArgs.json).

<a id="op-4b7e85b7401a64fe31c9b2cc"></a>
## ScanArgs

`struct` · `datafusion_session::table::ScanArgs` · datafusion-session 55.1.0

```rust
struct ScanArgs<'a>
```

Source: `src/table.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Arguments for scanning a table with [`TableProvider::scan_with_args`](../operations/datafusion_session.table.TableProvider.md#op-274938cecb923835571bdb77).

<a id="op-2a1be89695a4c0703ef171cd"></a>
## clone

`function` · `datafusion_session::table::ScanArgs::clone` · datafusion-session 55.1.0

```rust
fn clone(&self) -> ScanArgs<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 17], "end": [427, 22], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aeecff156e3369f616b66cd1"></a>
## default

`function` · `datafusion_session::table::ScanArgs::default` · datafusion-session 55.1.0

```rust
fn default() -> ScanArgs<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 24], "end": [427, 31], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/table.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cca52f6bd6125d3e5b1e7e64"></a>
## filters

`function` · `datafusion_session::table::ScanArgs::filters` · datafusion-session 55.1.0

```rust
fn filters(&self) -> Option<&'a [Expr]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [520, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the filter expressions for the scan.

Returns a reference to the filter expressions, or `None` if no filters were specified.

<a id="op-71c63d4c773f861b33d4b303"></a>
## fmt

`function` · `datafusion_session::table::ScanArgs::fmt` · datafusion-session 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 10], "end": [427, 15], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73a7f78fd0742f9bc2e53f48"></a>
## limit

`function` · `datafusion_session::table::ScanArgs::limit` · datafusion-session 55.1.0

```rust
fn limit(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [520, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the maximum number of rows to return from the scan.

Returns the row limit, or `None` if no limit was specified.

<a id="op-88ffb2dc0fb61bd44d4c6829"></a>
## projection

`function` · `datafusion_session::table::ScanArgs::projection` · datafusion-session 55.1.0

```rust
fn projection(&self) -> Option<&'a [usize]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [520, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the column projection for the scan.

Returns a reference to the projection column indices, or `None` if
no projection was specified (meaning all columns should be included).

<a id="op-03573381d33756a5441d35ca"></a>
## statistics_requests

`function` · `datafusion_session::table::ScanArgs::statistics_requests` · datafusion-session 55.1.0

```rust
fn statistics_requests(&self) -> &'a [StatisticsRequest]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [520, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:517`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the statistics requests for the scan. Empty if none were set.

See [`Self::with_statistics_requests`](../operations/datafusion_session.table.ScanArgs.md#op-e38237ea5ae2f46a64559ff1) for more details

<a id="op-25c13e28cbc100e3bb55a8fa"></a>
## with_filters

`function` · `datafusion_session::table::ScanArgs::with_filters` · datafusion-session 55.1.0

```rust
fn with_filters(self, filters: Option<&'a [Expr]>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [520, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Set the filter expressions for the scan.

Filters are boolean expressions that should be evaluated during the scan
to reduce the number of rows returned. All expressions are combined with AND logic.
Whether filters are actually pushed down depends on [`TableProvider::supports_filters_pushdown`](../operations/datafusion_session.table.TableProvider.md#op-a929088704b5681a65528134).

# Arguments
* `filters` - Optional slice of filter expressions

<a id="op-b6385eba158bd5cd64143545"></a>
## with_limit

`function` · `datafusion_session::table::ScanArgs::with_limit` · datafusion-session 55.1.0

```rust
fn with_limit(self, limit: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [520, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Set the maximum number of rows to return from the scan.

If specified, the scan should return at most this many rows. This is typically
used to optimize queries with `LIMIT` clauses.

# Arguments
* `limit` - Optional maximum number of rows to return

<a id="op-bc4040e5a9cf81d561f8ea70"></a>
## with_projection

`function` · `datafusion_session::table::ScanArgs::with_projection` · datafusion-session 55.1.0

```rust
fn with_projection(self, projection: Option<&'a [usize]>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [520, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Set the column projection for the scan.

The projection is a list of column indices from [`TableProvider::schema`](../operations/datafusion_session.table.TableProvider.md#op-61d424ca40b676444809c002)
that should be included in the scan results. If `None`, all columns are included.

# Arguments
* `projection` - Optional slice of column indices to project

<a id="op-e38237ea5ae2f46a64559ff1"></a>
## with_statistics_requests

`function` · `datafusion_session::table::ScanArgs::with_statistics_requests` · datafusion-session 55.1.0

```rust
fn with_statistics_requests(self, statistics_requests: &'a [StatisticsRequest]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_session::table::ScanArgs", "path": "ScanArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [520, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Specifies the statistics the caller may use when optimizing the query.

This is intended to allow the `TableProvider` to cheaply provide
statistics that may help, such as those it has in an in-memory catalog
or from some other metadata source.

`TableProvider`s read these via [`Self::statistics_requests()`](../operations/datafusion_session.table.ScanArgs.md#op-03573381d33756a5441d35ca); anything
a `TableProvider` cannot answer cheaply it simply ignores. DataFusion's
own `TableProvider`s ignore this field — it exists so a request can be
threaded from a custom optimizer rule (which annotates
`TableScan::statistics_requests`) through to a custom `TableProvider`.
