# `datafusion_functions_table::generate_series::GenerateSeriesTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_table.generate_series.GenerateSeriesTable.json).

<a id="op-95421324494b3abe99247e2b"></a>
## GenerateSeriesTable

`struct` · `datafusion_functions_table::generate_series::GenerateSeriesTable` · datafusion-functions-table 55.1.0

```rust
struct GenerateSeriesTable
```

Source: `src/generate_series.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Table that generates a series of integers/timestamps from `start`(inclusive) to `end`, incrementing by step

<a id="op-a6fb42989025354db40295e9"></a>
## as_generator

`function` · `datafusion_functions_table::generate_series::GenerateSeriesTable::as_generator` · datafusion-functions-table 55.1.0

```rust
fn as_generator(&self, batch_size: usize) -> Result<Arc<RwLock<dyn LazyBatchGenerator>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenerateSeriesTable", "path": "GenerateSeriesTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [411, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cd62289db02124cbdcff41c"></a>
## clone

`function` · `datafusion_functions_table::generate_series::GenerateSeriesTable::clone` · datafusion-functions-table 55.1.0

```rust
fn clone(&self) -> GenerateSeriesTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenerateSeriesTable", "path": "GenerateSeriesTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 17], "end": [279, 22], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generate_series.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aac9655c48d2cfd9b4eb0a23"></a>
## fmt

`function` · `datafusion_functions_table::generate_series::GenerateSeriesTable::fmt` · datafusion-functions-table 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenerateSeriesTable", "path": "GenerateSeriesTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 10], "end": [279, 15], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generate_series.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0a610ed466789e974702ba1"></a>
## new

`function` · `datafusion_functions_table::generate_series::GenerateSeriesTable::new` · datafusion-functions-table 55.1.0

```rust
fn new(schema: SchemaRef, args: GenSeriesArgs) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenerateSeriesTable", "path": "GenerateSeriesTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [411, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b960bbee242cb0c8ee401d42"></a>
## scan

`function` · `datafusion_functions_table::generate_series::GenerateSeriesTable::scan` · datafusion-functions-table 55.1.0

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], _limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenerateSeriesTable", "path": "GenerateSeriesTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 1], "end": [570, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/generate_series.rs:552`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4533527e2501214334caeaf6"></a>
## schema

`function` · `datafusion_functions_table::generate_series::GenerateSeriesTable::schema` · datafusion-functions-table 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenerateSeriesTable", "path": "GenerateSeriesTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 1], "end": [570, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/generate_series.rs:544`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4c22faafcb9166fe04510e2"></a>
## table_type

`function` · `datafusion_functions_table::generate_series::GenerateSeriesTable::table_type` · datafusion-functions-table 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenerateSeriesTable", "path": "GenerateSeriesTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 1], "end": [570, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/generate_series.rs:548`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
