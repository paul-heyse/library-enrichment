# `deltalake_catalog_unity::UnityCatalog`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.UnityCatalog.json).

<a id="op-f5f5c6a33de68533a4509012"></a>
## UnityCatalog

`struct` · `deltalake_catalog_unity::UnityCatalog` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UnityCatalog
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L676).

Source: `crates/catalog-unity/src/lib.rs:676`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Databricks Unity Catalog

<a id="op-f37a5dd61be5a5b2ad698b44"></a>
## Error

`assoc_type` · `deltalake_catalog_unity::UnityCatalog::Error` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = UnityCatalogError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L995).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [994, 1], "end": [1027, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::data_catalog::DataCatalog", "path": "DataCatalog"}, "trait_path": "deltalake_core::data_catalog::DataCatalog"}`

Source: `crates/catalog-unity/src/lib.rs:995`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf5a1976d1e2737c7a332cdf"></a>
## fmt

`function` · `deltalake_catalog_unity::UnityCatalog::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L1030).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1029, 1], "end": [1033, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/lib.rs:1030`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbbaaefc01eb160cc77520ab"></a>
## get_schema

`function` · `deltalake_catalog_unity::UnityCatalog::get_schema` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_schema<S>(&self, catalog_name: S, schema_name: S) -> Result<GetSchemaResponse, UnityCatalogError> where S: Into<String> + Debug
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L766).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [931, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:766`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Gets the specified schema within the metastore.#

The caller must be a metastore admin, the owner of the schema,
or a user that has the USE_SCHEMA privilege on the schema.

<a id="op-fe683d5845f6789f0e1d01eb"></a>
## get_table

`function` · `deltalake_catalog_unity::UnityCatalog::get_table` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_table<S>(&self, catalog_id: S, database_name: S, table_name: S) -> Result<GetTableResponse, UnityCatalogError> where S: Into<String> + Debug
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L842).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [931, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:842`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Gets a table from the metastore for a specific catalog and schema.

The caller must be a metastore admin, be the owner of the table and have the
USE_CATALOG privilege on the parent catalog and the USE_SCHEMA privilege on
the parent schema, or be the owner of the table and have the SELECT privilege on it as well.

# Parameters

<a id="op-4063d0f57aec19fea62d72ca"></a>
## get_table_storage_location

`function` · `deltalake_catalog_unity::UnityCatalog::get_table_storage_location` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_table_storage_location(&self, catalog_id: Option<String>, database_name: &str, table_name: &str) -> Result<String, UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L997).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [994, 1], "end": [1027, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::data_catalog::DataCatalog", "path": "DataCatalog"}, "trait_path": "deltalake_core::data_catalog::DataCatalog"}`

Source: `crates/catalog-unity/src/lib.rs:997`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table storage location from the UnityCatalog

<a id="op-e177ac9e7c829f889a062ff4"></a>
## get_temp_table_credentials

`function` · `deltalake_catalog_unity::UnityCatalog::get_temp_table_credentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_temp_table_credentials<S>(&self, catalog_id: S, database_name: S, table_name: S) -> Result<TableTempCredentialsResponse, UnityCatalogError> where S: Into<String> + Debug
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L876).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [931, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:876`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d13d4080350b4ef8b5115b08"></a>
## get_temp_table_credentials_with_permission

`function` · `deltalake_catalog_unity::UnityCatalog::get_temp_table_credentials_with_permission` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_temp_table_credentials_with_permission<S>(&self, catalog_id: S, database_name: S, table_name: S, operation: &str) -> Result<TableTempCredentialsResponse, UnityCatalogError> where S: Into<String> + Debug
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L895).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [931, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:895`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f6a9072ebab7a481d750598"></a>
## list_catalogs

`function` · `deltalake_catalog_unity::UnityCatalog::list_catalogs` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn list_catalogs(&self) -> Result<ListCatalogsResponse, UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L712).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [931, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:712`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Gets an array of catalogs in the metastore. If the caller is the metastore admin,
all catalogs will be retrieved. Otherwise, only catalogs owned by the caller
(or for which the caller has the USE_CATALOG privilege) will be retrieved.
There is no guarantee of a specific ordering of the elements in the array.

<a id="op-2cdcb52a9753042c5492d2c2"></a>
## list_schemas

`function` · `deltalake_catalog_unity::UnityCatalog::list_schemas` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn list_schemas<S>(&self, catalog_name: S) -> Result<ListSchemasResponse, UnityCatalogError> where S: Into<String> + Debug
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L738).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [931, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:738`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

List all schemas for a catalog in the metastore.

If the caller is the metastore admin or the owner of the parent catalog, all schemas
for the catalog will be retrieved. Otherwise, only schemas owned by the caller
(or for which the caller has the USE_SCHEMA privilege) will be retrieved.
There is no guarantee of a specific ordering of the elements in the array.

# Parameters
- catalog_name: Parent catalog for schemas of interest.

<a id="op-fa19466870d7b207f8b380b4"></a>
## list_table_summaries

`function` · `deltalake_catalog_unity::UnityCatalog::list_table_summaries` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn list_table_summaries<S>(&self, catalog_name: S, schema_name_pattern: S) -> Result<ListTableSummariesResponse, UnityCatalogError> where S: Into<String> + Debug
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L806).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalog", "path": "UnityCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [931, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:806`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Gets an array of summaries for tables for a schema and catalog within the metastore.

The table summaries returned are either:
- summaries for all tables (within the current metastore and parent catalog and schema),
  when the user is a metastore admin, or:
- summaries for all tables and schemas (within the current metastore and parent catalog)
  for which the user has ownership or the SELECT privilege on the table and ownership or
  USE_SCHEMA privilege on the schema, provided that the user also has ownership or the
  USE_CATALOG privilege on the parent catalog.

There is no guarantee of a specific ordering of the elements in the array.

<a id="op-c57a15ac304948eeb9c29b25"></a>
## client

`struct_field` · `deltalake_catalog_unity::UnityCatalog::client` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client: reqwest_middleware::ClientWithMiddleware
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L677).

Source: `crates/catalog-unity/src/lib.rs:677`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1742dfaa7dae36f8f52c899"></a>
## credential

`struct_field` · `deltalake_catalog_unity::UnityCatalog::credential` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
credential: credential::CredentialProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L678).

Source: `crates/catalog-unity/src/lib.rs:678`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e0c4e4e65d8223679865c0c"></a>
## table_cache

`struct_field` · `deltalake_catalog_unity::UnityCatalog::table_cache` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_cache: dashmap::DashMap<String, models::GetTableResponse>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L680).

Source: `crates/catalog-unity/src/lib.rs:680`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df756e4519d5e7a85adb73e5"></a>
## workspace_url

`struct_field` · `deltalake_catalog_unity::UnityCatalog::workspace_url` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
workspace_url: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L679).

Source: `crates/catalog-unity/src/lib.rs:679`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
