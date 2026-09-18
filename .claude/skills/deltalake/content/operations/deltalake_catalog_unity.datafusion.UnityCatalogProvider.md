# `deltalake_catalog_unity::datafusion::UnityCatalogProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.datafusion.UnityCatalogProvider.json).

<a id="op-bac5fd8289cfa7c7d2792633"></a>
## UnityCatalogProvider

`struct` · `deltalake_catalog_unity::datafusion::UnityCatalogProvider` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UnityCatalogProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L70).

Source: `crates/catalog-unity/src/datafusion.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A datafusion [`CatalogProvider`] backed by Databricks UnityCatalog

Unresolved upstream links (retained, not inferred): ``CatalogProvider``.

<a id="op-22fd819af4b8910910acfa78"></a>
## fmt

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogProvider::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogProvider", "path": "UnityCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 10], "end": [69, 15], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/datafusion.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-359b3d7e99f39e1baf1f7423"></a>
## schema

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogProvider::schema` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogProvider", "path": "UnityCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [109, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `crates/catalog-unity/src/datafusion.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-685360a18f2c1da4da407275"></a>
## schema_names

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogProvider::schema_names` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema_names(&self) -> Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L102).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogProvider", "path": "UnityCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [109, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `crates/catalog-unity/src/datafusion.rs:102`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c34fe3d29b78474eab8e5010"></a>
## schemas

`struct_field` · `deltalake_catalog_unity::datafusion::UnityCatalogProvider::schemas` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
schemas: dashmap::DashMap<String, std::sync::Arc<dyn SchemaProvider>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L72).

Source: `crates/catalog-unity/src/datafusion.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parent catalog for schemas of interest.

<a id="op-1d5433767210700ddb2570a0"></a>
## try_new

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogProvider::try_new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(client: Arc<UnityCatalog>, catalog_name: impl Into<String>) -> DataCatalogResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogProvider", "path": "UnityCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [99, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/datafusion.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [`UnityCatalogProvider`](../operations/deltalake_catalog_unity.datafusion.UnityCatalogProvider.md#op-bac5fd8289cfa7c7d2792633)
