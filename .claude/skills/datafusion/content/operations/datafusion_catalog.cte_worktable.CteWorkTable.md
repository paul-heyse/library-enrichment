# `datafusion_catalog::cte_worktable::CteWorkTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.cte_worktable.CteWorkTable.json).

<a id="op-5bf64b25bdc1d9321d44032a"></a>
## CteWorkTable

`struct` · `datafusion_catalog::cte_worktable::CteWorkTable` · datafusion-catalog 55.1.0

```rust
struct CteWorkTable
```

Source: `src/cte_worktable.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

The temporary working table where the previous iteration of a recursive query is stored
Naming is based on PostgreSQL's implementation.
See here for more details: www.postgresql.org/docs/11/queries-with.html#id-1.5.6.12.5.4

<a id="op-275437a5ddb5ab56f5dcbaa1"></a>
## fmt

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/cte_worktable.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cte_worktable.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30e7e265056af3ab3b8538d1"></a>
## get_logical_plan

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::get_logical_plan` · datafusion-catalog 55.1.0

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [117, 2], "filename": "src/cte_worktable.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/cte_worktable.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ad377c8c1036c5be2164a0c"></a>
## name

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::name` · datafusion-catalog 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [65, 2], "filename": "src/cte_worktable.rs"}, "trait": null, "trait_path": null}`

Source: `src/cte_worktable.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

The user-provided name of the CTE

<a id="op-827286e284f1e084cd40b1f7"></a>
## new

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::new` · datafusion-catalog 55.1.0

```rust
fn new(name: &str, table_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [65, 2], "filename": "src/cte_worktable.rs"}, "trait": null, "trait_path": null}`

Source: `src/cte_worktable.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Construct a new CteWorkTable with the given name and self-reference schema.

<a id="op-453c90604e62bdebd9b649b3"></a>
## scan

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::scan` · datafusion-catalog 55.1.0

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [117, 2], "filename": "src/cte_worktable.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/cte_worktable.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92f4988491240c0b32174c9f"></a>
## scan_with_args

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::scan_with_args` · datafusion-catalog 55.1.0

```rust
async fn scan_with_args<'a>(&self, _state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [117, 2], "filename": "src/cte_worktable.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/cte_worktable.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1788275ae489a4050e8459dc"></a>
## schema

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [65, 2], "filename": "src/cte_worktable.rs"}, "trait": null, "trait_path": null}`

Source: `src/cte_worktable.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

The schema exposed by scans of the recursive self-reference.

<a id="op-d0d232d32b25122d63d29a2e"></a>
## schema

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [117, 2], "filename": "src/cte_worktable.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/cte_worktable.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-113ac2ac89626d4f34bfa395"></a>
## supports_filters_pushdown

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::supports_filters_pushdown` · datafusion-catalog 55.1.0

```rust
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [117, 2], "filename": "src/cte_worktable.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/cte_worktable.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e69b82914eddffb6fa07ea"></a>
## table_type

`function` · `datafusion_catalog::cte_worktable::CteWorkTable::table_type` · datafusion-catalog 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::cte_worktable::CteWorkTable", "path": "CteWorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [117, 2], "filename": "src/cte_worktable.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/cte_worktable.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
