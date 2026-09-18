# `datafusion_catalog::stream::StreamTableFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.stream.StreamTableFactory.json).

<a id="op-271eabd5ad6f1eb941a87428"></a>
## StreamTableFactory

`struct` · `datafusion_catalog::stream::StreamTableFactory` · datafusion-catalog 55.1.0

```rust
struct StreamTableFactory
```

Source: `src/stream.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

A [`TableProviderFactory`](../operations/datafusion_session.table.TableProviderFactory.md#op-69227d35dfbf2f4aef45aae7) for [`StreamTable`](../operations/datafusion_catalog.stream.StreamTable.md#op-4f9e242f2c18b302a51a1b01)

<a id="op-490910e47f331bda9fdaa99c"></a>
## create

`function` · `datafusion_catalog::stream::StreamTableFactory::create` · datafusion-catalog 55.1.0

```rust
async fn create(&self, state: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTableFactory", "path": "StreamTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [90, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProviderFactory", "path": "TableProviderFactory"}, "trait_path": "datafusion_session::table::TableProviderFactory"}`

Source: `src/stream.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81c1d22b633e8f349930315c"></a>
## default

`function` · `datafusion_catalog::stream::StreamTableFactory::default` · datafusion-catalog 55.1.0

```rust
fn default() -> StreamTableFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTableFactory", "path": "StreamTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 17], "end": [45, 24], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/stream.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8197a2cd21b9455abba018e8"></a>
## fmt

`function` · `datafusion_catalog::stream::StreamTableFactory::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamTableFactory", "path": "StreamTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stream.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
