# `datafusion_ffi::table_provider::ForeignTableProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.table_provider.ForeignTableProvider.json).

<a id="op-ab25e1e1ab72dc9c708307d4"></a>
## ForeignTableProvider

`struct` · `datafusion_ffi::table_provider::ForeignTableProvider` · datafusion-ffi 55.1.0

```rust
struct ForeignTableProvider
```

Source: `src/table_provider.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_TableProvider to interact with the foreign table provider.

<a id="op-d6ed83d7763124bb1930b725"></a>
## 0

`struct_field` · `datafusion_ffi::table_provider::ForeignTableProvider::0` · datafusion-ffi 55.1.0

```rust
0: FFI_TableProvider
```

Source: `src/table_provider.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62614c8765a8c109dfd71854"></a>
## fmt

`function` · `datafusion_ffi::table_provider::ForeignTableProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::ForeignTableProvider", "path": "ForeignTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 10], "end": [452, 15], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_provider.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-705c89a69c76959233020d0b"></a>
## insert_into

`function` · `datafusion_ffi::table_provider::ForeignTableProvider::insert_into` · datafusion-ffi 55.1.0

```rust
async fn insert_into(&self, session: &dyn Session, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::ForeignTableProvider", "path": "ForeignTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [592, 2], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table_provider.rs:571`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63f62a5e3f497518538a40d8"></a>
## scan

`function` · `datafusion_ffi::table_provider::ForeignTableProvider::scan` · datafusion-ffi 55.1.0

```rust
async fn scan(&self, session: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::ForeignTableProvider", "path": "ForeignTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [592, 2], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table_provider.rs:500`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bdd544154c5d28e6e35f247"></a>
## schema

`function` · `datafusion_ffi::table_provider::ForeignTableProvider::schema` · datafusion-ffi 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::ForeignTableProvider", "path": "ForeignTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [592, 2], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table_provider.rs:476`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f50043a25b8d45a2317f1a7"></a>
## statistics

`function` · `datafusion_ffi::table_provider::ForeignTableProvider::statistics` · datafusion-ffi 55.1.0

```rust
fn statistics(&self) -> Option<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::ForeignTableProvider", "path": "ForeignTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [592, 2], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table_provider.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9381f156977df846d99f2153"></a>
## supports_filters_pushdown

`function` · `datafusion_ffi::table_provider::ForeignTableProvider::supports_filters_pushdown` · datafusion-ffi 55.1.0

```rust
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::ForeignTableProvider", "path": "ForeignTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [592, 2], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table_provider.rs:537`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Tests whether the table provider can make use of a filter expression
to optimize data retrieval.

<a id="op-0e4cddd26c7fe60e1f46df79"></a>
## table_type

`function` · `datafusion_ffi::table_provider::ForeignTableProvider::table_type` · datafusion-ffi 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::ForeignTableProvider", "path": "ForeignTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [592, 2], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table_provider.rs:481`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
