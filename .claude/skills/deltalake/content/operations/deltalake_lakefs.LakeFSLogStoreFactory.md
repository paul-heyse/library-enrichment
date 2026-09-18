# `deltalake_lakefs::LakeFSLogStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.LakeFSLogStoreFactory.json).

<a id="op-21e02c0b212ef3020c638cfd"></a>
## LakeFSLogStoreFactory

`struct` · `deltalake_lakefs::LakeFSLogStoreFactory` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LakeFSLogStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/lib.rs#L23).

Source: `crates/lakefs/src/lib.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a374405e83f61898048a3d0"></a>
## clone

`function` · `deltalake_lakefs::LakeFSLogStoreFactory::clone` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LakeFSLogStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/lib.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::LakeFSLogStoreFactory", "path": "LakeFSLogStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 15], "filename": "crates/lakefs/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/lakefs/src/lib.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9e181e02c6370a9c75dc606"></a>
## default

`function` · `deltalake_lakefs::LakeFSLogStoreFactory::default` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> LakeFSLogStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/lib.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::LakeFSLogStoreFactory", "path": "LakeFSLogStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 24], "end": [22, 31], "filename": "crates/lakefs/src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/lakefs/src/lib.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e273b8406a2409d881c0a52"></a>
## fmt

`function` · `deltalake_lakefs::LakeFSLogStoreFactory::fmt` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/lib.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::LakeFSLogStoreFactory", "path": "LakeFSLogStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 17], "end": [22, 22], "filename": "crates/lakefs/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/lakefs/src/lib.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-febcbdc22d20cb848bbd1d43"></a>
## with_options

`function` · `deltalake_lakefs::LakeFSLogStoreFactory::with_options` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, config: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/lib.rs#L28).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::LakeFSLogStoreFactory", "path": "LakeFSLogStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [39, 2], "filename": "crates/lakefs/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::LogStoreFactory", "path": "LogStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::LogStoreFactory"}`

Source: `crates/lakefs/src/lib.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
