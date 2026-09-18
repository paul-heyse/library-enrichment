# `deltalake_core::logstore::LogStoreConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.LogStoreConfig.json).

<a id="op-59c339b030128d76c2027908"></a>
## LogStoreConfig

`struct` · `deltalake_core::logstore::LogStoreConfig` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LogStoreConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L321).

Source: `crates/core/src/logstore/mod.rs:321`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration parameters for a log store

<a id="op-69fe5097ee074a257b491da9"></a>
## clone

`function` · `deltalake_core::logstore::LogStoreConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LogStoreConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L320).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 17], "end": [320, 22], "filename": "crates/core/src/logstore/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/mod.rs:320`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77b0ee5079d6918405e42734"></a>
## decorate_store

`function` · `deltalake_core::logstore::LogStoreConfig::decorate_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn decorate_store<T: ObjectStore + Clone>(&self, store: T, table_root: Option<&url::Url>) -> DeltaResult<Box<dyn ObjectStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L347).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [360, 2], "filename": "crates/core/src/logstore/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/mod.rs:347`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Wrap a raw object `store` with the decorators (retry, limit, runtime, ...) implied by
this configuration, scoped to `table_root` (defaulting to the config's location).

<a id="op-d8e01c6fce90e0837da319e3"></a>
## deserialize

`function` · `deltalake_core::logstore::LogStoreConfig::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L742).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [741, 1], "end": [775, 2], "filename": "crates/core/src/logstore/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/logstore/mod.rs:742`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d4063d23fc09c518b969253"></a>
## fmt

`function` · `deltalake_core::logstore::LogStoreConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L320).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 10], "end": [320, 15], "filename": "crates/core/src/logstore/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/mod.rs:320`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-233d00339591188337920414"></a>
## location

`function` · `deltalake_core::logstore::LogStoreConfig::location` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn location(&self) -> &Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L336).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [360, 2], "filename": "crates/core/src/logstore/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/mod.rs:336`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the normalized root URL of the table this config describes.

<a id="op-95132d26662b9c9caf747f51"></a>
## new

`function` · `deltalake_core::logstore::LogStoreConfig::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(location: &Url, options: StorageConfig) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L330).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [360, 2], "filename": "crates/core/src/logstore/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/mod.rs:330`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new config for the given table `location`, normalizing the URL form.

<a id="op-112b4cc6d75ebbc5f835b1d2"></a>
## object_store_factory

`function` · `deltalake_core::logstore::LogStoreConfig::object_store_factory` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn object_store_factory(&self) -> ObjectStoreFactoryRegistry
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L357).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [360, 2], "filename": "crates/core/src/logstore/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/mod.rs:357`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the global registry of object store factories used for backend discovery.

<a id="op-1ad3332be51e95dcb5b8c992"></a>
## options

`function` · `deltalake_core::logstore::LogStoreConfig::options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn options(&self) -> &StorageConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L341).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [360, 2], "filename": "crates/core/src/logstore/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/mod.rs:341`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the storage options used to build and decorate the object store.

<a id="op-878ef2d0b6cfdcdb8da9a12f"></a>
## serialize

`function` · `deltalake_core::logstore::LogStoreConfig::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L730).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::LogStoreConfig", "path": "LogStoreConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [729, 1], "end": [739, 2], "filename": "crates/core/src/logstore/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/logstore/mod.rs:730`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4909a1161c95a52edaeafd39"></a>
## location

`struct_field` · `deltalake_core::logstore::LogStoreConfig::location` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
location: url::Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L323).

Source: `crates/core/src/logstore/mod.rs:323`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

url corresponding to the storage location.

<a id="op-34eb82826e8125d891752174"></a>
## options

`struct_field` · `deltalake_core::logstore::LogStoreConfig::options` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
options: StorageConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L325).

Source: `crates/core/src/logstore/mod.rs:325`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
