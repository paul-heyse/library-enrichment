# `datafusion::datasource::provider::DefaultTableFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.provider.DefaultTableFactory.json).

<a id="op-bc555a7d741c8244509e1afb"></a>
## DefaultTableFactory

`struct` · `datafusion::datasource::provider::DefaultTableFactory` · datafusion 55.1.0

```rust
struct DefaultTableFactory
```

Source: `src/datasource/provider.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The default [`TableProviderFactory`](../operations/datafusion_session.table.TableProviderFactory.md#op-69227d35dfbf2f4aef45aae7)

If [`CreateExternalTable`](../operations/datafusion_expr.logical_plan.ddl.CreateExternalTable.md#op-4041a48d736315b98cb3dab6) is unbounded calls [`StreamTableFactory::create`],
otherwise calls [`ListingTableFactory::create`](../operations/datafusion.datasource.listing_table_factory.ListingTableFactory.md#op-e9f5624a4e6d1430db553283)

Unresolved upstream links (retained, not inferred): ``StreamTableFactory::create``.

<a id="op-cada5c610fd8f81d606d07bc"></a>
## create

`function` · `datafusion::datasource::provider::DefaultTableFactory::create` · datafusion 55.1.0

```rust
async fn create(&self, state: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::provider::DefaultTableFactory", "path": "DefaultTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [68, 2], "filename": "src/datasource/provider.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProviderFactory", "path": "TableProviderFactory"}, "trait_path": "datafusion_session::table::TableProviderFactory"}`

Source: `src/datasource/provider.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46b11c4fe66f050f1ee9648b"></a>
## default

`function` · `datafusion::datasource::provider::DefaultTableFactory::default` · datafusion 55.1.0

```rust
fn default() -> DefaultTableFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::provider::DefaultTableFactory", "path": "DefaultTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 24], "filename": "src/datasource/provider.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datasource/provider.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b78426b5a6fc294c8a2e334"></a>
## fmt

`function` · `datafusion::datasource::provider::DefaultTableFactory::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::provider::DefaultTableFactory", "path": "DefaultTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/datasource/provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datasource/provider.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-820ade823f75f98a0b7cdde3"></a>
## new

`function` · `datafusion::datasource::provider::DefaultTableFactory::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::provider::DefaultTableFactory", "path": "DefaultTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [47, 2], "filename": "src/datasource/provider.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/provider.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new [`DefaultTableFactory`](../operations/datafusion.datasource.provider.DefaultTableFactory.md#op-bc555a7d741c8244509e1afb)
