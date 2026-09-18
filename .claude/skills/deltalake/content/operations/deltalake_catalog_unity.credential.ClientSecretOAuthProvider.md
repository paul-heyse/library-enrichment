# `deltalake_catalog_unity::credential::ClientSecretOAuthProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.credential.ClientSecretOAuthProvider.json).

<a id="op-36879501159054861bd08277"></a>
## ClientSecretOAuthProvider

`struct` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ClientSecretOAuthProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L124).

Source: `crates/catalog-unity/src/credential.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Encapsulates the logic to perform an OAuth token challenge

<a id="op-4ab0d010f52a93173cb9d3f4"></a>
## clone

`function` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ClientSecretOAuthProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::ClientSecretOAuthProvider", "path": "ClientSecretOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 17], "end": [123, 22], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/credential.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33c8769a6804ac34eb2c1a78"></a>
## fetch_token

`function` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider::fetch_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L156).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::ClientSecretOAuthProvider", "path": "ClientSecretOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [180, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "deltalake_catalog_unity::credential::TokenCredential", "path": "TokenCredential"}, "trait_path": "deltalake_catalog_unity::credential::TokenCredential"}`

Source: `crates/catalog-unity/src/credential.rs:156`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Fetch a token

<a id="op-56336f5e7dae36893bf87b7f"></a>
## fmt

`function` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::ClientSecretOAuthProvider", "path": "ClientSecretOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 10], "end": [123, 15], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/credential.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2839311aec4f6411d461891"></a>
## new

`function` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider::new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(client_id: impl Into<String>, client_secret: impl Into<String>, authority_id: impl AsRef<str>, authority_host: Option<impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L132).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::ClientSecretOAuthProvider", "path": "ClientSecretOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [151, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/credential.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`ClientSecretOAuthProvider`](../operations/deltalake_catalog_unity.credential.ClientSecretOAuthProvider.md#op-36879501159054861bd08277) for an azure backed store

<a id="op-703a016c74b1f84ae78c5e24"></a>
## client_id

`struct_field` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider::client_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_id: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L126).

Source: `crates/catalog-unity/src/credential.rs:126`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec99e51ff7df9458ad5555ec"></a>
## client_secret

`struct_field` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider::client_secret` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_secret: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L127).

Source: `crates/catalog-unity/src/credential.rs:127`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-829133d2c20687e39af25bf3"></a>
## token_url

`struct_field` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider::token_url` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
token_url: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L125).

Source: `crates/catalog-unity/src/credential.rs:125`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
