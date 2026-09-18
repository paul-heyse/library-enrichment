# `deltalake_catalog_unity::datafusion::UnityCatalogList`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.datafusion.UnityCatalogList.json).

<a id="op-b5fb770e3688040360406a3c"></a>
## UnityCatalogList

`struct` · `deltalake_catalog_unity::datafusion::UnityCatalogList` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UnityCatalogList
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L24).

Source: `crates/catalog-unity/src/datafusion.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

In-memory list of catalogs populated by unity catalog

<a id="op-30a4c134aed5a1456a93acd3"></a>
## catalog

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogList::catalog` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogList", "path": "UnityCatalogList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [66, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `crates/catalog-unity/src/datafusion.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-519b166e7f6f97ce745fb53f"></a>
## catalog_names

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogList::catalog_names` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn catalog_names(&self) -> Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogList", "path": "UnityCatalogList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [66, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `crates/catalog-unity/src/datafusion.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76d728bf791427b71a7a89a0"></a>
## catalogs

`struct_field` · `deltalake_catalog_unity::datafusion::UnityCatalogList::catalogs` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
catalogs: dashmap::DashMap<String, std::sync::Arc<dyn CatalogProvider>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L26).

Source: `crates/catalog-unity/src/datafusion.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Collection of catalogs containing schemas and ultimately TableProviders

<a id="op-d4c606a89a53d3e3170e6504"></a>
## fmt

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogList::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogList", "path": "UnityCatalogList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/datafusion.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45703131a6bf49b1f01e04f1"></a>
## register_catalog

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogList::register_catalog` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogList", "path": "UnityCatalogList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [66, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `crates/catalog-unity/src/datafusion.rs:51`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bbe0ba933873ecdbd2f6a5c"></a>
## try_new

`function` · `deltalake_catalog_unity::datafusion::UnityCatalogList::try_new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(client: Arc<UnityCatalog>) -> DataCatalogResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/datafusion.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::datafusion::UnityCatalogList", "path": "UnityCatalogList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [48, 2], "filename": "crates/catalog-unity/src/datafusion.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/datafusion.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [`UnityCatalogList`](../operations/deltalake_catalog_unity.datafusion.UnityCatalogList.md#op-b5fb770e3688040360406a3c)
