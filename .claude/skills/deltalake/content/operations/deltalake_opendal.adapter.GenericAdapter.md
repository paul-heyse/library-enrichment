# `deltalake_opendal::adapter::GenericAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.adapter.GenericAdapter.json).

<a id="op-bdda3815c77aad989e184fca"></a>
## GenericAdapter

`struct` · `deltalake_opendal::adapter::GenericAdapter` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct GenericAdapter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L50).

Source: `crates/opendal/src/adapter.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Adapter for "simple" OpenDAL services whose operator is scoped at the bucket
root: the URL host is the bucket and the URL path is the table prefix.

<a id="op-ad4d8e1005fc821eca17577a"></a>
## clone

`function` · `deltalake_opendal::adapter::GenericAdapter::clone` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> GenericAdapter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::adapter::GenericAdapter", "path": "GenericAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 17], "end": [49, 22], "filename": "crates/opendal/src/adapter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/opendal/src/adapter.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45b44dd7e7a7034accf16a48"></a>
## fmt

`function` · `deltalake_opendal::adapter::GenericAdapter::fmt` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::adapter::GenericAdapter", "path": "GenericAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 15], "filename": "crates/opendal/src/adapter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/opendal/src/adapter.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ca7642b94915bc71ae31d43"></a>
## new

`function` · `deltalake_opendal::adapter::GenericAdapter::new` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(service: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::adapter::GenericAdapter", "path": "GenericAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [65, 2], "filename": "crates/opendal/src/adapter.rs"}, "trait": null, "trait_path": null}`

Source: `crates/opendal/src/adapter.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A generic adapter for `service`, reading `opendal.<key>` storage options.

<a id="op-9f64f1af715a3ab57fc34e8a"></a>
## option_prefix

`struct_field` · `deltalake_opendal::adapter::GenericAdapter::option_prefix` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
option_prefix: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L54).

Source: `crates/opendal/src/adapter.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Storage-option prefix whose entries are forwarded to OpenDAL.

<a id="op-8a956158aed34bcd680743fb"></a>
## resolve

`function` · `deltalake_opendal::adapter::GenericAdapter::resolve` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn resolve(&self, url: &Url, config: &StorageConfig) -> DeltaResult<OperatorSpec>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::adapter::GenericAdapter", "path": "GenericAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [92, 2], "filename": "crates/opendal/src/adapter.rs"}, "trait": {"args": null, "id": "deltalake_opendal::adapter::OpendalAdapter", "path": "OpendalAdapter"}, "trait_path": "deltalake_opendal::adapter::OpendalAdapter"}`

Source: `crates/opendal/src/adapter.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35bc5501a5b838f69e7306ee"></a>
## service

`struct_field` · `deltalake_opendal::adapter::GenericAdapter::service` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
service: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L52).

Source: `crates/opendal/src/adapter.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

OpenDAL service scheme passed to `Operator::via_iter`.
