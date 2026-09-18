# `deltalake_catalog_unity::UnityCatalogBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.UnityCatalogBuilder.json).

<a id="op-3d81dc09c647904bf0c3e078"></a>
## UnityCatalogBuilder

`struct` · `deltalake_catalog_unity::UnityCatalogBuilder` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UnityCatalogBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L357).

Source: `crates/catalog-unity/src/lib.rs:357`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Builder for creating a UnityCatalogClient

<a id="op-280756cd121d1a94931c9ecb"></a>
## build

`function` · `deltalake_catalog_unity::UnityCatalogBuilder::build` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DataCatalogResult<UnityCatalog>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L647).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogBuilder", "path": "UnityCatalogBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [673, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:647`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build an instance of [`UnityCatalog`](../operations/deltalake_catalog_unity.UnityCatalog.md#op-f5f5c6a33de68533a4509012)

<a id="op-c345d5fdc3269995771e5824"></a>
## builder

`function` · `deltalake_catalog_unity::UnityCatalogBuilder::builder` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder() -> UnityCatalogBuilderBuilder<((), (), (), (), (), (), (), (), (), (), (), (), (), (), ())>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L355).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogBuilder", "path": "UnityCatalogBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [355, 10], "end": [355, 22], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:355`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a builder for building `UnityCatalogBuilder`.
On the builder, call `.workspace_url(...)`(optional), `.bearer_token(...)`(optional), `.client_id(...)`(optional), `.client_secret(...)`(optional), `.authority_id(...)`(optional), `.authority_host(...)`(optional), `.msi_endpoint(...)`(optional), `.object_id(...)`(optional), `.msi_resource_id(...)`(optional), `.federated_token_file(...)`(optional), `.use_azure_cli(...)`(optional), `.allow_http_url(...)`(optional), `.retry_config(...)`(optional), `.client_options(...)`(optional), `.token_credential(...)`(optional) to set the values of the fields.
Finally, call `.build()` to create the instance of `UnityCatalogBuilder`.
                

<a id="op-11c5d5436d9a8bdb841cb790"></a>
## from_env

`function` · `deltalake_catalog_unity::UnityCatalogBuilder::from_env` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_env() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L473).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogBuilder", "path": "UnityCatalogBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [673, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:473`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse configuration from the environment.

Environment keys prefixed with "UNITY_" or "DATABRICKS_" will be considered

<a id="op-caccb0b27ca38b89cf82aa8b"></a>
## get_uc_location_and_token

`function` · `deltalake_catalog_unity::UnityCatalogBuilder::get_uc_location_and_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_uc_location_and_token(table_uri: &str, storage_options: Option<&HashMap<String, String>>) -> Result<(String, HashMap<String, String>), UnityCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L532).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogBuilder", "path": "UnityCatalogBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [673, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:532`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the storage location and temporary token for the Unity Catalog table.

If storage options are provided, they override environment variables for authentication.

<a id="op-3f00cfa38dea1a3a5eccf345"></a>
## try_with_option

`function` · `deltalake_catalog_unity::UnityCatalogBuilder::try_with_option` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_with_option(self, key: impl AsRef<str>, value: impl Into<String>) -> DataCatalogResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L423).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogBuilder", "path": "UnityCatalogBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [673, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:423`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set an option on the builder via a key - value pair.

<a id="op-4838543cf083fd0aa0d4626e"></a>
## try_with_options

`function` · `deltalake_catalog_unity::UnityCatalogBuilder::try_with_options` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_with_options<I: IntoIterator<Item = (impl AsRef<str>, impl Into<String>)>>(self, options: I) -> DataCatalogResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L457).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::UnityCatalogBuilder", "path": "UnityCatalogBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [673, 2], "filename": "crates/catalog-unity/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/lib.rs:457`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Hydrate builder from key value pairs.

