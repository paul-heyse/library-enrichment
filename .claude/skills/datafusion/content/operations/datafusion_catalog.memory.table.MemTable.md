# `datafusion_catalog::memory::table::MemTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.memory.table.MemTable.json).

<a id="op-edca0df891fe36e3d9bd9dea"></a>
## MemTable

`struct` · `datafusion_catalog::memory::table::MemTable` · datafusion-catalog 55.1.0

```rust
struct MemTable
```

Source: `src/memory/table.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

In-memory data source for presenting a `Vec<RecordBatch>` as a
data source that can be queried by DataFusion. This allows data to
be pre-loaded into memory and then repeatedly queried without
incurring additional file I/O overhead.

<a id="op-3375c6762af78fc23e89e3a6"></a>
## batches

`struct_field` · `datafusion_catalog::memory::table::MemTable::batches` · datafusion-catalog 55.1.0

```rust
batches: Vec<PartitionData>
```

Source: `src/memory/table.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-488c6d7e899576d88aaa40d5"></a>
## constraints

`function` · `datafusion_catalog::memory::table::MemTable::constraints` · datafusion-catalog 55.1.0

```rust
fn constraints(&self) -> Option<&Constraints>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [465, 2], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/memory/table.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afaec01f85d04ac7bfe721bb"></a>
## delete_from

`function` · `datafusion_catalog::memory::table::MemTable::delete_from` · datafusion-catalog 55.1.0

```rust
async fn delete_from(&self, state: &dyn Session, filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [465, 2], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/memory/table.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1bc666648241a6193472b3a"></a>
## fmt

`function` · `datafusion_catalog::memory::table::MemTable::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory/table.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5162273a919542603fade3ac"></a>
## get_column_default

`function` · `datafusion_catalog::memory::table::MemTable::get_column_default` · datafusion-catalog 55.1.0

```rust
fn get_column_default(&self, column: &str) -> Option<&Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [465, 2], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/memory/table.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e203ad7f4d280ebb52f6963e"></a>
## insert_into

`function` · `datafusion_catalog::memory::table::MemTable::insert_into` · datafusion-catalog 55.1.0

```rust
async fn insert_into(&self, _state: &dyn Session, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [465, 2], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/memory/table.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Returns an ExecutionPlan that inserts the execution results of a given [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) into this [`MemTable`](../operations/datafusion_catalog.memory.table.MemTable.md#op-edca0df891fe36e3d9bd9dea).

The [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) must have the same schema as this [`MemTable`](../operations/datafusion_catalog.memory.table.MemTable.md#op-edca0df891fe36e3d9bd9dea).

# Arguments

* `state` - The [`SessionState`] containing the context for executing the plan.
* `input` - The [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) to execute and insert.

# Returns

* A plan that returns the number of rows written.

[`SessionState`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html

<a id="op-017310644d33bb18fc964646"></a>
## load

`function` · `datafusion_catalog::memory::table::MemTable::load` · datafusion-catalog 55.1.0

```rust
async fn load(t: Arc<dyn TableProvider>, output_partitions: Option<usize>, state: &dyn Session) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [171, 2], "filename": "src/memory/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory/table.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Create a mem table by reading from another data source

<a id="op-fb9612c84476b53944c829f5"></a>
## scan

`function` · `datafusion_catalog::memory::table::MemTable::scan` · datafusion-catalog 55.1.0

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], _limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [465, 2], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/memory/table.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-314815a1b085a1ee1f6b1e52"></a>
## schema

`function` · `datafusion_catalog::memory::table::MemTable::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [465, 2], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/memory/table.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed457598d4f5c98a6476f1fd"></a>
## sort_order

`struct_field` · `datafusion_catalog::memory::table::MemTable::sort_order` · datafusion-catalog 55.1.0

```rust
sort_order: std::sync::Arc<parking_lot::Mutex<Vec<Vec<datafusion_expr::SortExpr>>>>
```

Source: `src/memory/table.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Optional pre-known sort order(s). Must be `SortExpr`s.
inserting data into this table removes the order

<a id="op-c8e03c42a50c0a91bda47785"></a>
## table_type

`function` · `datafusion_catalog::memory::table::MemTable::table_type` · datafusion-catalog 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [465, 2], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/memory/table.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5a86b7b4c9fdc46bcbce229"></a>
## try_new

`function` · `datafusion_catalog::memory::table::MemTable::try_new` · datafusion-catalog 55.1.0

```rust
fn try_new(schema: SchemaRef, partitions: Vec<Vec<RecordBatch>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [171, 2], "filename": "src/memory/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory/table.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Create a new in-memory table from the provided schema and record batches.

Requires at least one partition. To construct an empty `MemTable`, pass
`vec![vec![]]` as the `partitions` argument, this represents one partition with
no batches.

<a id="op-9942fadf3c6a28ed8d647707"></a>
## update

`function` · `datafusion_catalog::memory::table::MemTable::update` · datafusion-catalog 55.1.0

```rust
async fn update(&self, state: &dyn Session, assignments: Vec<(String, Expr)>, filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [465, 2], "filename": "src/memory/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/memory/table.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3db445d137107387ab4946a1"></a>
## with_column_defaults

`function` · `datafusion_catalog::memory::table::MemTable::with_column_defaults` · datafusion-catalog 55.1.0

```rust
fn with_column_defaults(self, column_defaults: HashMap<String, Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [171, 2], "filename": "src/memory/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory/table.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Assign column defaults

<a id="op-38c63b0626856e7658c70e79"></a>
## with_constraints

`function` · `datafusion_catalog::memory::table::MemTable::with_constraints` · datafusion-catalog 55.1.0

```rust
fn with_constraints(self, constraints: Constraints) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [171, 2], "filename": "src/memory/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory/table.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Assign constraints

<a id="op-d62a1f5f1a815ead26f0e71e"></a>
## with_sort_order

`function` · `datafusion_catalog::memory::table::MemTable::with_sort_order` · datafusion-catalog 55.1.0

```rust
fn with_sort_order(self, sort_order: Vec<Vec<SortExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::table::MemTable", "path": "MemTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [171, 2], "filename": "src/memory/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory/table.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Specify an optional pre-known sort order(s). Must be `SortExpr`s.

If the data is not sorted by this order, DataFusion may produce
incorrect results.

DataFusion may take advantage of this ordering to omit sorts
or use more efficient algorithms.

Note that multiple sort orders are supported, if some are known to be
equivalent,
