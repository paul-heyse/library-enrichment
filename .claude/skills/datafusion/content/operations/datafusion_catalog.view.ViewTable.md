# `datafusion_catalog::view::ViewTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.view.ViewTable.json).

<a id="op-bfab6ebe5a11caa5667b8bcd"></a>
## ViewTable

`struct` · `datafusion_catalog::view::ViewTable` · datafusion-catalog 55.1.0

```rust
struct ViewTable
```

Source: `src/view.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

An implementation of `TableProvider` that uses another logical plan.

<a id="op-1650dd883beae7febeece06c"></a>
## definition

`function` · `datafusion_catalog::view::ViewTable::definition` · datafusion-catalog 55.1.0

```rust
fn definition(&self) -> Option<&String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [71, 2], "filename": "src/view.rs"}, "trait": null, "trait_path": null}`

Source: `src/view.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Get definition ref

<a id="op-cce8e3119d465e9147c482a2"></a>
## fmt

`function` · `datafusion_catalog::view::ViewTable::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/view.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/view.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd1f09e187a1f042904d1657"></a>
## get_logical_plan

`function` · `datafusion_catalog::view::ViewTable::get_logical_plan` · datafusion-catalog 55.1.0

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [140, 2], "filename": "src/view.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/view.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45ac8daaff3a0f152b1a0dd6"></a>
## get_table_definition

`function` · `datafusion_catalog::view::ViewTable::get_table_definition` · datafusion-catalog 55.1.0

```rust
fn get_table_definition(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [140, 2], "filename": "src/view.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/view.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8dfffe2207b8fd790e32444"></a>
## logical_plan

`function` · `datafusion_catalog::view::ViewTable::logical_plan` · datafusion-catalog 55.1.0

```rust
fn logical_plan(&self) -> &LogicalPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [71, 2], "filename": "src/view.rs"}, "trait": null, "trait_path": null}`

Source: `src/view.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Get logical_plan ref

<a id="op-7369c2464a2fc34c5d4f99b7"></a>
## new

`function` · `datafusion_catalog::view::ViewTable::new` · datafusion-catalog 55.1.0

```rust
fn new(logical_plan: LogicalPlan, definition: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [71, 2], "filename": "src/view.rs"}, "trait": null, "trait_path": null}`

Source: `src/view.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Create new view that is executed at query runtime.

Takes a `LogicalPlan` and optionally the SQL text of the `CREATE`
statement.

Notes: the `LogicalPlan` is not validated or type coerced. If this is
needed it should be done after calling this function.

<a id="op-b765bbec85a3572bd6c527d8"></a>
## scan

`function` · `datafusion_catalog::view::ViewTable::scan` · datafusion-catalog 55.1.0

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [140, 2], "filename": "src/view.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/view.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53e386c85c01cb8bec8afdf2"></a>
## schema

`function` · `datafusion_catalog::view::ViewTable::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [140, 2], "filename": "src/view.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/view.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e850fc95c40bdee2a55eeb8"></a>
## supports_filters_pushdown

`function` · `datafusion_catalog::view::ViewTable::supports_filters_pushdown` · datafusion-catalog 55.1.0

```rust
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [140, 2], "filename": "src/view.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/view.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57629c5b2434d7d9090487b1"></a>
## table_type

`function` · `datafusion_catalog::view::ViewTable::table_type` · datafusion-catalog 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::view::ViewTable", "path": "ViewTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [140, 2], "filename": "src/view.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/view.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
