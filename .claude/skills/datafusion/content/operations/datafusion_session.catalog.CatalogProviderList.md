# `datafusion_session::catalog::CatalogProviderList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.catalog.CatalogProviderList.json).

<a id="op-d1c9ece1dd28ba403a6492b6"></a>
## CatalogProviderList

`trait` · `datafusion_session::catalog::CatalogProviderList` · datafusion-session 55.1.0

```rust
trait CatalogProviderList: Any + Debug + Sync + Send
```

Source: `src/catalog.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Represent a list of named [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad)s.

Please see the documentation on [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad) for details of
implementing a custom catalog.

<a id="op-3923334c75215a66b2d36a73"></a>
## catalog

`function` · `datafusion_session::catalog::CatalogProviderList::catalog` · datafusion-session 55.1.0

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
```

Source: `src/catalog.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Retrieves a specific catalog by name, provided it exists.

<a id="op-0553992160d6c2ffa1e65914"></a>
## catalog_names

`function` · `datafusion_session::catalog::CatalogProviderList::catalog_names` · datafusion-session 55.1.0

```rust
fn catalog_names(&self) -> Vec<String>
```

Source: `src/catalog.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Retrieves the list of available catalog names

<a id="op-f15bbb2411c2703c532bc5cc"></a>
## register_catalog

`function` · `datafusion_session::catalog::CatalogProviderList::register_catalog` · datafusion-session 55.1.0

```rust
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

Source: `src/catalog.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Adds a new catalog to this catalog list
If a catalog of the same name existed before, it is replaced in the list and returned.
