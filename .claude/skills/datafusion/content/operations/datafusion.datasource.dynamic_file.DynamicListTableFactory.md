# `datafusion::datasource::dynamic_file::DynamicListTableFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.dynamic_file.DynamicListTableFactory.json).

<a id="op-ad56d14f3d00d824136610ca"></a>
## DynamicListTableFactory

`struct` · `datafusion::datasource::dynamic_file::DynamicListTableFactory` · datafusion 55.1.0

```rust
struct DynamicListTableFactory
```

Source: `src/datasource/dynamic_file.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

[DynamicListTableFactory](../operations/datafusion.datasource.dynamic_file.DynamicListTableFactory.md#op-ad56d14f3d00d824136610ca) is a factory that can create a [ListingTable](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a) from the given url.

<a id="op-a40859a30046c15371d37a6f"></a>
## default

`function` · `datafusion::datasource::dynamic_file::DynamicListTableFactory::default` · datafusion 55.1.0

```rust
fn default() -> DynamicListTableFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::dynamic_file::DynamicListTableFactory", "path": "DynamicListTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 17], "filename": "src/datasource/dynamic_file.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datasource/dynamic_file.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0c2e8ae0a1aff0faecd99a6"></a>
## fmt

`function` · `datafusion::datasource::dynamic_file::DynamicListTableFactory::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::dynamic_file::DynamicListTableFactory", "path": "DynamicListTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 19], "end": [36, 24], "filename": "src/datasource/dynamic_file.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datasource/dynamic_file.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be908939ec0f6594f2666f9e"></a>
## new

`function` · `datafusion::datasource::dynamic_file::DynamicListTableFactory::new` · datafusion 55.1.0

```rust
fn new(session_store: SessionStore) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::dynamic_file::DynamicListTableFactory", "path": "DynamicListTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [52, 2], "filename": "src/datasource/dynamic_file.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/dynamic_file.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a new [DynamicListTableFactory](../operations/datafusion.datasource.dynamic_file.DynamicListTableFactory.md#op-ad56d14f3d00d824136610ca) with the given state store.

<a id="op-aabc9bed606b57f0ce6cbf61"></a>
## session_store

`function` · `datafusion::datasource::dynamic_file::DynamicListTableFactory::session_store` · datafusion 55.1.0

```rust
fn session_store(&self) -> &SessionStore
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::dynamic_file::DynamicListTableFactory", "path": "DynamicListTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [52, 2], "filename": "src/datasource/dynamic_file.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/dynamic_file.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get the session store.

<a id="op-ff485ceae7af872e80e89d0c"></a>
## try_new

`function` · `datafusion::datasource::dynamic_file::DynamicListTableFactory::try_new` · datafusion 55.1.0

```rust
async fn try_new(&self, url: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::dynamic_file::DynamicListTableFactory", "path": "DynamicListTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [90, 2], "filename": "src/datasource/dynamic_file.rs"}, "trait": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::UrlTableFactory", "path": "UrlTableFactory"}, "trait_path": "datafusion_catalog::dynamic_file::catalog::UrlTableFactory"}`

Source: `src/datasource/dynamic_file.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
