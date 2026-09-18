# `deltalake_catalog_unity::credential::AzureCliCredential`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.credential.AzureCliCredential.json).

<a id="op-c697725552220452dfd3c301"></a>
## AzureCliCredential

`struct` · `deltalake_catalog_unity::credential::AzureCliCredential` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AzureCliCredential
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L214).

Source: `crates/catalog-unity/src/credential.rs:214`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Credential for acquiring access tokens via the Azure CLI

<a id="op-205fb56875b6debc25f94ff2"></a>
## default

`function` · `deltalake_catalog_unity::credential::AzureCliCredential::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> AzureCliCredential
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L213).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::AzureCliCredential", "path": "AzureCliCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 10], "end": [213, 17], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/credential.rs:213`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3d38da7c9c1353a7d16032b"></a>
## fetch_token

`function` · `deltalake_catalog_unity::credential::AzureCliCredential::fetch_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn fetch_token(&self, _client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L228).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::AzureCliCredential", "path": "AzureCliCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 1], "end": [299, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "deltalake_catalog_unity::credential::TokenCredential", "path": "TokenCredential"}, "trait_path": "deltalake_catalog_unity::credential::TokenCredential"}`

Source: `crates/catalog-unity/src/credential.rs:228`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Fetch a token

<a id="op-c1199c2c44759d097bf08a5a"></a>
## fmt

`function` · `deltalake_catalog_unity::credential::AzureCliCredential::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L213).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::AzureCliCredential", "path": "AzureCliCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 19], "end": [213, 24], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/credential.rs:213`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a93f9f616818223183a578c4"></a>
## new

`function` · `deltalake_catalog_unity::credential::AzureCliCredential::new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L220).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::AzureCliCredential", "path": "AzureCliCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 1], "end": [223, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/credential.rs:220`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [`AzureCliCredential`](../operations/deltalake_catalog_unity.credential.AzureCliCredential.md#op-c697725552220452dfd3c301)

<a id="op-452c914e34d6761ff03563d3"></a>
## _private

`struct_field` · `deltalake_catalog_unity::credential::AzureCliCredential::_private` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
_private: ()
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L215).

Source: `crates/catalog-unity/src/credential.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
