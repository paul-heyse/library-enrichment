# `datafusion_catalog::stream::StreamTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.stream.StreamTable.json).

<a id="op-4f9e242f2c18b302a51a1b01"></a>
## StreamTable

`struct` · `datafusion_catalog::stream::StreamTable` · datafusion-catalog 55.1.0

```rust
struct StreamTable
```

Source: `src/stream.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

A [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) for an unbounded stream source

Currently only reading from / appending to a single file in-place is supported, but
other stream sources and sinks may be added in future.

Applications looking to read/write datasets comprising multiple files, e.g. [Hadoop]-style
data stored in object storage, should instead consider [`ListingTable`].

[Hadoop]: https://hadoop.apache.org/
[`ListingTable`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTable.html

<a id="op-b5e3ddcdf7fca1d65c418cde"></a>
## constraints

`function` · `datafusion_catalog::stream::StreamTable::constraints` · datafusion-catalog 55.1.0

```rust
fn constraints(&self) -> Option<&Constraints>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTable", "path": "StreamTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [372, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/stream.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e209581a8e7ff892e87eabc"></a>
## fmt

`function` · `datafusion_catalog::stream::StreamTable::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTable", "path": "StreamTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 10], "end": [301, 15], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stream.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-649dd80a6865ba2a9ca04481"></a>
## insert_into

`function` · `datafusion_catalog::stream::StreamTable::insert_into` · datafusion-catalog 55.1.0

```rust
async fn insert_into(&self, _state: &dyn Session, input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTable", "path": "StreamTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [372, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/stream.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbef08b4c4c9e1cdbeb5f070"></a>
## new

`function` · `datafusion_catalog::stream::StreamTable::new` · datafusion-catalog 55.1.0

```rust
fn new(config: Arc<StreamConfig>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTable", "path": "StreamTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [309, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Create a new [`StreamTable`](../operations/datafusion_catalog.stream.StreamTable.md#op-4f9e242f2c18b302a51a1b01) for the given [`StreamConfig`](../operations/datafusion_catalog.stream.StreamConfig.md#op-5a9c0db207927628f8881b75)

<a id="op-fc168033351dfd3e7bae3834"></a>
## scan

`function` · `datafusion_catalog::stream::StreamTable::scan` · datafusion-catalog 55.1.0

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTable", "path": "StreamTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [372, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/stream.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-751d8f835e0032407a33217e"></a>
## schema

`function` · `datafusion_catalog::stream::StreamTable::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTable", "path": "StreamTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [372, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/stream.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0cfd21a21064620d82ca67f"></a>
## table_type

`function` · `datafusion_catalog::stream::StreamTable::table_type` · datafusion-catalog 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTable", "path": "StreamTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [372, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/stream.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
