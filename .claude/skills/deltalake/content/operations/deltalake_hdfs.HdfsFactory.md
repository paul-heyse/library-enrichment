# `deltalake_hdfs::HdfsFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_hdfs.HdfsFactory.json).

<a id="op-1977622353ab20547bf576b9"></a>
## HdfsFactory

`struct` · `deltalake_hdfs::HdfsFactory` · deltalake-hdfs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct HdfsFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/hdfs/src/lib.rs#L12).

Source: `crates/hdfs/src/lib.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07f544b3ca5bfa2df54dadc6"></a>
## clone

`function` · `deltalake_hdfs::HdfsFactory::clone` · deltalake-hdfs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> HdfsFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/hdfs/src/lib.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_hdfs::HdfsFactory", "path": "HdfsFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 10], "end": [11, 15], "filename": "crates/hdfs/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/hdfs/src/lib.rs:11`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26eb943b0180cda561ea6ff3"></a>
## default

`function` · `deltalake_hdfs::HdfsFactory::default` · deltalake-hdfs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> HdfsFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/hdfs/src/lib.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_hdfs::HdfsFactory", "path": "HdfsFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 17], "end": [11, 24], "filename": "crates/hdfs/src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/hdfs/src/lib.rs:11`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-871656084861413d9948f101"></a>
## fmt

`function` · `deltalake_hdfs::HdfsFactory::fmt` · deltalake-hdfs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/hdfs/src/lib.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_hdfs::HdfsFactory", "path": "HdfsFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 26], "end": [11, 31], "filename": "crates/hdfs/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/hdfs/src/lib.rs:11`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a6091eed97468d1f992a950"></a>
## parse_url_opts

`function` · `deltalake_hdfs::HdfsFactory::parse_url_opts` · deltalake-hdfs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/hdfs/src/lib.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_hdfs::HdfsFactory", "path": "HdfsFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 1], "end": [33, 2], "filename": "crates/hdfs/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::ObjectStoreFactory", "path": "ObjectStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::ObjectStoreFactory"}`

Source: `crates/hdfs/src/lib.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4fbbaa369eb9b40dcc96f50"></a>
## with_options

`function` · `deltalake_hdfs::HdfsFactory::with_options` · deltalake-hdfs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/hdfs/src/lib.rs#L36).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_hdfs::HdfsFactory", "path": "HdfsFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [50, 2], "filename": "crates/hdfs/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::LogStoreFactory", "path": "LogStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::LogStoreFactory"}`

Source: `crates/hdfs/src/lib.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
