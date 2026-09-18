# `deltalake_catalog_unity::UnityCatalogConfigKey`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.UnityCatalogConfigKey.json).

<a id="op-2bfaf29ec1d1dfbf92db216f"></a>
## UnityCatalogConfigKey

`enum` · `deltalake_catalog_unity::UnityCatalogConfigKey` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum UnityCatalogConfigKey
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L177).

Source: `crates/catalog-unity/src/lib.rs:177`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration options for unity catalog client

<a id="op-872a677bfc76de41adf9d6e6"></a>
## AccessToken

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::AccessToken` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AccessToken
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L200).

Source: `crates/catalog-unity/src/lib.rs:200`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Access token to authorize API requests

Supported keys:
- `unity_access_token`
- `databricks_access_token`
- `access_token`

<a id="op-7b32f4aa590380e7d963c169"></a>
## AllowHttpUrl

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::AllowHttpUrl` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AllowHttpUrl
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L277).

Source: `crates/catalog-unity/src/lib.rs:277`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow http url (e.g. http://localhost:8080/api/2.1/...)
Supported keys:
- `unity_allow_http_url`

<a id="op-c6e257e0b18a0f5d59550487"></a>
## AuthorityHost

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::AuthorityHost` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AuthorityHost
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L235).

Source: `crates/catalog-unity/src/lib.rs:235`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Authority host used in oauth flows

Supported keys:
- `azure_authority_host`
- `unity_authority_host`
- `authority_host`

<a id="op-c11723468ada64f5e4cf54c2"></a>
## AuthorityId

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::AuthorityId` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AuthorityId
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L227).

Source: `crates/catalog-unity/src/lib.rs:227`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Authority (tenant) id used in oauth flows

Supported keys:
- `azure_tenant_id`
- `unity_tenant_id`
- `tenant_id`

<a id="op-b5e785d8077a6e6f9c6a3944"></a>
## ClientId

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::ClientId` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ClientId
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L211).

Source: `crates/catalog-unity/src/lib.rs:211`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Service principal client id for authorizing requests

Supported keys:
- `azure_client_id`
- `unity_client_id`
- `client_id`

<a id="op-248e29dd2e57fc4351b72929"></a>
## ClientSecret

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::ClientSecret` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ClientSecret
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L219).

Source: `crates/catalog-unity/src/lib.rs:219`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Service principal client secret for authorizing requests

Supported keys:
- `azure_client_secret`
- `unity_client_secret`
- `client_secret`

<a id="op-4d13d0a3b36abd701c34670d"></a>
## Err

`assoc_type` · `deltalake_catalog_unity::UnityCatalogConfigKey::Err` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = DataCatalogError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L281).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogConfigKey", "path": "UnityCatalogConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 1], "end": [330, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/catalog-unity/src/lib.rs:281`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a650b2a55d7f1eea629e5304"></a>
## FederatedTokenFile

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::FederatedTokenFile` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
FederatedTokenFile
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L265).

Source: `crates/catalog-unity/src/lib.rs:265`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File containing token for Azure AD workload identity federation

Supported keys:
- `azure_federated_token_file`
- `federated_token_file`

<a id="op-ea9b90b611a1f7da46d3dad1"></a>
## Host

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::Host` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Host
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L188).

Source: `crates/catalog-unity/src/lib.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Host of the Databricks workspace

<a id="op-9242173a2307d81a83ce0552"></a>
## MsiEndpoint

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::MsiEndpoint` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MsiEndpoint
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L244).

Source: `crates/catalog-unity/src/lib.rs:244`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Endpoint to request a imds managed identity token

Supported keys:
- `azure_msi_endpoint`
- `azure_identity_endpoint`
- `identity_endpoint`
- `msi_endpoint`

<a id="op-39a42cf9f7b9023d2974d942"></a>
## MsiResourceId

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::MsiResourceId` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MsiResourceId
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L258).

Source: `crates/catalog-unity/src/lib.rs:258`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Msi resource id for use with managed identity authentication

Supported keys:
- `azure_msi_resource_id`
- `msi_resource_id`

<a id="op-bb1e3e4cbf408609bfdf780a"></a>
## ObjectId

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::ObjectId` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ObjectId
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L251).

Source: `crates/catalog-unity/src/lib.rs:251`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Object id for use with managed identity authentication

Supported keys:
- `azure_object_id`
- `object_id`

<a id="op-702270a0c88e0ed64b0ca80b"></a>
## Token

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::Token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Token
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L203).

Source: `crates/catalog-unity/src/lib.rs:203`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Token to use for Databricks Unity

<a id="op-7d8c1f59eb019b4c592c67ea"></a>
## UseAzureCli

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::UseAzureCli` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UseAzureCli
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L272).

Source: `crates/catalog-unity/src/lib.rs:272`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Use azure cli for acquiring access token

Supported keys:
- `azure_use_azure_cli`
- `use_azure_cli`

<a id="op-35498a1a134244620b7575c7"></a>
## WorkspaceUrl

`variant` · `deltalake_catalog_unity::UnityCatalogConfigKey::WorkspaceUrl` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
WorkspaceUrl
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L185).

Source: `crates/catalog-unity/src/lib.rs:185`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Url of a Databricks workspace

Supported keys:
- `unity_workspace_url`
- `databricks_workspace_url`
- `workspace_url`

<a id="op-ae83ea3c7151f6f2052b5899"></a>
## as_ref

`function` · `deltalake_catalog_unity::UnityCatalogConfigKey::as_ref` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_ref(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L334).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogConfigKey", "path": "UnityCatalogConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [352, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `crates/catalog-unity/src/lib.rs:334`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e59a97d3650bbdde73c27dca"></a>
## from_str

`function` · `deltalake_catalog_unity::UnityCatalogConfigKey::from_str` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L284).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogConfigKey", "path": "UnityCatalogConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 1], "end": [330, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/catalog-unity/src/lib.rs:284`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
