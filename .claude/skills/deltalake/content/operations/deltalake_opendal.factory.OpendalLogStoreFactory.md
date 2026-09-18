# `deltalake_opendal::factory::OpendalLogStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.factory.OpendalLogStoreFactory.json).

<a id="op-5e5583e81279a2143b0475a4"></a>
## OpendalLogStoreFactory

`struct` · `deltalake_opendal::factory::OpendalLogStoreFactory` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct OpendalLogStoreFactory<A: OpendalAdapter>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/factory.rs#L65).

Source: `crates/opendal/src/factory.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

[`LogStoreFactory`](../operations/deltalake_core.logstore.factories.LogStoreFactory.md#op-2bd01a2a1c90cd84c18ef166) that pairs with [`OpendalObjectStoreFactory`](../operations/deltalake_opendal.factory.OpendalObjectStoreFactory.md#op-3357fd1c29a9679d63313282).

The PrefixStore is always recomputed from the adapter's
[`OpendalAdapter::logstore_prefix`](../operations/deltalake_opendal.adapter.OpendalAdapter.md#op-54c17f21a4d0b21303a489b1) rather than trusting the `prefixed_store`
passed in by delta's `decorate_store` (which derives its prefix from
`url.path()`). For bucket-root services the two agree; for services whose
operator is scoped deeper than the bucket root, the adapter is authoritative.

<a id="op-ee787a0a47ef2830656aadea"></a>
## 0

`struct_field` · `deltalake_opendal::factory::OpendalLogStoreFactory::0` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
0: std::sync::Arc<A>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/factory.rs#L65).

Source: `crates/opendal/src/factory.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2eba89e7b1dc2bb324ffae8"></a>
## fmt

`function` · `deltalake_opendal::factory::OpendalLogStoreFactory::fmt` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/factory.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "A"}}], "constraints": []}}, "id": "deltalake_opendal::factory::OpendalLogStoreFactory", "path": "OpendalLogStoreFactory"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "deltalake_opendal::adapter::OpendalAdapter", "path": "OpendalAdapter"}}}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "crates/opendal/src/factory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/opendal/src/factory.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0799aa5b43f8674fd6cf6d1"></a>
## with_options

`function` · `deltalake_opendal::factory::OpendalLogStoreFactory::with_options` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, _prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/factory.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "A"}}], "constraints": []}}, "id": "deltalake_opendal::factory::OpendalLogStoreFactory", "path": "OpendalLogStoreFactory"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "deltalake_opendal::adapter::OpendalAdapter", "path": "OpendalAdapter"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [91, 2], "filename": "crates/opendal/src/factory.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::LogStoreFactory", "path": "LogStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::LogStoreFactory"}`

Source: `crates/opendal/src/factory.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
