# `datafusion_catalog::streaming::StreamingTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.streaming.StreamingTable.json).

<a id="op-92bc9a605a67efa493f5a3f4"></a>
## StreamingTable

`struct` · `datafusion_catalog::streaming::StreamingTable` · datafusion-catalog 55.1.0

```rust
struct StreamingTable
```

Source: `src/streaming.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

A [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) that streams a set of [`PartitionStream`](../operations/datafusion_physical_plan.streaming.PartitionStream.md#op-477eb3d02543add98f425b63)

<a id="op-c421d0ebfbf9b08fa98b27b2"></a>
## fmt

`function` · `datafusion_catalog::streaming::StreamingTable::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::streaming::StreamingTable", "path": "StreamingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/streaming.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ff5a6d3bc49442706d88898"></a>
## scan

`function` · `datafusion_catalog::streaming::StreamingTable::scan` · datafusion-catalog 55.1.0

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::streaming::StreamingTable", "path": "StreamingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [171, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/streaming.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5191c57692305028f1d844bc"></a>
## schema

`function` · `datafusion_catalog::streaming::StreamingTable::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::streaming::StreamingTable", "path": "StreamingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [171, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/streaming.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae00e883e39c8b52a36c366d"></a>
## table_type

`function` · `datafusion_catalog::streaming::StreamingTable::table_type` · datafusion-catalog 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::streaming::StreamingTable", "path": "StreamingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [171, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/streaming.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1175efc1d3a0795524b06da"></a>
## try_new

`function` · `datafusion_catalog::streaming::StreamingTable::try_new` · datafusion-catalog 55.1.0

```rust
fn try_new(schema: SchemaRef, partitions: Vec<Arc<dyn PartitionStream>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::streaming::StreamingTable", "path": "StreamingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [112, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Try to create a new [`StreamingTable`](../operations/datafusion_catalog.streaming.StreamingTable.md#op-92bc9a605a67efa493f5a3f4) returning an error if the schema is incorrect

<a id="op-d4b183d496efa3cdcf6facdc"></a>
## with_infinite_table

`function` · `datafusion_catalog::streaming::StreamingTable::with_infinite_table` · datafusion-catalog 55.1.0

```rust
fn with_infinite_table(self, infinite: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::streaming::StreamingTable", "path": "StreamingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [112, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Sets streaming table can be infinite.

<a id="op-6be58ab70668ecf413762849"></a>
## with_output_partitioning

`function` · `datafusion_catalog::streaming::StreamingTable::with_output_partitioning` · datafusion-catalog 55.1.0

```rust
fn with_output_partitioning(self, output_partitioning: Partitioning) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::streaming::StreamingTable", "path": "StreamingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [112, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Declares the output partitioning of this streaming table.

The partitioning expressions refer to the table schema before scan
projection. If a scan projection removes a partitioning expression, the
physical plan reports unknown partitioning.

<a id="op-9d441e664ccad6fe3bad6b9b"></a>
## with_sort_order

`function` · `datafusion_catalog::streaming::StreamingTable::with_sort_order` · datafusion-catalog 55.1.0

```rust
fn with_sort_order(self, sort_order: Vec<SortExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::streaming::StreamingTable", "path": "StreamingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [112, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Sets the existing ordering of streaming table.
