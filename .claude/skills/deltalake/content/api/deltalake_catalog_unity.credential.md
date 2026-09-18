# `deltalake_catalog_unity::credential`

Crate `deltalake-catalog-unity` · 7 public items · structured records in [`model/deltalake_catalog_unity.credential.json`](../model/deltalake_catalog_unity.credential.json)

## CredentialProvider

`enum` · `deltalake_catalog_unity::credential::CredentialProvider`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.credential.CredentialProvider.md)

```rust
enum CredentialProvider
```

**Variants**: `BearerToken`, `TokenCredential`

**Derives**: Debug

Provides credentials for use when signing requests

---

## AzureCliCredential

`struct` · `deltalake_catalog_unity::credential::AzureCliCredential`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.credential.AzureCliCredential.md)

```rust
struct AzureCliCredential
```

**Implements**: `deltalake_catalog_unity::credential::TokenCredential`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `deltalake_catalog_unity::credential::TokenCredential`**

```rust
async fn fetch_token(&self, _client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

Credential for acquiring access tokens via the Azure CLI

---

## ClientSecretOAuthProvider

`struct` · `deltalake_catalog_unity::credential::ClientSecretOAuthProvider`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.credential.ClientSecretOAuthProvider.md)

```rust
struct ClientSecretOAuthProvider
```

**Implements**: `deltalake_catalog_unity::credential::TokenCredential`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(client_id: impl Into<String>, client_secret: impl Into<String>, authority_id: impl AsRef<str>, authority_host: Option<impl Into<String>>) -> Self
```

**via `deltalake_catalog_unity::credential::TokenCredential`**

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

Encapsulates the logic to perform an OAuth token challenge

---

## ImdsManagedIdentityOAuthProvider

`struct` · `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.credential.ImdsManagedIdentityOAuthProvider.md)

```rust
struct ImdsManagedIdentityOAuthProvider
```

**Implements**: `deltalake_catalog_unity::credential::TokenCredential`

**Derives**: Debug

**Methods** (1)

```rust
fn new(client_id: Option<String>, object_id: Option<String>, msi_res_id: Option<String>, msi_endpoint: Option<String>, client: ClientWithMiddleware) -> Self
```

**via `deltalake_catalog_unity::credential::TokenCredential`**

```rust
async fn fetch_token(&self, _client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

Attempts authentication using a managed identity that has been assigned to the deployment environment.

This authentication type works in Azure VMs, App Service and Azure Functions applications, as well as the Azure Cloud Shell
<https://learn.microsoft.com/en-gb/azure/active-directory/managed-identities-azure-resources/how-to-use-vm-token#get-a-token-using-http>

---

## WorkloadIdentityOAuthProvider

`struct` · `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.credential.WorkloadIdentityOAuthProvider.md)

```rust
struct WorkloadIdentityOAuthProvider
```

**Implements**: `deltalake_catalog_unity::credential::TokenCredential`

**Derives**: Debug

**Methods** (1)

```rust
fn new(client_id: impl Into<String>, federated_token_file: impl Into<String>, tenant_id: impl AsRef<str>, authority_host: Option<String>) -> Self
```

**via `deltalake_catalog_unity::credential::TokenCredential`**

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

Credential for using workload identity dfederation

<https://learn.microsoft.com/en-us/azure/active-directory/develop/workload-identity-federation>

---

## WorkspaceOAuthProvider

`struct` · `deltalake_catalog_unity::credential::WorkspaceOAuthProvider`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.credential.WorkspaceOAuthProvider.md)

```rust
struct WorkspaceOAuthProvider
```

**Implements**: `deltalake_catalog_unity::credential::TokenCredential`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(client_id: impl Into<String>, client_secret: impl Into<String>, workspace_host: impl Into<String>) -> Self
```

**via `deltalake_catalog_unity::credential::TokenCredential`**

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

The same thing as the azure oauth provider, but uses the databricks api to
get tokens directly from the workspace.

---

## TokenCredential

`trait` · `deltalake_catalog_unity::credential::TokenCredential`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.credential.TokenCredential.md)

```rust
trait TokenCredential: std::fmt::Debug + Send + Sync + 'static
```

**Implementors** (5)

- `deltalake_catalog_unity::credential::AzureCliCredential`
- `deltalake_catalog_unity::credential::ClientSecretOAuthProvider`
- `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider`
- `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider`
- `deltalake_catalog_unity::credential::WorkspaceOAuthProvider`

**Methods** (1)

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

Trait for providing authorization tokens for catalog requests

---
