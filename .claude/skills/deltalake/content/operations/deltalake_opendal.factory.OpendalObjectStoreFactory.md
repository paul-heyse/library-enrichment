# `deltalake_opendal::factory::OpendalObjectStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.factory.OpendalObjectStoreFactory.json).

<a id="op-3357fd1c29a9679d63313282"></a>
## OpendalObjectStoreFactory

`struct` · `deltalake_opendal::factory::OpendalObjectStoreFactory` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct OpendalObjectStoreFactory<A: OpendalAdapter>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/factory.rs#L21).

Source: `crates/opendal/src/factory.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

[`ObjectStoreFactory`](../operations/deltalake_core.logstore.factories.ObjectStoreFactory.md#op-ecc00d1a52536f5b5d865cff) that builds an OpenDAL operator via an [`OpendalAdapter`](../operations/deltalake_opendal.adapter.OpendalAdapter.md#op-710e9b27c5cc68f393fd433d).

<a id="op-d1c8d84bf31a0cd68851ba08"></a>
## 0

`struct_field` · `deltalake_opendal::factory::OpendalObjectStoreFactory::0` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
0: std::sync::Arc<A>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/factory.rs#L21).

Source: `crates/opendal/src/factory.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adc188d8490cbbf612a51235"></a>
## fmt

`function` · `deltalake_opendal::factory::OpendalObjectStoreFactory::fmt` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/factory.rs#L20).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "A"}}], "constraints": []}}, "id": "deltalake_opendal::factory::OpendalObjectStoreFactory", "path": "OpendalObjectStoreFactory"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "deltalake_opendal::adapter::OpendalAdapter", "path": "OpendalAdapter"}}}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 10], "end": [20, 15], "filename": "crates/opendal/src/factory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/opendal/src/factory.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d42e6b77e6a005f8450908e"></a>
## parse_url_opts

`function` · `deltalake_opendal::factory::OpendalObjectStoreFactory::parse_url_opts` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/factory.rs#L24).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "A"}}], "constraints": []}}, "id": "deltalake_opendal::factory::OpendalObjectStoreFactory", "path": "OpendalObjectStoreFactory"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "deltalake_opendal::adapter::OpendalAdapter", "path": "OpendalAdapter"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 1], "end": [55, 2], "filename": "crates/opendal/src/factory.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::ObjectStoreFactory", "path": "ObjectStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::ObjectStoreFactory"}`

Source: `crates/opendal/src/factory.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