Keys that are not recognised as Unity Catalog options are skipped, so
callers can pass a mixed config map that also contains object store
options (e.g. `aws_region`, `timeout`) destined for downstream
consumers. Use [`Self::try_with_option`](../operations/deltalake_catalog_unity.UnityCatalogBuilder.md#op-3f00cfa38dea1a3a5eccf345) to fail on unknown keys.

<a id="op-3cf9a41e94b04f06d386737a"></a>
## allow_http_url

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::allow_http_url` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
allow_http_url: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L404).

Source: `crates/catalog-unity/src/lib.rs:404`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When set to true, http will be allowed in the catalog url

<a id="op-2ee5dfa8023d87c867db2abd"></a>
## authority_host

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::authority_host` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
authority_host: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L380).

Source: `crates/catalog-unity/src/lib.rs:380`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Authority host

<a id="op-96f07833c1a5155826260b31"></a>
## authority_id

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::authority_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
authority_id: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L376).

Source: `crates/catalog-unity/src/lib.rs:376`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Tenant id

<a id="op-4d7074666470ac33a1097526"></a>
## bearer_token

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::bearer_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
bearer_token: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L364).

Source: `crates/catalog-unity/src/lib.rs:364`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Bearer token

<a id="op-b4116de4e7df83064215fa53"></a>
## client_id

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::client_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_id: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L368).

Source: `crates/catalog-unity/src/lib.rs:368`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Client id

<a id="op-2e2cfacef7072d4dc7306bdf"></a>
## client_options

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::client_options` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_options: client::ClientOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L413).

Source: `crates/catalog-unity/src/lib.rs:413`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Options for the underlying http client

<a id="op-40e191a5a775019284ec2bd3"></a>
## client_secret

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::client_secret` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client_secret: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L372).

Source: `crates/catalog-unity/src/lib.rs:372`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Client secret

<a id="op-fab9d0c9643132ed16a0c45d"></a>
## federated_token_file

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::federated_token_file` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
federated_token_file: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L396).

Source: `crates/catalog-unity/src/lib.rs:396`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File containing token for Azure AD workload identity federation

<a id="op-7cc4848af15626d1046c0c85"></a>
## msi_endpoint

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::msi_endpoint` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
msi_endpoint: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L384).

Source: `crates/catalog-unity/src/lib.rs:384`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Msi endpoint for acquiring managed identity token

<a id="op-a54ec7da25472293be14798b"></a>
## msi_resource_id

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::msi_resource_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
msi_resource_id: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L392).

Source: `crates/catalog-unity/src/lib.rs:392`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Msi resource id for use with managed identity authentication

<a id="op-0ea94b5a65c26125fdccddc4"></a>
## object_id

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::object_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
object_id: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L388).

Source: `crates/catalog-unity/src/lib.rs:388`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Object id for use with managed identity authentication

<a id="op-1bdabdc853f13c09682b9a83"></a>
## retry_config

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::retry_config` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
retry_config: deltalake_core::logstore::object_store::RetryConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L409).

Source: `crates/catalog-unity/src/lib.rs:409`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Retry config

<a id="op-98203a562c42d45005aca9ae"></a>
## token_credential

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::token_credential` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
token_credential: Option<Box<dyn TokenCredential>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L417).

Source: `crates/catalog-unity/src/lib.rs:417`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When set, this token credential will be used for acquiring access tokens

<a id="op-25011768598bd0984c67d6d7"></a>
## use_azure_cli

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::use_azure_cli` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
use_azure_cli: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L400).

Source: `crates/catalog-unity/src/lib.rs:400`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When set to true, azure cli has to be used for acquiring access token

<a id="op-2e453f885b2bc259165f5d3c"></a>
## workspace_url

`struct_field` · `deltalake_catalog_unity::UnityCatalogBuilder::workspace_url` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
workspace_url: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L360).

Source: `crates/catalog-unity/src/lib.rs:360`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Url of a Databricks workspace
