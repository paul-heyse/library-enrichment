# `datafusion::datasource::listing_table_factory::ListingTableFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.listing_table_factory.ListingTableFactory.json).

<a id="op-2dbce41fbc197dca1c61ded0"></a>
## ListingTableFactory

`struct` · `datafusion::datasource::listing_table_factory::ListingTableFactory` · datafusion 55.1.0

```rust
struct ListingTableFactory
```

Source: `src/datasource/listing_table_factory.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

A `TableProviderFactory` capable of creating new `ListingTable`s

<a id="op-e9f5624a4e6d1430db553283"></a>
## create

`function` · `datafusion::datasource::listing_table_factory::ListingTableFactory::create` · datafusion 55.1.0

```rust
async fn create(&self, state: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::listing_table_factory::ListingTableFactory", "path": "ListingTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [301, 2], "filename": "src/datasource/listing_table_factory.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProviderFactory", "path": "TableProviderFactory"}, "trait_path": "datafusion_session::table::TableProviderFactory"}`

Source: `src/datasource/listing_table_factory.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a0798f94ece68b79ce0f82a"></a>
## default

`function` · `datafusion::datasource::listing_table_factory::ListingTableFactory::default` · datafusion 55.1.0

```rust
fn default() -> ListingTableFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::listing_table_factory::ListingTableFactory", "path": "ListingTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 24], "filename": "src/datasource/listing_table_factory.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datasource/listing_table_factory.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1708bcaa7258a53eaea21bb9"></a>
## fmt

`function` · `datafusion::datasource::listing_table_factory::ListingTableFactory::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::listing_table_factory::ListingTableFactory", "path": "ListingTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/datasource/listing_table_factory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datasource/listing_table_factory.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52164a7cf5d1ff5134496fb6"></a>
## new

`function` · `datafusion::datasource::listing_table_factory::ListingTableFactory::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::datasource::listing_table_factory::ListingTableFactory", "path": "ListingTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [49, 2], "filename": "src/datasource/listing_table_factory.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/listing_table_factory.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new `ListingTableFactory`
