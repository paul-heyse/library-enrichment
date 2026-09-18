# `datafusion_ffi::schema_provider::ForeignSchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.schema_provider.ForeignSchemaProvider.json).

<a id="op-ba818db0795b6e7da2b99305"></a>
## ForeignSchemaProvider

`struct` · `datafusion_ffi::schema_provider::ForeignSchemaProvider` · datafusion-ffi 55.1.0

```rust
struct ForeignSchemaProvider
```

Source: `src/schema_provider.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_SchemaProvider to interact with the foreign table provider.

<a id="op-f8dfadd37dd294d01ad88b9e"></a>
## 0

`struct_field` · `datafusion_ffi::schema_provider::ForeignSchemaProvider::0` · datafusion-ffi 55.1.0

```rust
0: FFI_SchemaProvider
```

Source: `src/schema_provider.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bb9fbf4977ecdaf23c11e05"></a>
## deregister_table

`function` · `datafusion_ffi::schema_provider::ForeignSchemaProvider::deregister_table` · datafusion-ffi 55.1.0

```rust
fn deregister_table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::ForeignSchemaProvider", "path": "ForeignSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [380, 2], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/schema_provider.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-121f8c8c97a2940bc2aa8cfb"></a>
## fmt

`function` · `datafusion_ffi::schema_provider::ForeignSchemaProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::ForeignSchemaProvider", "path": "ForeignSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 10], "end": [290, 15], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_provider.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf2526fc39e06d2a2572bae6"></a>
## owner_name

`function` · `datafusion_ffi::schema_provider::ForeignSchemaProvider::owner_name` · datafusion-ffi 55.1.0

```rust
fn owner_name(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::ForeignSchemaProvider", "path": "ForeignSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [380, 2], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/schema_provider.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d428c9cdce4db74c363c1c4"></a>
## register_table

`function` · `datafusion_ffi::schema_provider::ForeignSchemaProvider::register_table` · datafusion-ffi 55.1.0

```rust
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::ForeignSchemaProvider", "path": "ForeignSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [380, 2], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/schema_provider.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bd9c2026577e3c2e528ee9b"></a>
## table

`function` · `datafusion_ffi::schema_provider::ForeignSchemaProvider::table` · datafusion-ffi 55.1.0

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::ForeignSchemaProvider", "path": "ForeignSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [380, 2], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/schema_provider.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42eff17d7ea906865b6a3aaf"></a>
## table_exist

`function` · `datafusion_ffi::schema_provider::ForeignSchemaProvider::table_exist` · datafusion-ffi 55.1.0

```rust
fn table_exist(&self, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::ForeignSchemaProvider", "path": "ForeignSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [380, 2], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/schema_provider.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Returns true if table exist in the schema provider, false otherwise.

<a id="op-6194e27dfb6be98fa02cd05e"></a>
## table_names

`function` · `datafusion_ffi::schema_provider::ForeignSchemaProvider::table_names` · datafusion-ffi 55.1.0

```rust
fn table_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::ForeignSchemaProvider", "path": "ForeignSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [380, 2], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/schema_provider.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
