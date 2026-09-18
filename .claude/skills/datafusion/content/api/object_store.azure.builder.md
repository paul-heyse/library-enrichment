# `object_store::azure::builder`

Crate `object_store` · 2 public items · structured records in [`model/object_store.azure.builder.json`](../model/object_store.azure.builder.json)

## AzureConfigKey

`enum` · `object_store::azure::builder::AzureConfigKey`

Also reachable as `object_store::azure::AzureConfigKey`

```rust
enum AzureConfigKey
```

**Variants**: `AccountName`, `AccessKey`, `ClientId`, `ClientSecret`, `AuthorityId`, `AuthorityHost`, `SasKey`, `Token`, `UseEmulator`, `Endpoint`, `UseFabricEndpoint`, `MsiEndpoint`, `ObjectId`, `MsiResourceId`, `FederatedTokenFile`, `UseAzureCli`, `SkipSignature`, `ContainerName`, `DisableTagging`, `FabricTokenServiceUrl`, `FabricWorkloadHost`, `FabricSessionToken`, `FabricClusterIdentifier`, `Client`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/object_store.azure.builder.AzureConfigKey.md).


Configuration keys for [`MicrosoftAzureBuilder`]

Configuration via keys can be done via [`MicrosoftAzureBuilder::with_config`]

# Example
```
# use object_store::azure::{MicrosoftAzureBuilder, AzureConfigKey};
let builder = MicrosoftAzureBuilder::new()
    .with_config("azure_client_id".parse().unwrap(), "my-client-id")
    .with_config(AzureConfigKey::AuthorityId, "my-tenant-id");
```

---

## MicrosoftAzureBuilder

`struct` · `object_store::azure::builder::MicrosoftAzureBuilder`

Also reachable as `object_store::azure::MicrosoftAzureBuilder`

```rust
struct MicrosoftAzureBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (32)

```rust
fn build(self) -> Result<MicrosoftAzure>
fn from_env() -> Self
fn get_config_value(&self, key: &AzureConfigKey) -> Option<String>
fn new() -> Self
fn with_access_key(self, access_key: impl Into<String>) -> Self
fn with_account(self, account: impl Into<String>) -> Self
fn with_allow_http(self, allow_http: bool) -> Self
fn with_authority_host(self, authority_host: impl Into<String>) -> Self
fn with_bearer_token_authorization(self, bearer_token: impl Into<String>) -> Self
fn with_client_id(self, client_id: impl Into<String>) -> Self
fn with_client_options(self, options: ClientOptions) -> Self
fn with_client_secret(self, client_secret: impl Into<String>) -> Self
fn with_client_secret_authorization(self, client_id: impl Into<String>, client_secret: impl Into<String>, tenant_id: impl Into<String>) -> Self
fn with_config(self, key: AzureConfigKey, value: impl Into<String>) -> Self
fn with_container_name(self, container_name: impl Into<String>) -> Self
fn with_credentials(self, credentials: AzureCredentialProvider) -> Self
fn with_disable_tagging(self, ignore: bool) -> Self
fn with_endpoint(self, endpoint: String) -> Self
fn with_federated_token_file(self, federated_token_file: impl Into<String>) -> Self
fn with_http_connector<C: HttpConnector>(self, connector: C) -> Self
fn with_msi_endpoint(self, msi_endpoint: impl Into<String>) -> Self
fn with_proxy_ca_certificate(self, proxy_ca_certificate: impl Into<String>) -> Self
fn with_proxy_excludes(self, proxy_excludes: impl Into<String>) -> Self
fn with_proxy_url(self, proxy_url: impl Into<String>) -> Self
fn with_retry(self, retry_config: RetryConfig) -> Self
fn with_sas_authorization(self, query_pairs: impl Into<Vec<(String, String)>>) -> Self
fn with_skip_signature(self, skip_signature: bool) -> Self
fn with_tenant_id(self, tenant_id: impl Into<String>) -> Self
fn with_url(self, url: impl Into<String>) -> Self
fn with_use_azure_cli(self, use_azure_cli: bool) -> Self
fn with_use_emulator(self, use_emulator: bool) -> Self
fn with_use_fabric_endpoint(self, use_fabric_endpoint: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.azure.builder.MicrosoftAzureBuilder.md).


Configure a connection to Microsoft Azure Blob Storage container using
the specified credentials.

# Example
```
# let ACCOUNT = "foo";
# let BUCKET_NAME = "foo";
# let ACCESS_KEY = "foo";
# use object_store::azure::MicrosoftAzureBuilder;
let azure = MicrosoftAzureBuilder::new()
 .with_account(ACCOUNT)
 .with_access_key(ACCESS_KEY)
 .with_container_name(BUCKET_NAME)
 .build();
```

---
