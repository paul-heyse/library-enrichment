# TokenCredential

`deltalake_catalog_unity::credential::TokenCredential`

```rust
trait TokenCredential: std::fmt::Debug + Send + Sync + 'static
```

Prose: [`api/deltalake_catalog_unity.credential.md`](../api/deltalake_catalog_unity.credential.md#tokencredential) · records: [`model/deltalake_catalog_unity.credential.json`](../model/deltalake_catalog_unity.credential.json)

## Required

Every implementation must supply these.

```rust
async fn fetch_token(&self, client: &ClientWithMiddleware) -> Result<TemporaryToken<String>, UnityCatalogError>
```

## Implementors (5)

Read one before writing your own.

- `deltalake_catalog_unity::credential::AzureCliCredential`
- `deltalake_catalog_unity::credential::ClientSecretOAuthProvider`
- `deltalake_catalog_unity::credential::ImdsManagedIdentityOAuthProvider`
- `deltalake_catalog_unity::credential::WorkloadIdentityOAuthProvider`
- `deltalake_catalog_unity::credential::WorkspaceOAuthProvider`

## Documentation

Trait for providing authorization tokens for catalog requests
