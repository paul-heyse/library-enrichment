# `deltalake_catalog_unity::client::backoff::BackoffConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.backoff.BackoffConfig.json).

<a id="op-2ad37cade38b3ae6f9d5b87b"></a>
## BackoffConfig

`struct` · `deltalake_catalog_unity::client::backoff::BackoffConfig` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct BackoffConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L10).

Source: `crates/catalog-unity/src/client/backoff.rs:10`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Exponential backoff with jitter

See <https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/>

<a id="op-a800854b3c5f784354499ead"></a>
## base

`struct_field` · `deltalake_catalog_unity::client::backoff::BackoffConfig::base` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
base: f64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L16).

Source: `crates/catalog-unity/src/client/backoff.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The base of the exponential to use

<a id="op-a6cdcb712f351e8454ddda75"></a>
## clone

`function` · `deltalake_catalog_unity::client::backoff::BackoffConfig::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> BackoffConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::backoff::BackoffConfig", "path": "BackoffConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 17], "end": [9, 22], "filename": "crates/catalog-unity/src/client/backoff.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/client/backoff.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f1b611da64d4b756c7ed45b"></a>
## default

`function` · `deltalake_catalog_unity::client::backoff::BackoffConfig::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L20).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::backoff::BackoffConfig", "path": "BackoffConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 1], "end": [27, 2], "filename": "crates/catalog-unity/src/client/backoff.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/client/backoff.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a2e8877824a4f99558c42d3"></a>
## fmt

`function` · `deltalake_catalog_unity::client::backoff::BackoffConfig::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::backoff::BackoffConfig", "path": "BackoffConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 10], "end": [9, 15], "filename": "crates/catalog-unity/src/client/backoff.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/client/backoff.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e1050a30f3ba3011e9feb44"></a>
## init_backoff

`struct_field` · `deltalake_catalog_unity::client::backoff::BackoffConfig::init_backoff` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
init_backoff: std::time::Duration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L12).

Source: `crates/catalog-unity/src/client/backoff.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The initial backoff duration

<a id="op-e4ff5ae564b411b319778e5b"></a>
## max_backoff

`struct_field` · `deltalake_catalog_unity::client::backoff::BackoffConfig::max_backoff` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
max_backoff: std::time::Duration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L14).

Source: `crates/catalog-unity/src/client/backoff.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The maximum backoff duration
