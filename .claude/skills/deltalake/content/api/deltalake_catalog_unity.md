# `deltalake_catalog_unity`

Crate `deltalake-catalog-unity` · 7 public items · structured records in [`model/deltalake_catalog_unity.json`](../model/deltalake_catalog_unity.json)

## UnityCatalogConfigKey

`enum` · `deltalake_catalog_unity::UnityCatalogConfigKey`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.UnityCatalogConfigKey.md)

Also reachable as `deltalake::unity_catalog::UnityCatalogConfigKey`, `deltalake_catalog_unity::prelude::UnityCatalogConfigKey`

```rust
enum UnityCatalogConfigKey
```

**Variants**: `WorkspaceUrl`, `Host`, `AccessToken`, `Token`, `ClientId`, `ClientSecret`, `AuthorityId`, `AuthorityHost`, `MsiEndpoint`, `ObjectId`, `MsiResourceId`, `FederatedTokenFile`, `UseAzureCli`, `AllowHttpUrl`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Configuration options for unity catalog client

---

## UnityCatalogError

`enum` · `deltalake_catalog_unity::UnityCatalogError`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.UnityCatalogError.md)

Also reachable as `deltalake::unity_catalog::UnityCatalogError`, `deltalake_catalog_unity::prelude::UnityCatalogError`

```rust
enum UnityCatalogError
```

**Variants**: `RequestError`, `RequestMiddlewareError`, `InvalidTable`, `InvalidHeader`, `InvalidTableURI`, `MissingConfiguration`, `MissingCredential`, `TemporaryCredentialsFetchFailure`, `AzureCli`, `FederatedTokenFile`, `DatafusionError`, `InitializationError`, `Generic`, `InvalidCredentials`, `NotATable`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: DataCatalogError) -> Self
fn from(source: reqwest_middleware::Error) -> Self
fn from(source: reqwest::Error) -> Self
fn from(value: ErrorResponse) -> Self
fn from(source: ::datafusion::common::DataFusionError) -> Self
fn from(source: InvalidHeaderValue) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Possible errors from the unity-catalog/tables API call

---

## register_handlers

`function` · `deltalake_catalog_unity::register_handlers`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.register_handlers.md)

Also reachable as `deltalake::unity_catalog::register_handlers`

```rust
fn register_handlers(_additional_prefixes: Option<reqwest::Url>)
```

Register an [ObjectStoreFactory] for common UnityCatalogFactory [Url] schemes

---

## UnityCatalog

`struct` · `deltalake_catalog_unity::UnityCatalog`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.UnityCatalog.md)

Also reachable as `deltalake::unity_catalog::UnityCatalog`, `deltalake_catalog_unity::prelude::UnityCatalog`

```rust
struct UnityCatalog
```

**Implements**: `deltalake_core::data_catalog::DataCatalog`

**Derives**: Debug

**Methods** (7)

```rust
async fn get_schema<S>(&self, catalog_name: S, schema_name: S) -> Result<GetSchemaResponse, UnityCatalogError> where S: Into<String> + Debug
async fn get_table<S>(&self, catalog_id: S, database_name: S, table_name: S) -> Result<GetTableResponse, UnityCatalogError> where S: Into<String> + Debug
async fn get_temp_table_credentials<S>(&self, catalog_id: S, database_name: S, table_name: S) -> Result<TableTempCredentialsResponse, UnityCatalogError> where S: Into<String> + Debug
async fn get_temp_table_credentials_with_permission<S>(&self, catalog_id: S, database_name: S, table_name: S, operation: &str) -> Result<TableTempCredentialsResponse, UnityCatalogError> where S: Into<String> + Debug
async fn list_catalogs(&self) -> Result<ListCatalogsResponse, UnityCatalogError>
async fn list_schemas<S>(&self, catalog_name: S) -> Result<ListSchemasResponse, UnityCatalogError> where S: Into<String> + Debug
async fn list_table_summaries<S>(&self, catalog_name: S, schema_name_pattern: S) -> Result<ListTableSummariesResponse, UnityCatalogError> where S: Into<String> + Debug
```

**via `deltalake_core::data_catalog::DataCatalog`**

```rust
async fn get_table_storage_location(&self, catalog_id: Option<String>, database_name: &str, table_name: &str) -> Result<String, UnityCatalogError>
```

