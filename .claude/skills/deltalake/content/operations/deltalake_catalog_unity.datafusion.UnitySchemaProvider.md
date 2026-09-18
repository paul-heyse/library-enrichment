# `deltalake_catalog_unity::datafusion::UnitySchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.datafusion.UnitySchemaProvider.json).

<a id="op-5615ed312aa506091d360a3d"></a>
## UnitySchemaProvider

`struct` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UnitySchemaProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L130).

Source: `crates/catalog-unity/src/datafusion.rs:130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A datafusion [`SchemaProvider`] backed by Databricks UnityCatalog

Unresolved upstream links (retained, not inferred): ``SchemaProvider``.

<a id="op-7d6547a1cea2ca420b045551"></a>
## fmt

`function` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L129).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnitySchemaProvider", "path": "UnitySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 10], "end": [129, 15], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/datafusion.rs:129`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe740176beb08a560ad0549f"></a>
## table

`function` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::table` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn table(&self, name: &str) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L221).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnitySchemaProvider", "path": "UnitySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [283, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `crates/catalog-unity/src/datafusion.rs:221`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1534145d7411b810751edd74"></a>
## table_exist

`function` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::table_exist` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_exist(&self, name: &str) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L280).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnitySchemaProvider", "path": "UnitySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [283, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `crates/catalog-unity/src/datafusion.rs:280`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9df3d30351c1e1ef34adb7e4"></a>
## table_names

`function` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::table_names` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_names(&self) -> Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L217).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnitySchemaProvider", "path": "UnitySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [283, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `crates/catalog-unity/src/datafusion.rs:217`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83257142fe1fc54ab950d5c7"></a>
## try_new

`function` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::try_new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(client: Arc<UnityCatalog>, catalog_name: impl Into<String>, schema_name: impl Into<String>) -> DataCatalogResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L143).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnitySchemaProvider", "path": "UnitySchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [213, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/datafusion.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [`UnitySchemaProvider`](../operations/deltalake_catalog_unity.datafusion.UnitySchemaProvider.md#op-5615ed312aa506091d360a3d)

<a id="op-aed80524ad1530d96199bbb6"></a>
## catalog_name

`struct_field` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::catalog_name` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
catalog_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L132).

Source: `crates/catalog-unity/src/datafusion.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6d715ba770d1c90d111d30d"></a>
## client

`struct_field` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::client` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client: std::sync::Arc<super::UnityCatalog>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L131).

Source: `crates/catalog-unity/src/datafusion.rs:131`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39367517c63574fb94c3fb7f"></a>
## schema_name

`struct_field` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::schema_name` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
schema_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L133).

Source: `crates/catalog-unity/src/datafusion.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a6b76318e651e1a2a2ca083"></a>
## table_cache

`struct_field` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::table_cache` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_cache: dashmap::DashMap<String, std::sync::Arc<dyn TableProvider>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L137).

Source: `crates/catalog-unity/src/datafusion.rs:137`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af9efa86b0d19e2b25a81e86"></a>
## table_names

`struct_field` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::table_names` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_names: Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L136).

Source: `crates/catalog-unity/src/datafusion.rs:136`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parent catalog for schemas of interest.

<a id="op-c94c520acf507988e825e537"></a>
## token_cache

`struct_field` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider::token_cache` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
token_cache: moka::future::Cache<String, super::models::TemporaryTableCredentials>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L138).

Source: `crates/catalog-unity/src/datafusion.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
