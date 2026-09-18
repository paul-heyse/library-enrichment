# `datafusion::test_util::TestTableProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.TestTableProvider.json).

<a id="op-1ca1dfb5392b3d4695483a82"></a>
## TestTableProvider

`struct` · `datafusion::test_util::TestTableProvider` · datafusion 55.1.0

```rust
struct TestTableProvider
```

Source: `src/test_util/mod.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

TableProvider for testing purposes

<a id="op-58225096484a32fc2744c25a"></a>
## fmt

`function` · `datafusion::test_util::TestTableProvider::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::TestTableProvider", "path": "TestTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 10], "end": [202, 15], "filename": "src/test_util/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test_util/mod.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dda0de036f29b79817369b9"></a>
## scan

`function` · `datafusion::test_util::TestTableProvider::scan` · datafusion 55.1.0

```rust
async fn scan(&self, _state: &dyn Session, _projection: Option<&Vec<usize>>, _filters: &[Expr], _limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::TestTableProvider", "path": "TestTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 1], "end": [231, 2], "filename": "src/test_util/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/test_util/mod.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3729f8db0a334178dd516f7d"></a>
## schema

`function` · `datafusion::test_util::TestTableProvider::schema` · datafusion 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::TestTableProvider", "path": "TestTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 1], "end": [231, 2], "filename": "src/test_util/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/test_util/mod.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c82050a0b04e32cafaf2030"></a>
## schema

`struct_field` · `datafusion::test_util::TestTableProvider::schema` · datafusion 55.1.0

```rust
schema: arrow::datatypes::SchemaRef
```

Source: `src/test_util/mod.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

test table schema

<a id="op-d6b6e683bb127d8984a9470a"></a>
## table_type

`function` · `datafusion::test_util::TestTableProvider::table_type` · datafusion 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::TestTableProvider", "path": "TestTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 1], "end": [231, 2], "filename": "src/test_util/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/test_util/mod.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b043b77bc7b613f0477ab18"></a>
## url

`struct_field` · `datafusion::test_util::TestTableProvider::url` · datafusion 55.1.0

```rust
url: String
```

Source: `src/test_util/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

URL of table files or folder
