# `deltalake_mount::MountFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_mount.MountFactory.json).

<a id="op-0a89cf68f84e00c60bdfa324"></a>
## MountFactory

`struct` · `deltalake_mount::MountFactory` · deltalake-mount 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MountFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/lib.rs#L37).

Source: `crates/mount/src/lib.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73d4239820c21dc53eb69702"></a>
## clone

`function` · `deltalake_mount::MountFactory::clone` · deltalake-mount 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> MountFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/lib.rs#L36).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_mount::MountFactory", "path": "MountFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "crates/mount/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/mount/src/lib.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d42dd14380d152084bc70f3"></a>
## default

`function` · `deltalake_mount::MountFactory::default` · deltalake-mount 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> MountFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/lib.rs#L36).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_mount::MountFactory", "path": "MountFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 24], "filename": "crates/mount/src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/mount/src/lib.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bfb185cd5e3bdb989e1ee5a"></a>
## fmt

`function` · `deltalake_mount::MountFactory::fmt` · deltalake-mount 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/lib.rs#L36).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_mount::MountFactory", "path": "MountFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 26], "end": [36, 31], "filename": "crates/mount/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/mount/src/lib.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76fc6c9dfe1530be9d8592b3"></a>
## parse_url_opts

`function` · `deltalake_mount::MountFactory::parse_url_opts` · deltalake-mount 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/lib.rs#L40).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_mount::MountFactory", "path": "MountFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [83, 2], "filename": "crates/mount/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::ObjectStoreFactory", "path": "ObjectStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::ObjectStoreFactory"}`

Source: `crates/mount/src/lib.rs:40`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c5ef1a7a62e3af2435e8d56"></a>
## with_options

`function` · `deltalake_mount::MountFactory::with_options` · deltalake-mount 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/lib.rs#L86).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_mount::MountFactory", "path": "MountFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [100, 2], "filename": "crates/mount/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::LogStoreFactory", "path": "LogStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::LogStoreFactory"}`

Source: `crates/mount/src/lib.rs:86`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
