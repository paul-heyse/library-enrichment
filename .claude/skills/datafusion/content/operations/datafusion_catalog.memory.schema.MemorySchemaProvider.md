# `datafusion_catalog::memory::schema::MemorySchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.memory.schema.MemorySchemaProvider.json).

<a id="op-faa734bbbe1c631477491cd1"></a>
## MemorySchemaProvider

`struct` · `datafusion_catalog::memory::schema::MemorySchemaProvider` · datafusion-catalog 55.1.0

```rust
struct MemorySchemaProvider
```

Source: `src/memory/schema.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Simple in-memory implementation of a schema.

<a id="op-339d6b16bf9a5a54e06b0646"></a>
## default

`function` · `datafusion_catalog::memory::schema::MemorySchemaProvider::default` · datafusion-catalog 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::schema::MemorySchemaProvider", "path": "MemorySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [45, 2], "filename": "src/memory/schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/memory/schema.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-accaa4d6b3a8d13095909f76"></a>
## deregister_table

`function` · `datafusion_catalog::memory::schema::MemorySchemaProvider::deregister_table` · datafusion-catalog 55.1.0

```rust
fn deregister_table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::schema::MemorySchemaProvider", "path": "MemorySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [84, 2], "filename": "src/memory/schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/memory/schema.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-575534dfa0cc25f26b49cb92"></a>
## fmt

`function` · `datafusion_catalog::memory::schema::MemorySchemaProvider::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::schema::MemorySchemaProvider", "path": "MemorySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "src/memory/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory/schema.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c384b8ce28fd1b676dadfa9e"></a>
## new

`function` · `datafusion_catalog::memory::schema::MemorySchemaProvider::new` · datafusion-catalog 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::schema::MemorySchemaProvider", "path": "MemorySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [39, 2], "filename": "src/memory/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory/schema.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Instantiates a new MemorySchemaProvider with an empty collection of tables.

<a id="op-89eb55493aa78d3d42e2a9e4"></a>
## register_table

`function` · `datafusion_catalog::memory::schema::MemorySchemaProvider::register_table` · datafusion-catalog 55.1.0

```rust
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::schema::MemorySchemaProvider", "path": "MemorySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [84, 2], "filename": "src/memory/schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/memory/schema.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b046d8bf18ff0d2376a9ae5a"></a>
## table

`function` · `datafusion_catalog::memory::schema::MemorySchemaProvider::table` · datafusion-catalog 55.1.0

```rust
async fn table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::schema::MemorySchemaProvider", "path": "MemorySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [84, 2], "filename": "src/memory/schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/memory/schema.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c0065c5681bebaf76519797"></a>
## table_exist

`function` · `datafusion_catalog::memory::schema::MemorySchemaProvider::table_exist` · datafusion-catalog 55.1.0

```rust
fn table_exist(&self, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::schema::MemorySchemaProvider", "path": "MemorySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [84, 2], "filename": "src/memory/schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/memory/schema.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58ee467d7ff9b826322c553a"></a>
## table_names

`function` · `datafusion_catalog::memory::schema::MemorySchemaProvider::table_names` · datafusion-catalog 55.1.0

```rust
fn table_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::schema::MemorySchemaProvider", "path": "MemorySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [84, 2], "filename": "src/memory/schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/memory/schema.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
