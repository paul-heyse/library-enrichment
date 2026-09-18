# `deltalake_lakefs::client::LakeFSConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.client.LakeFSConfig.json).

<a id="op-3159e7547cb7b3569a0bbf8d"></a>
## LakeFSConfig

`struct` · `deltalake_lakefs::client::LakeFSConfig` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LakeFSConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L15).

Source: `crates/lakefs/src/client.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b41662283177acb1012613b4"></a>
## clone

`function` · `deltalake_lakefs::client::LakeFSConfig::clone` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LakeFSConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSConfig", "path": "LakeFSConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 17], "end": [14, 22], "filename": "crates/lakefs/src/client.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/lakefs/src/client.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f16a0a5122e03d2acb4dffef"></a>
## fmt

`function` · `deltalake_lakefs::client::LakeFSConfig::fmt` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSConfig", "path": "LakeFSConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 10], "end": [14, 15], "filename": "crates/lakefs/src/client.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/lakefs/src/client.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-979f7d706809098785753fbc"></a>
## new

`function` · `deltalake_lakefs::client::LakeFSConfig::new` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(host: String, username: String, password: String) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSConfig", "path": "LakeFSConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 1], "end": [29, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dc3513e08fd45ee91c17206"></a>
## host

`struct_field` · `deltalake_lakefs::client::LakeFSConfig::host` · deltalake-lakefs 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
host: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L16).

Source: `crates/lakefs/src/client.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69ec3b04c37b88ee01e17a1c"></a>
## password

`struct_field` · `deltalake_lakefs::client::LakeFSConfig::password` · deltalake-lakefs 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
password: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L18).

Source: `crates/lakefs/src/client.rs:18`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b41a581a1fcf81ce49d9f19"></a>
## username

`struct_field` · `deltalake_lakefs::client::LakeFSConfig::username` · deltalake-lakefs 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
username: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L17).

Source: `crates/lakefs/src/client.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
