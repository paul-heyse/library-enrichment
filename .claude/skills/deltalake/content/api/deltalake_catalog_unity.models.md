# `deltalake_catalog_unity::models`

Crate `deltalake-catalog-unity` · 29 public items · structured records in [`model/deltalake_catalog_unity.models.json`](../model/deltalake_catalog_unity.models.json)

## CatalogType

`enum` · `deltalake_catalog_unity::models::CatalogType`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.CatalogType.md)

```rust
enum CatalogType
```

**Variants**: `Undefined`, `ManagedCatalog`, `DeltasharingCatalog`, `SystemCatalog`, `InternalCatalog`, `ForeignCatalog`, `ManagedOnlineCatalog`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

The type of the catalog.

---

## ColumnTypeName

`enum` · `deltalake_catalog_unity::models::ColumnTypeName`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ColumnTypeName.md)

```rust
enum ColumnTypeName
```

**Variants**: `Boolean`, `Byte`, `Short`, `Int`, `Long`, `Float`, `Double`, `Date`, `Timestamp`, `TimestampNtz`, `String`, `Binary`, `Decimal`, `Interval`, `Array`, `Struct`, `Map`, `Char`, `Null`, `UserDefinedType`, `TableType`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## DataSourceFormat

`enum` · `deltalake_catalog_unity::models::DataSourceFormat`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.DataSourceFormat.md)

```rust
enum DataSourceFormat
```

**Variants**: `Undefined`, `Delta`, `Csv`, `Json`, `Avro`, `Parquet`, `Orc`, `Text`, `UnityCatalog`, `Deltasharing`, `DatabricksFormat`, `MySQLFormat`, `PostgreSQLFormat`, `RedshiftFormat`, `SnowflakeFormat`, `SQLDWFormat`, `SQLServerFormat`, `SalesForceFormat`, `BigQueryFormat`, `NetSuiteFormat`, `WorkdayRAASFormat`, `HiveSerde`, `HiveCustom`, `VectorIndexFormat`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Possible data source formats for unity tables

---

## GetSchemaResponse

`enum` · `deltalake_catalog_unity::models::GetSchemaResponse`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.GetSchemaResponse.md)

```rust
enum GetSchemaResponse
```

**Variants**: `Success`, `Error`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

List schemas response

---

## GetTableResponse

`enum` · `deltalake_catalog_unity::models::GetTableResponse`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.GetTableResponse.md)

```rust
enum GetTableResponse
```

**Variants**: `Success`, `Error`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Get table response

---

## IsolationMode

`enum` · `deltalake_catalog_unity::models::IsolationMode`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.IsolationMode.md)

```rust
enum IsolationMode
```

**Variants**: `Undefined`, `Open`, `Isolated`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Whether the current securable is accessible from all workspaces or a specific set of workspaces.

---

## ListCatalogsResponse

`enum` · `deltalake_catalog_unity::models::ListCatalogsResponse`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ListCatalogsResponse.md)

```rust
enum ListCatalogsResponse
```

**Variants**: `Success`, `Error`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

List catalogs response

---

## ListSchemasResponse

`enum` · `deltalake_catalog_unity::models::ListSchemasResponse`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ListSchemasResponse.md)

```rust
enum ListSchemasResponse
```

**Variants**: `Success`, `Error`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

List schemas response

---

## ListTableSummariesResponse

`enum` · `deltalake_catalog_unity::models::ListTableSummariesResponse`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ListTableSummariesResponse.md)

```rust
enum ListTableSummariesResponse
```

**Variants**: `Success`, `Error`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

List table summaries response

---

## ProvisioningState

`enum` · `deltalake_catalog_unity::models::ProvisioningState`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ProvisioningState.md)

```rust
enum ProvisioningState
```

**Variants**: `Provisioning`, `Active`, `Failed`, `Deleting`, `Updating`, `Degraded`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## TableTempCredentialsResponse

`enum` · `deltalake_catalog_unity::models::TableTempCredentialsResponse`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.TableTempCredentialsResponse.md)

```rust
enum TableTempCredentialsResponse
```

**Variants**: `Success`, `Error`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## TableType

`enum` · `deltalake_catalog_unity::models::TableType`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.TableType.md)

```rust
enum TableType
```

**Variants**: `Undefined`, `Managed`, `External`, `View`, `MaterializedView`, `StreamingTable`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Possible data source formats for unity tables

---

## AwsTempCredentials

`struct` · `deltalake_catalog_unity::models::AwsTempCredentials`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.AwsTempCredentials.md)

```rust
struct AwsTempCredentials
```

**Fields**: `access_key_id`, `secret_access_key`, `session_token`, `access_point`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## AzureUserDelegationSas

`struct` · `deltalake_catalog_unity::models::AzureUserDelegationSas`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.AzureUserDelegationSas.md)

```rust
struct AzureUserDelegationSas
```

**Fields**: `sas_token`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## Catalog

