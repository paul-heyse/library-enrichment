# `deltalake_catalog_unity::UnityCatalogFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.UnityCatalogFactory.json).

<a id="op-163a0c60ae10ea2394505db8"></a>
## UnityCatalogFactory

`struct` · `deltalake_catalog_unity::UnityCatalogFactory` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UnityCatalogFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L934).

Source: `crates/catalog-unity/src/lib.rs:934`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb7b4bfc87b2947875472a81"></a>
## clone

`function` · `deltalake_catalog_unity::UnityCatalogFactory::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> UnityCatalogFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L933).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogFactory", "path": "UnityCatalogFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 10], "end": [933, 15], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/lib.rs:933`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-433014251a8751a9d45de418"></a>
## default

`function` · `deltalake_catalog_unity::UnityCatalogFactory::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> UnityCatalogFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L933).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogFactory", "path": "UnityCatalogFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 17], "end": [933, 24], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/lib.rs:933`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e064b0d0708e6b874225fde"></a>
## fmt

`function` · `deltalake_catalog_unity::UnityCatalogFactory::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L933).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogFactory", "path": "UnityCatalogFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 26], "end": [933, 31], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/lib.rs:933`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ecc594bcbdfdd44a4e495cd"></a>
## parse_url_opts

`function` · `deltalake_catalog_unity::UnityCatalogFactory::parse_url_opts` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, table_uri: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L937).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogFactory", "path": "UnityCatalogFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [936, 1], "end": [966, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::ObjectStoreFactory", "path": "ObjectStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::ObjectStoreFactory"}`

Source: `crates/catalog-unity/src/lib.rs:937`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a247d944acbb0f000a14d6c"></a>
## with_options

`function` · `deltalake_catalog_unity::UnityCatalogFactory::with_options` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L969).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogFactory", "path": "UnityCatalogFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [968, 1], "end": [983, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::LogStoreFactory", "path": "LogStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::LogStoreFactory"}`

Source: `crates/catalog-unity/src/lib.rs:969`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
