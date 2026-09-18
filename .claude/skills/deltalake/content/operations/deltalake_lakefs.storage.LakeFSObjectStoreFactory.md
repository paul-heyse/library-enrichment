# `deltalake_lakefs::storage::LakeFSObjectStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.storage.LakeFSObjectStoreFactory.json).

<a id="op-286a549113a75196d67f0d0f"></a>
## LakeFSObjectStoreFactory

`struct` · `deltalake_lakefs::storage::LakeFSObjectStoreFactory` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LakeFSObjectStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/storage.rs#L17).

Source: `crates/lakefs/src/storage.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d51d794cc4f484192d51d06"></a>
## clone

`function` · `deltalake_lakefs::storage::LakeFSObjectStoreFactory::clone` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LakeFSObjectStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/storage.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::storage::LakeFSObjectStoreFactory", "path": "LakeFSObjectStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 10], "end": [16, 15], "filename": "crates/lakefs/src/storage.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/lakefs/src/storage.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e8cda902e41360f0c42f5a4"></a>
## default

`function` · `deltalake_lakefs::storage::LakeFSObjectStoreFactory::default` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> LakeFSObjectStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/storage.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::storage::LakeFSObjectStoreFactory", "path": "LakeFSObjectStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 17], "end": [16, 24], "filename": "crates/lakefs/src/storage.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/lakefs/src/storage.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36d45d002afcb2ca725ec25f"></a>
## fmt

`function` · `deltalake_lakefs::storage::LakeFSObjectStoreFactory::fmt` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/storage.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::storage::LakeFSObjectStoreFactory", "path": "LakeFSObjectStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 26], "end": [16, 31], "filename": "crates/lakefs/src/storage.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/lakefs/src/storage.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-367f7e89d0839d8b8d5b1a30"></a>
## parse_url_opts

`function` · `deltalake_lakefs::storage::LakeFSObjectStoreFactory::parse_url_opts` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/storage.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::storage::LakeFSObjectStoreFactory", "path": "LakeFSObjectStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [106, 2], "filename": "crates/lakefs/src/storage.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::ObjectStoreFactory", "path": "ObjectStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::ObjectStoreFactory"}`

Source: `crates/lakefs/src/storage.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
