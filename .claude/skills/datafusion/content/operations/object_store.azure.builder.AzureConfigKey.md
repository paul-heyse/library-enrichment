# `object_store::azure::builder::AzureConfigKey`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.azure.builder.AzureConfigKey.json).

<a id="op-aa00d8de7bc841f29f90ac9d"></a>
## AzureConfigKey

`enum` · `object_store::azure::builder::AzureConfigKey` · object_store 0.13.2

```rust
enum AzureConfigKey
```

Source: `src/azure/builder.rs:198`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configuration keys for [`MicrosoftAzureBuilder`](../operations/object_store.azure.builder.MicrosoftAzureBuilder.md#op-448e2696c354c14c06e02e5c)

Configuration via keys can be done via [`MicrosoftAzureBuilder::with_config`](../operations/object_store.azure.builder.MicrosoftAzureBuilder.md#op-7cd7fb1ba99bc0b236e85d94)

# Example
```
# use object_store::azure::{MicrosoftAzureBuilder, AzureConfigKey};
let builder = MicrosoftAzureBuilder::new()
    .with_config("azure_client_id".parse().unwrap(), "my-client-id")
    .with_config(AzureConfigKey::AuthorityId, "my-tenant-id");
```

<a id="op-0ed89c0e36c32076b358fd8c"></a>
## AccessKey

`variant` · `object_store::azure::builder::AzureConfigKey::AccessKey` · object_store 0.13.2

```rust
AccessKey
```

Source: `src/azure/builder.rs:215`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Master key for accessing storage account

Supported keys:
- `azure_storage_account_key`
- `azure_storage_access_key`
- `azure_storage_master_key`
- `access_key`
- `account_key`
- `master_key`

<a id="op-d65fd062c3debc5286aeaf53"></a>
## AccountName

`variant` · `object_store::azure::builder::AzureConfigKey::AccountName` · object_store 0.13.2

```rust
AccountName
```

Source: `src/azure/builder.rs:204`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The name of the azure storage account

Supported keys:
- `azure_storage_account_name`
- `account_name`

<a id="op-cdddf7808731b76165a04106"></a>
## AuthorityHost

`variant` · `object_store::azure::builder::AzureConfigKey::AuthorityHost` · object_store 0.13.2

```rust
AuthorityHost
```

Source: `src/azure/builder.rs:250`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Authority host used in oauth flows

Supported keys:
- `azure_storage_authority_host`
- `azure_authority_host`
- `authority_host`

<a id="op-f24a76ae66519cd3afb49e78"></a>
## AuthorityId

`variant` · `object_store::azure::builder::AzureConfigKey::AuthorityId` · object_store 0.13.2

```rust
AuthorityId
```

Source: `src/azure/builder.rs:242`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Tenant id used in oauth flows

Supported keys:
- `azure_storage_tenant_id`
- `azure_storage_authority_id`
- `azure_tenant_id`
- `azure_authority_id`
- `tenant_id`
- `authority_id`

<a id="op-c13551612b2f7684bcde682e"></a>
## Client

`variant` · `object_store::azure::builder::AzureConfigKey::Client` · object_store 0.13.2

```rust
Client
```

Source: `src/azure/builder.rs:384`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Client options

<a id="op-d75c70b18d67b61ba36ba5b4"></a>
## ClientId

`variant` · `object_store::azure::builder::AzureConfigKey::ClientId` · object_store 0.13.2

```rust
ClientId
```

Source: `src/azure/builder.rs:223`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Service principal client id for authorizing requests

Supported keys:
- `azure_storage_client_id`
- `azure_client_id`
- `client_id`

<a id="op-81740b1af88b453e231dc022"></a>
## ClientSecret

`variant` · `object_store::azure::builder::AzureConfigKey::ClientSecret` · object_store 0.13.2

```rust
ClientSecret
```

Source: `src/azure/builder.rs:231`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Service principal client secret for authorizing requests

Supported keys:
- `azure_storage_client_secret`
- `azure_client_secret`
- `client_secret`

<a id="op-84371ffe398d87b9ed4e72b6"></a>
## ContainerName

`variant` · `object_store::azure::builder::AzureConfigKey::ContainerName` · object_store 0.13.2

```rust
ContainerName
```

Source: `src/azure/builder.rs:344`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Container name

Supported keys:
- `azure_container_name`
- `container_name`

<a id="op-373b829cefb6eee1ee9bb94d"></a>
## DisableTagging

`variant` · `object_store::azure::builder::AzureConfigKey::DisableTagging` · object_store 0.13.2

```rust
DisableTagging
```

Source: `src/azure/builder.rs:353`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Disables tagging objects

This can be desirable if not supported by the backing store

Supported keys:
- `azure_disable_tagging`
- `disable_tagging`

<a id="op-6065a4335efed7b54f930ce9"></a>
## Endpoint

`variant` · `object_store::azure::builder::AzureConfigKey::Endpoint` · object_store 0.13.2

```rust
Endpoint
```

Source: `src/azure/builder.rs:286`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Override the endpoint used to communicate with blob storage

Supported keys:
- `azure_storage_endpoint`
- `azure_endpoint`
- `endpoint`

<a id="op-6721ae60016338b70ac0b10c"></a>
## Err

`assoc_type` · `object_store::azure::builder::AzureConfigKey::Err` · object_store 0.13.2

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [477, 2], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/azure/builder.rs:419`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abe6e3010d255e0bf2e5010b"></a>
## FabricClusterIdentifier

`variant` · `object_store::azure::builder::AzureConfigKey::FabricClusterIdentifier` · object_store 0.13.2

```rust
FabricClusterIdentifier
```

Source: `src/azure/builder.rs:381`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Fabric cluster identifier

Supported keys:
- `azure_fabric_cluster_identifier`
- `fabric_cluster_identifier`

<a id="op-78877be9961c4140efe35bee"></a>
## FabricSessionToken

`variant` · `object_store::azure::builder::AzureConfigKey::FabricSessionToken` · object_store 0.13.2

```rust
FabricSessionToken
```

Source: `src/azure/builder.rs:374`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Fabric session token

Supported keys:
- `azure_fabric_session_token`
- `fabric_session_token`

<a id="op-e978ad27947668d6305b7b85"></a>
## FabricTokenServiceUrl

`variant` · `object_store::azure::builder::AzureConfigKey::FabricTokenServiceUrl` · object_store 0.13.2

```rust
FabricTokenServiceUrl
```

Source: `src/azure/builder.rs:360`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Fabric token service url

Supported keys:
- `azure_fabric_token_service_url`
- `fabric_token_service_url`

<a id="op-b7cc849bd11b5ba0cb660d47"></a>
## FabricWorkloadHost

`variant` · `object_store::azure::builder::AzureConfigKey::FabricWorkloadHost` · object_store 0.13.2

```rust
FabricWorkloadHost
```

Source: `src/azure/builder.rs:367`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Fabric workload host

Supported keys:
- `azure_fabric_workload_host`
- `fabric_workload_host`

<a id="op-fdb5aa23f489ac25207fe927"></a>
## FederatedTokenFile

`variant` · `object_store::azure::builder::AzureConfigKey::FederatedTokenFile` · object_store 0.13.2

```rust
FederatedTokenFile
```

Source: `src/azure/builder.rs:323`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

File containing token for Azure AD workload identity federation

Supported keys:
- `azure_federated_token_file`
- `federated_token_file`

<a id="op-b6230a0e896b2e5d533f5c4f"></a>
## MsiEndpoint

`variant` · `object_store::azure::builder::AzureConfigKey::MsiEndpoint` · object_store 0.13.2

```rust
MsiEndpoint
```

Source: `src/azure/builder.rs:302`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Endpoint to request a imds managed identity token

Supported keys:
- `azure_msi_endpoint`
- `azure_identity_endpoint`
- `identity_endpoint`
- `msi_endpoint`

<a id="op-3a4ec47b93ffb2e752bab191"></a>
## MsiResourceId

`variant` · `object_store::azure::builder::AzureConfigKey::MsiResourceId` · object_store 0.13.2

```rust
MsiResourceId
```

Source: `src/azure/builder.rs:316`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Msi resource id for use with managed identity authentication

Supported keys:
- `azure_msi_resource_id`
- `msi_resource_id`

<a id="op-3d00dff22fadf42d94adeae1"></a>
## ObjectId

`variant` · `object_store::azure::builder::AzureConfigKey::ObjectId` · object_store 0.13.2

```rust
ObjectId
```

Source: `src/azure/builder.rs:309`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Object id for use with managed identity authentication

Supported keys:
- `azure_object_id`
- `object_id`

<a id="op-8c0cb0f5f28d4c2cc72c52ba"></a>
## SasKey

`variant` · `object_store::azure::builder::AzureConfigKey::SasKey` · object_store 0.13.2

```rust
SasKey
```

Source: `src/azure/builder.rs:262`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Shared access signature.

The signature is expected to be percent-encoded, much like they are provided
in the azure storage explorer or azure portal.

Supported keys:
- `azure_storage_sas_key`
- `azure_storage_sas_token`
- `sas_key`
- `sas_token`

<a id="op-ad592c6faedbfd3a7c6c072d"></a>
## SkipSignature

`variant` · `object_store::azure::builder::AzureConfigKey::SkipSignature` · object_store 0.13.2

```rust
SkipSignature
```

Source: `src/azure/builder.rs:337`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Skip signing requests

Supported keys:
- `azure_skip_signature`
- `skip_signature`

<a id="op-9f397dbe16b8ac94f0a0c336"></a>
## Token

`variant` · `object_store::azure::builder::AzureConfigKey::Token` · object_store 0.13.2

```rust
Token
```

Source: `src/azure/builder.rs:270`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Bearer token

Supported keys:
- `azure_storage_token`
- `bearer_token`
- `token`

<a id="op-01191ef8005757fc453f0edd"></a>
## UseAzureCli

`variant` · `object_store::azure::builder::AzureConfigKey::UseAzureCli` · object_store 0.13.2

```rust
UseAzureCli
```

Source: `src/azure/builder.rs:330`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Use azure cli for acquiring access token

Supported keys:
- `azure_use_azure_cli`
- `use_azure_cli`

<a id="op-6c283b98fb492a440767ad38"></a>
## UseEmulator

`variant` · `object_store::azure::builder::AzureConfigKey::UseEmulator` · object_store 0.13.2

```rust
UseEmulator
```

Source: `src/azure/builder.rs:278`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Use object store with azurite storage emulator

Supported keys:
- `azure_storage_use_emulator`
- `object_store_use_emulator`
- `use_emulator`

<a id="op-62a231fbce214e99f8682533"></a>
## UseFabricEndpoint

`variant` · `object_store::azure::builder::AzureConfigKey::UseFabricEndpoint` · object_store 0.13.2

```rust
UseFabricEndpoint
```

Source: `src/azure/builder.rs:293`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Use object store with url scheme account.dfs.fabric.microsoft.com

Supported keys:
- `azure_use_fabric_endpoint`
- `use_fabric_endpoint`

<a id="op-f40f8c8b9df06999e05288c4"></a>
## as_ref

`function` · `object_store::azure::builder::AzureConfigKey::as_ref` · object_store 0.13.2

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [416, 2], "filename": "src/azure/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/azure/builder.rs:388`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8a15460f2b81ddb204fdd9f"></a>
## clone

`function` · `object_store::azure::builder::AzureConfigKey::clone` · object_store 0.13.2

```rust
fn clone(&self) -> AzureConfigKey
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 31], "end": [196, 36], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/azure/builder.rs:196`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15c9ddb2949bc8ab291d13f8"></a>
## deserialize

`function` · `object_store::azure::builder::AzureConfigKey::deserialize` · object_store 0.13.2

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 51], "end": [196, 62], "filename": "src/azure/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/azure/builder.rs:196`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bae9198439fae9468494494"></a>
## eq

`function` · `object_store::azure::builder::AzureConfigKey::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &AzureConfigKey) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 10], "end": [196, 19], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/azure/builder.rs:196`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8907d6dfe1a6a4c3dab1133a"></a>
## fmt

`function` · `object_store::azure::builder::AzureConfigKey::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 38], "end": [196, 43], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/azure/builder.rs:196`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e53d34d72e387396a49f735b"></a>
## from_str

`function` · `object_store::azure::builder::AzureConfigKey::from_str` · object_store 0.13.2

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [477, 2], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/azure/builder.rs:421`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c743e475a0d6e1df4d782a85"></a>
## hash

`function` · `object_store::azure::builder::AzureConfigKey::hash` · object_store 0.13.2

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 25], "end": [196, 29], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/azure/builder.rs:196`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25cd559c46932f2c58f60a6a"></a>
## serialize

`function` · `object_store::azure::builder::AzureConfigKey::serialize` · object_store 0.13.2

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::builder::AzureConfigKey", "path": "AzureConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 64], "end": [196, 73], "filename": "src/azure/builder.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/azure/builder.rs:196`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
