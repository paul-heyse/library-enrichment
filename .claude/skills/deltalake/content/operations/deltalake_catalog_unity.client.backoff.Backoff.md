# `deltalake_catalog_unity::client::backoff::Backoff`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.backoff.Backoff.json).

<a id="op-9cb35b297cf38d1f35e33e64"></a>
## Backoff

`struct` · `deltalake_catalog_unity::client::backoff::Backoff` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Backoff
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L32).

Source: `crates/catalog-unity/src/client/backoff.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

[`Backoff`](../operations/deltalake_catalog_unity.client.backoff.Backoff.md#op-9cb35b297cf38d1f35e33e64) can be created from a [`BackoffConfig`](../operations/deltalake_catalog_unity.client.backoff.BackoffConfig.md#op-2ad37cade38b3ae6f9d5b87b)

Consecutive calls to [`Backoff::tick`](../operations/deltalake_catalog_unity.client.backoff.Backoff.md#op-cc452462cca3b51282bf9ffe) will return the next backoff interval

<a id="op-f4dcf0ce4561148631df9c87"></a>
## fmt

`function` · `deltalake_catalog_unity::client::backoff::Backoff::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::backoff::Backoff", "path": "Backoff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [49, 2], "filename": "crates/catalog-unity/src/client/backoff.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/client/backoff.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2baf115c08df68b9dcf3690d"></a>
## new

`function` · `deltalake_catalog_unity::client::backoff::Backoff::new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(config: &BackoffConfig) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L53).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::backoff::Backoff", "path": "Backoff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [83, 2], "filename": "crates/catalog-unity/src/client/backoff.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/client/backoff.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`Backoff`](../operations/deltalake_catalog_unity.client.backoff.Backoff.md#op-9cb35b297cf38d1f35e33e64) from the provided [`BackoffConfig`](../operations/deltalake_catalog_unity.client.backoff.BackoffConfig.md#op-2ad37cade38b3ae6f9d5b87b)

<a id="op-6167490d28999e5cf3c36890"></a>
## new_with_rng

`function` · `deltalake_catalog_unity::client::backoff::Backoff::new_with_rng` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_with_rng(config: &BackoffConfig, rng: Option<Box<dyn Rng + Sync + Send>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::backoff::Backoff", "path": "Backoff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [83, 2], "filename": "crates/catalog-unity/src/client/backoff.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/client/backoff.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a new `Backoff` with the optional `rng`

Used [`rand::random_range()`] if no rng provided

Unresolved upstream links (retained, not inferred): ``rand::random_range()``.

<a id="op-cc452462cca3b51282bf9ffe"></a>
## tick

`function` · `deltalake_catalog_unity::client::backoff::Backoff::tick` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn tick(&mut self) -> Duration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::client::backoff::Backoff", "path": "Backoff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [83, 2], "filename": "crates/catalog-unity/src/client/backoff.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/client/backoff.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the next backoff duration to wait for

<a id="op-1830e55c23c09044559855dd"></a>
## base

`struct_field` · `deltalake_catalog_unity::client::backoff::Backoff::base` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
base: f64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L36).

Source: `crates/catalog-unity/src/client/backoff.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b02e98785f4d6a18cc989fc"></a>
## init_backoff

`struct_field` · `deltalake_catalog_unity::client::backoff::Backoff::init_backoff` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
init_backoff: f64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L33).

Source: `crates/catalog-unity/src/client/backoff.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f447c8a4cc2027a5fd4b56aa"></a>
## max_backoff_secs

`struct_field` · `deltalake_catalog_unity::client::backoff::Backoff::max_backoff_secs` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_backoff_secs: f64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L35).

Source: `crates/catalog-unity/src/client/backoff.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecd0804e14b7bd31ecbdb182"></a>
## next_backoff_secs

`struct_field` · `deltalake_catalog_unity::client::backoff::Backoff::next_backoff_secs` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
next_backoff_secs: f64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L34).

Source: `crates/catalog-unity/src/client/backoff.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fd8fff1b385825f42afda69"></a>
## rng

`struct_field` · `deltalake_catalog_unity::client::backoff::Backoff::rng` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
rng: Option<Box<dyn Rng + Sync + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/backoff.rs#L37).

Source: `crates/catalog-unity/src/client/backoff.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
