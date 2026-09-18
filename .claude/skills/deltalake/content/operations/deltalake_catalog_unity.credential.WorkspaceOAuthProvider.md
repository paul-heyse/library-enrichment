# `deltalake_catalog_unity::credential::WorkspaceOAuthProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.credential.WorkspaceOAuthProvider.json).

<a id="op-7da9c997455a5971dbcc650f"></a>
## WorkspaceOAuthProvider

`struct` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct WorkspaceOAuthProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L64).

Source: `crates/catalog-unity/src/credential.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The same thing as the azure oauth provider, but uses the databricks api to
get tokens directly from the workspace.

<a id="op-16aa9befa8484865e4277b59"></a>
## clone

`function` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> WorkspaceOAuthProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::WorkspaceOAuthProvider", "path": "WorkspaceOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 17], "end": [63, 22], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/credential.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1162d506beda7ec22bdcc246"></a>
## fetch_token

`function` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider::fetch_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L96).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::WorkspaceOAuthProvider", "path": "WorkspaceOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [120, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "deltalake_catalog_unity::credential::TokenCredential", "path": "TokenCredential"}, "trait_path": "deltalake_catalog_unity::credential::TokenCredential"}`

Source: `crates/catalog-unity/src/credential.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2872e7c9d9de2def970b3554"></a>
## fmt

`function` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::WorkspaceOAuthProvider", "path": "WorkspaceOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 10], "end": [63, 15], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/credential.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48ab4e37f03e57ab5eae0ea9"></a>
## new

`function` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider::new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(client_id: impl Into<String>, client_secret: impl Into<String>, workspace_host: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L81).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::WorkspaceOAuthProvider", "path": "WorkspaceOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [92, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/credential.rs:81`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85c98f64f42ea5c8931bd1c8"></a>
## client_id

`struct_field` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider::client_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_id: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L66).

Source: `crates/catalog-unity/src/credential.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75923ea2f7c6c4fe3a4e18ac"></a>
## client_secret

`struct_field` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider::client_secret` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_secret: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L67).

Source: `crates/catalog-unity/src/credential.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aefcb295274472a4f69b70c3"></a>
## token_url

`struct_field` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider::token_url` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
token_url: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L65).

Source: `crates/catalog-unity/src/credential.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
