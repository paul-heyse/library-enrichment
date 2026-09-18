# `object_store::client::StaticCredentialProvider`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.StaticCredentialProvider.json).

<a id="op-82735c7c173fb6ef2a92935e"></a>
## StaticCredentialProvider

`struct` · `object_store::client::StaticCredentialProvider` · object_store 0.13.2

```rust
struct StaticCredentialProvider<T>
```

Source: `src/client/mod.rs:917`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A static set of credentials

<a id="op-9d9ebc743c68e049f0e0134e"></a>
## Credential

`assoc_type` · `object_store::client::StaticCredentialProvider::Credential` · object_store 0.13.2

```rust
Credential
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::client::StaticCredentialProvider", "path": "StaticCredentialProvider"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "std::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [931, 1], "end": [940, 2], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "object_store::client::CredentialProvider", "path": "CredentialProvider"}, "trait_path": "object_store::client::CredentialProvider"}`

Source: `src/client/mod.rs:935`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0336d1e0e81e52bc72c1b3aa"></a>
## fmt

`function` · `object_store::client::StaticCredentialProvider::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::client::StaticCredentialProvider", "path": "StaticCredentialProvider"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [916, 10], "end": [916, 15], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/mod.rs:916`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54d2aa11e10d5f2bdbd03350"></a>
## get_credential

`function` · `object_store::client::StaticCredentialProvider::get_credential` · object_store 0.13.2

```rust
async fn get_credential(&self) -> Result<Arc<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::client::StaticCredentialProvider", "path": "StaticCredentialProvider"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "std::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [931, 1], "end": [940, 2], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "object_store::client::CredentialProvider", "path": "CredentialProvider"}, "trait_path": "object_store::client::CredentialProvider"}`

Source: `src/client/mod.rs:937`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbc8118652eb09c1d0aba184"></a>
## new

`function` · `object_store::client::StaticCredentialProvider::new` · object_store 0.13.2

```rust
fn new(credential: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::client::StaticCredentialProvider", "path": "StaticCredentialProvider"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 1], "end": [928, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:923`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A [`CredentialProvider`](../operations/object_store.client.CredentialProvider.md#op-76004bbb5f119134cca4253a) for a static credential of type `T`
