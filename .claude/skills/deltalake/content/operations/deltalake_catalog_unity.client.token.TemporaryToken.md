# `deltalake_catalog_unity::client::token::TemporaryToken`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.token.TemporaryToken.json).

<a id="op-5bd01ed00906a0658fc6b82b"></a>
## TemporaryToken

`struct` · `deltalake_catalog_unity::client::token::TemporaryToken` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TemporaryToken<T>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L9).

Source: `crates/catalog-unity/src/client/token.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A temporary authentication token with an associated expiry

<a id="op-29dccadb49139a5d66e9f8b6"></a>
## clone

`function` · `deltalake_catalog_unity::client::token::TemporaryToken::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TemporaryToken<T>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L8).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_catalog_unity::client::token::TemporaryToken", "path": "TemporaryToken"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8, 17], "end": [8, 22], "filename": "crates/catalog-unity/src/client/token.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/client/token.rs:8`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbf2f707d222468c476a423f"></a>
## expiry

`struct_field` · `deltalake_catalog_unity::client::token::TemporaryToken::expiry` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
expiry: Option<std::time::Instant>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L14).

Source: `crates/catalog-unity/src/client/token.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The instant at which this credential is no longer valid
None means the credential does not expire

<a id="op-87929bc975da79366bb4dae6"></a>
## fmt

`function` · `deltalake_catalog_unity::client::token::TemporaryToken::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L8).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_catalog_unity::client::token::TemporaryToken", "path": "TemporaryToken"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8, 10], "end": [8, 15], "filename": "crates/catalog-unity/src/client/token.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/client/token.rs:8`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5539d32611fc3a1a87776e1f"></a>
## token

`struct_field` · `deltalake_catalog_unity::client::token::TemporaryToken::token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
token: T
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/token.rs#L11).

Source: `crates/catalog-unity/src/client/token.rs:11`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The temporary credential
