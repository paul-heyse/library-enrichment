# `deltalake_gcp::GcpFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_gcp.GcpFactory.json).

<a id="op-0522254c64b5aebde3763b6d"></a>
## GcpFactory

`struct` · `deltalake_gcp::GcpFactory` · deltalake-gcp 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct GcpFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/gcp/src/lib.rs#L38).

Source: `crates/gcp/src/lib.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1fa5f7e7479b5a6bac992d6"></a>
## clone

`function` · `deltalake_gcp::GcpFactory::clone` · deltalake-gcp 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> GcpFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/gcp/src/lib.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_gcp::GcpFactory", "path": "GcpFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "crates/gcp/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/gcp/src/lib.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7de583d2e6574cd8579430d"></a>
## default

`function` · `deltalake_gcp::GcpFactory::default` · deltalake-gcp 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> GcpFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/gcp/src/lib.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_gcp::GcpFactory", "path": "GcpFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 24], "filename": "crates/gcp/src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/gcp/src/lib.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1261dcdad641b9a38ab5801"></a>
## fmt

`function` · `deltalake_gcp::GcpFactory::fmt` · deltalake-gcp 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/gcp/src/lib.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_gcp::GcpFactory", "path": "GcpFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 26], "end": [37, 31], "filename": "crates/gcp/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/gcp/src/lib.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a1b41ecff54ec82c98cf764"></a>
## parse_url_opts

`function` · `deltalake_gcp::GcpFactory::parse_url_opts` · deltalake-gcp 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/gcp/src/lib.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_gcp::GcpFactory", "path": "GcpFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [76, 2], "filename": "crates/gcp/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::ObjectStoreFactory", "path": "ObjectStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::ObjectStoreFactory"}`

Source: `crates/gcp/src/lib.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3094ebc20882fb1f3715aac"></a>
## with_options

`function` · `deltalake_gcp::GcpFactory::with_options` · deltalake-gcp 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/gcp/src/lib.rs#L79).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_gcp::GcpFactory", "path": "GcpFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [93, 2], "filename": "crates/gcp/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::LogStoreFactory", "path": "LogStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::LogStoreFactory"}`

Source: `crates/gcp/src/lib.rs:79`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