Databricks Unity Catalog

---

## UnityCatalogBuilder

`struct` · `deltalake_catalog_unity::UnityCatalogBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.UnityCatalogBuilder.md)

Also reachable as `deltalake::unity_catalog::UnityCatalogBuilder`, `deltalake_catalog_unity::prelude::UnityCatalogBuilder`

```rust
struct UnityCatalogBuilder
```

**Methods** (6)

```rust
fn build(self) -> DataCatalogResult<UnityCatalog>
fn builder() -> UnityCatalogBuilderBuilder<((), (), (), (), (), (), (), (), (), (), (), (), (), (), ())>
fn from_env() -> Self
async fn get_uc_location_and_token(table_uri: &str, storage_options: Option<&HashMap<String, String>>) -> Result<(String, HashMap<String, String>), UnityCatalogError>
fn try_with_option(self, key: impl AsRef<str>, value: impl Into<String>) -> DataCatalogResult<Self>
fn try_with_options<I: IntoIterator<Item = (impl AsRef<str>, impl Into<String>)>>(self, options: I) -> DataCatalogResult<Self>
```

Builder for creating a UnityCatalogClient

---

## UnityCatalogBuilderBuilder

`struct` · `deltalake_catalog_unity::UnityCatalogBuilderBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.UnityCatalogBuilderBuilder.md)

Also reachable as `deltalake::unity_catalog::UnityCatalogBuilderBuilder`

```rust
struct UnityCatalogBuilderBuilder<TypedBuilderFields = ((), (), (), (), (), (), (), (), (), (), (), (), (), (), ())>
```

**Derives**: Clone

**Methods** (16)

```rust
fn allow_http_url(self, allow_http_url: bool) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, (bool,), __retry_config, __client_options, __token_credential)>
fn authority_host(self, authority_host: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, (Option<String>,), __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn authority_id(self, authority_id: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, (Option<String>,), __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn bearer_token(self, bearer_token: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, (Option<String>,), __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn build(self) -> UnityCatalogBuilder
fn client_id(self, client_id: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, (Option<String>,), __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn client_options(self, client_options: client::ClientOptions) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, (client::ClientOptions,), __token_credential)>
fn client_secret(self, client_secret: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, (Option<String>,), __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn federated_token_file(self, federated_token_file: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, (Option<String>,), __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn msi_endpoint(self, msi_endpoint: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, (Option<String>,), __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn msi_resource_id(self, msi_resource_id: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, (Option<String>,), __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn object_id(self, object_id: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, (Option<String>,), __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
fn retry_config(self, retry_config: RetryConfig) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, (RetryConfig,), __client_options, __token_credential)>
fn token_credential(self, token_credential: Option<Box<dyn TokenCredential>>) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, (Option<Box<dyn TokenCredential>>,))>
fn use_azure_cli(self, use_azure_cli: bool) -> UnityCatalogBuilderBuilder<(__workspace_url, __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, (bool,), __allow_http_url, __retry_config, __client_options, __token_credential)>
fn workspace_url(self, workspace_url: impl ::core::convert::Into<String>) -> UnityCatalogBuilderBuilder<((Option<String>,), __bearer_token, __client_id, __client_secret, __authority_id, __authority_host, __msi_endpoint, __object_id, __msi_resource_id, __federated_token_file, __use_azure_cli, __allow_http_url, __retry_config, __client_options, __token_credential)>
```

Builder for [`UnityCatalogBuilder`] instances.

See [`UnityCatalogBuilder::builder()`] for more info.

---

## UnityCatalogFactory

`struct` · `deltalake_catalog_unity::UnityCatalogFactory`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.UnityCatalogFactory.md)

Also reachable as `deltalake::unity_catalog::UnityCatalogFactory`

```rust
struct UnityCatalogFactory
```

**Implements**: `deltalake_core::logstore::factories::LogStoreFactory`, `deltalake_core::logstore::factories::ObjectStoreFactory`

**Derives**: Clone, Debug, Default

**via `deltalake_core::logstore::factories::LogStoreFactory`**

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

**via `deltalake_core::logstore::factories::ObjectStoreFactory`**

```rust
fn parse_url_opts(&self, table_uri: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

---
