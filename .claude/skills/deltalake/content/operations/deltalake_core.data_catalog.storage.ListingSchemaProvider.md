# `deltalake_core::data_catalog::storage::ListingSchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.data_catalog.storage.ListingSchemaProvider.json).

<a id="op-77905797bc9ea970d9636b9b"></a>
## ListingSchemaProvider

`struct` · `deltalake_core::data_catalog::storage::ListingSchemaProvider` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ListingSchemaProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L33).

Source: `crates/core/src/data_catalog/storage/mod.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A `SchemaProvider` that scans an `ObjectStore` to automatically discover delta tables.

A subfolder relationship is assumed, i.e. given:
authority = s3://host.example.com:3000
path = /data/tpch

A table called "customer" will be registered for the folder:
s3://host.example.com:3000/data/tpch/customer

assuming it contains valid deltalake data, i.e a `_delta_log` folder:
s3://host.example.com:3000/data/tpch/customer/_delta_log/

<a id="op-a593fc6137c2ea4e60057433"></a>
## deregister_table

`function` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::deregister_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deregister_table(&self, _name: &str) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::storage::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [145, 2], "filename": "crates/core/src/data_catalog/storage/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `crates/core/src/data_catalog/storage/mod.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0245d66d2d908b64050a7df"></a>
## fmt

`function` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::storage::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "crates/core/src/data_catalog/storage/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/data_catalog/storage/mod.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-babde3fe0ba6f46d80733bcc"></a>
## refresh

`function` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::refresh` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn refresh(&self) -> datafusion::common::Result<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L61).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::storage::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [87, 2], "filename": "crates/core/src/data_catalog/storage/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/data_catalog/storage/mod.rs:61`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Reload table information from ObjectStore

<a id="op-3f5dd11a6f4a1e62636fd5f0"></a>
## register_table

`function` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::register_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_table(&self, _name: String, _table: Arc<dyn TableProvider>) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::storage::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [145, 2], "filename": "crates/core/src/data_catalog/storage/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `crates/core/src/data_catalog/storage/mod.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87989148393b8a9fd1f582a3"></a>
## table

`function` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn table(&self, name: &str) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L108).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::storage::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [145, 2], "filename": "crates/core/src/data_catalog/storage/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `crates/core/src/data_catalog/storage/mod.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-111be4a30f5d5ab607393b1a"></a>
## table_exist

`function` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::table_exist` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_exist(&self, name: &str) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L142).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::storage::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [145, 2], "filename": "crates/core/src/data_catalog/storage/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `crates/core/src/data_catalog/storage/mod.rs:142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63e26152049e93fddab4d6fe"></a>
## table_names

`function` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::table_names` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_names(&self) -> Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::storage::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [145, 2], "filename": "crates/core/src/data_catalog/storage/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `crates/core/src/data_catalog/storage/mod.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce05bacfd92b6c05f5ab520c"></a>
## try_new

`function` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(root_uri: impl AsRef<str>, options: Option<HashMap<String, String>>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L45).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::storage::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [87, 2], "filename": "crates/core/src/data_catalog/storage/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/data_catalog/storage/mod.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`ListingSchemaProvider`](../operations/deltalake_core.data_catalog.storage.ListingSchemaProvider.md#op-77905797bc9ea970d9636b9b)

<a id="op-19ac8ea09646d4fc09c42965"></a>
## authority

`struct_field` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::authority` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
authority: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L34).

Source: `crates/core/src/data_catalog/storage/mod.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe3ab8a5bdd1556a23485e09"></a>
## storage_options

`struct_field` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
storage_options: logstore::StorageConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L40).

Source: `crates/core/src/data_catalog/storage/mod.rs:40`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Options used to create underlying object stores

<a id="op-c715b146c3e079b56f0f687c"></a>
## store

`struct_field` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
store: std::sync::Arc<dyn ObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L36).

Source: `crates/core/src/data_catalog/storage/mod.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Underlying object store

<a id="op-17edd7143da04e0ca5916c2b"></a>
## tables

`struct_field` · `deltalake_core::data_catalog::storage::ListingSchemaProvider::tables` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
tables: dashmap::DashMap<String, String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/storage/mod.rs#L38).

Source: `crates/core/src/data_catalog/storage/mod.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A map of table names to a fully quilfied storage location
