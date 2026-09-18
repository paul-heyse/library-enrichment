# `datafusion_catalog::empty::EmptyTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.empty.EmptyTable.json).

<a id="op-2c4e3066473d0e4a64f71738"></a>
## EmptyTable

`struct` · `datafusion_catalog::empty::EmptyTable` · datafusion-catalog 55.1.0

```rust
struct EmptyTable
```

Source: `src/empty.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

An empty plan that is useful for testing and generating plans
without mapping them to actual data.

<a id="op-9f269c8b63a00579fe4c328d"></a>
## fmt

`function` · `datafusion_catalog::empty::EmptyTable::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::empty::EmptyTable", "path": "EmptyTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/empty.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/empty.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14eb151f8aa184dd413929b1"></a>
## new

`function` · `datafusion_catalog::empty::EmptyTable::new` · datafusion-catalog 55.1.0

```rust
fn new(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::empty::EmptyTable", "path": "EmptyTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [54, 2], "filename": "src/empty.rs"}, "trait": null, "trait_path": null}`

Source: `src/empty.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Initialize a new `EmptyTable` from a schema.

<a id="op-a7d83b41bcb5b52d181d605f"></a>
## scan

`function` · `datafusion_catalog::empty::EmptyTable::scan` · datafusion-catalog 55.1.0

```rust
async fn scan(&self, _state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], _limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::empty::EmptyTable", "path": "EmptyTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [79, 2], "filename": "src/empty.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/empty.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffaa5c2a7bacd892d46d979e"></a>
## schema

`function` · `datafusion_catalog::empty::EmptyTable::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::empty::EmptyTable", "path": "EmptyTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [79, 2], "filename": "src/empty.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/empty.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92482174feeab8306b957522"></a>
## table_type

`function` · `datafusion_catalog::empty::EmptyTable::table_type` · datafusion-catalog 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::empty::EmptyTable", "path": "EmptyTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [79, 2], "filename": "src/empty.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/empty.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0d8b546140ba2725ee2fa85"></a>
## with_partitions

`function` · `datafusion_catalog::empty::EmptyTable::with_partitions` · datafusion-catalog 55.1.0

```rust
fn with_partitions(self, partitions: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::empty::EmptyTable", "path": "EmptyTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [54, 2], "filename": "src/empty.rs"}, "trait": null, "trait_path": null}`

Source: `src/empty.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Creates a new EmptyTable with specified partition number.
