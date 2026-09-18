# `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.credential.ImdsManagedIdentityOAuthProvider.json).

<a id="op-e6e9d73cbdaca914566720c9"></a>
## ImdsManagedIdentityOAuthProvider

`struct` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ImdsManagedIdentityOAuthProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L389).

Source: `crates/catalog-unity/src/credential.rs:389`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Attempts authentication using a managed identity that has been assigned to the deployment environment.

This authentication type works in Azure VMs, App Service and Azure Functions applications, as well as the Azure Cloud Shell
<https://learn.microsoft.com/en-gb/azure/active-directory/managed-identities-azure-resources/how-to-use-vm-token#get-a-token-using-http>

<a id="op-811041d213fd14800d600bd7"></a>
## fetch_token

`function` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider::fetch_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn fetch_token(&self, _client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L422).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider", "path": "ImdsManagedIdentityOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [420, 1], "end": [465, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "deltalake_catalog_unity::credential::TokenCredential", "path": "TokenCredential"}, "trait_path": "deltalake_catalog_unity::credential::TokenCredential"}`

Source: `crates/catalog-unity/src/credential.rs:422`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Fetch a token

<a id="op-755d403b01b132856f1cf83d"></a>
## fmt

`function` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L388).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider", "path": "ImdsManagedIdentityOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [388, 10], "end": [388, 15], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/credential.rs:388`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8b2f52d3f20144fff28428a"></a>
## new

`function` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider::new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(client_id: Option<String>, object_id: Option<String>, msi_res_id: Option<String>, msi_endpoint: Option<String>, client: ClientWithMiddleware) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L399).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider", "path": "ImdsManagedIdentityOAuthProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [417, 2], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/credential.rs:399`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`ImdsManagedIdentityOAuthProvider`](../operations/deltalake_catalog_unity.credential.ImdsManagedIdentityOAuthProvider.md#op-e6e9d73cbdaca914566720c9) for an azure backed store

<a id="op-5d847f39ad88a5c488439731"></a>
## client

`struct_field` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider::client` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client: reqwest_middleware::ClientWithMiddleware
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L394).

Source: `crates/catalog-unity/src/credential.rs:394`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e88ec0d625b239f9fb373d73"></a>
## client_id

`struct_field` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider::client_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_id: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L391).

Source: `crates/catalog-unity/src/credential.rs:391`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f19d0f3b4077878dde80e347"></a>
## msi_endpoint

`struct_field` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider::msi_endpoint` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
msi_endpoint: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L390).

Source: `crates/catalog-unity/src/credential.rs:390`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b28965d8c9673ef783b36f3d"></a>
## msi_res_id

`struct_field` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider::msi_res_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
msi_res_id: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L393).

Source: `crates/catalog-unity/src/credential.rs:393`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f534fc6d2574a0d3f3b5c424"></a>
## object_id

`struct_field` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider::object_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
object_id: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L392).

Source: `crates/catalog-unity/src/credential.rs:392`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