`struct` · `deltalake_catalog_unity::models::Catalog`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.Catalog.md)

```rust
struct Catalog
```

**Fields**: `created_by`, `name`, `updated_by`, `isolation_mode`, `catalog_type`, `storage_root`, `provider_name`, `storage_location`, `properties`, `share_name`, `comment`, `created_at`, `owner`, `updated_at`, `metastore_id`, `enabled_predictive_optimization`, `effective_predictive_optimization_flag`, `connection_name`, `full_name`, `options`, `securable_type`, `provisioning_info`, `browse_only`, `accessible_in_current_workspace`, `id`, `securable_kind`, `delta_sharing_valid_through_timestamp`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

A catalog within a metastore

---

## ColumnInfo

`struct` · `deltalake_catalog_unity::models::ColumnInfo`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ColumnInfo.md)

```rust
struct ColumnInfo
```

**Fields**: `name`, `type_text`, `type_json`, `type_name`, `type_precision`, `type_scale`, `type_interval_type`, `position`, `comment`, `nullable`, `partition_index`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## DeltaRuntimeProperties

`struct` · `deltalake_catalog_unity::models::DeltaRuntimeProperties`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.DeltaRuntimeProperties.md)

```rust
struct DeltaRuntimeProperties
```

**Fields**: `delta_runtime_properties`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## EffectivePredictiveOptimizationFlag

`struct` · `deltalake_catalog_unity::models::EffectivePredictiveOptimizationFlag`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.EffectivePredictiveOptimizationFlag.md)

```rust
struct EffectivePredictiveOptimizationFlag
```

**Fields**: `value`, `inherited_from_type`, `inherited_from_name`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## ErrorDetails

`struct` · `deltalake_catalog_unity::models::ErrorDetails`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ErrorDetails.md)

```rust
struct ErrorDetails
```

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## ErrorResponse

`struct` · `deltalake_catalog_unity::models::ErrorResponse`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ErrorResponse.md)

```rust
struct ErrorResponse
```

**Fields**: `error_code`, `message`, `details`

**Implements**: `core::error::Error`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Error response from unity API

---

## GcpOauthToken

`struct` · `deltalake_catalog_unity::models::GcpOauthToken`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.GcpOauthToken.md)

```rust
struct GcpOauthToken
```

**Fields**: `oauth_token`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## ProvisioningInfo

`struct` · `deltalake_catalog_unity::models::ProvisioningInfo`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.ProvisioningInfo.md)

```rust
struct ProvisioningInfo
```

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## R2TempCredentials

`struct` · `deltalake_catalog_unity::models::R2TempCredentials`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.R2TempCredentials.md)

```rust
struct R2TempCredentials
```

**Fields**: `access_key_id`, `secret_access_key`, `session_token`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## Schema

`struct` · `deltalake_catalog_unity::models::Schema`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.Schema.md)

```rust
struct Schema
```

**Fields**: `created_by`, `name`, `updated_by`, `full_name`, `catalog_type`, `catalog_name`, `storage_root`, `storage_location`, `properties`, `comment`, `created_at`, `owner`, `updated_at`, `metastore_id`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

A schema within a catalog

---

## Table

`struct` · `deltalake_catalog_unity::models::Table`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.Table.md)

```rust
struct Table
```

**Fields**: `name`, `catalog_name`, `schema_name`, `table_type`, `data_source_format`, `columns`, `storage_location`, `comment`, `properties`, `created_at`, `updated_at`, `table_id`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

A table within a schema

---

## TableSummary

`struct` · `deltalake_catalog_unity::models::TableSummary`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.TableSummary.md)

```rust
struct TableSummary
```

**Fields**: `full_name`, `table_type`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Summary of the table

---

## TemporaryTableCredentials

`struct` · `deltalake_catalog_unity::models::TemporaryTableCredentials`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.TemporaryTableCredentials.md)

```rust
struct TemporaryTableCredentials
```

**Fields**: `aws_temp_credentials`, `azure_user_delegation_sas`, `gcp_oauth_token`, `r2_temp_credentials`, `expiration_time`, `url`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn get_aws_credentials(&self) -> Option<HashMap<String, String>>
fn get_azure_credentials(&self) -> Option<HashMap<String, String>>
fn get_credentials(self) -> Option<HashMap<String, String>>
fn get_gcp_credentials(&self) -> Option<HashMap<String, String>>
fn get_r2_credentials(&self) -> Option<HashMap<String, String>>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## TemporaryTableCredentialsRequest

`struct` · `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.TemporaryTableCredentialsRequest.md)

```rust
struct TemporaryTableCredentialsRequest
```

**Fields**: `table_id`, `operation`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(table_id: &str, operation: &str) -> Self
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## TokenErrorResponse

`struct` · `deltalake_catalog_unity::models::TokenErrorResponse`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.models.TokenErrorResponse.md)

```rust
struct TokenErrorResponse
```

**Fields**: `error`, `error_id`, `error_description`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---
