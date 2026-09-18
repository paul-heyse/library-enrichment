# `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.credential.WorkloadIdentityOAuthProvider.json).

<a id="op-c5ce808ca1b36596a3404190"></a>
## WorkloadIdentityOAuthProvider

`struct` · `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct WorkloadIdentityOAuthProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L305).

Source: `crates/catalog-unity/src/credential.rs:305`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Credential for using workload identity dfederation

<https://learn.microsoft.com/en-us/azure/active-directory/develop/workload-identity-federation>

<a id="op-92a49fb51ebfb2187ebb7c0b"></a>
## fetch_token

`function` · `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider::fetch_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L333).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider", "path": "WorkloadIdentityOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 1], "end": [365, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "deltalake_catalog_unity::credential::TokenCredential", "path": "TokenCredential"}, "trait_path": "deltalake_catalog_unity::credential::TokenCredential"}`

Source: `crates/catalog-unity/src/credential.rs:333`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Fetch a token

<a id="op-5f5b9136b8f4261763a18fe6"></a>
## fmt

`function` · `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L304).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider", "path": "WorkloadIdentityOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 10], "end": [304, 15], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/credential.rs:304`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af9ff3dee81c20ea5969067e"></a>
## new

`function` · `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider::new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(client_id: impl Into<String>, federated_token_file: impl Into<String>, tenant_id: impl AsRef<str>, authority_host: Option<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L313).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider", "path": "WorkloadIdentityOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [328, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/credential.rs:313`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`WorkloadIdentityOAuthProvider`](../operations/deltalake_catalog_unity.credential.WorkloadIdentityOAuthProvider.md#op-c5ce808ca1b36596a3404190)

<a id="op-299df131d0127d8ac09814ab"></a>
## client_id

`struct_field` · `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider::client_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_id: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L307).

Source: `crates/catalog-unity/src/credential.rs:307`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31489f936a4e6f7c792bbff2"></a>
## federated_token_file

`struct_field` · `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider::federated_token_file` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
federated_token_file: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L308).

Source: `crates/catalog-unity/src/credential.rs:308`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f952d4d380de199a233323c"></a>
## token_url

`struct_field` · `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider::token_url` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
token_url: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L306).

Source: `crates/catalog-unity/src/credential.rs:306`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
