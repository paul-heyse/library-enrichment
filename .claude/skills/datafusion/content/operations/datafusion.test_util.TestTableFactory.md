# `datafusion::test_util::TestTableFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.TestTableFactory.json).

<a id="op-cb63c96e6fe7fc37b37b8f96"></a>
## TestTableFactory

`struct` · `datafusion::test_util::TestTableFactory` · datafusion 55.1.0

```rust
struct TestTableFactory
```

Source: `src/test_util/mod.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

TableFactory for tests

<a id="op-c24961ee22c02a6e1eb78a93"></a>
## create

`function` · `datafusion::test_util::TestTableFactory::create` · datafusion 55.1.0

```rust
async fn create(&self, _: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::TestTableFactory", "path": "TestTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [184, 1], "end": [199, 2], "filename": "src/test_util/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProviderFactory", "path": "TableProviderFactory"}, "trait_path": "datafusion_session::table::TableProviderFactory"}`

Source: `src/test_util/mod.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8978bab101f020f57808e16c"></a>
## default

`function` · `datafusion::test_util::TestTableFactory::default` · datafusion 55.1.0

```rust
fn default() -> TestTableFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::TestTableFactory", "path": "TestTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 10], "end": [180, 17], "filename": "src/test_util/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test_util/mod.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc4f6024abf905a540a64312"></a>
## fmt

`function` · `datafusion::test_util::TestTableFactory::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::TestTableFactory", "path": "TestTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 19], "end": [180, 24], "filename": "src/test_util/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test_util/mod.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
