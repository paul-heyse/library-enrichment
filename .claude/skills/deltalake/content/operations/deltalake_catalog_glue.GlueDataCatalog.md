# `deltalake_catalog_glue::GlueDataCatalog`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_glue.GlueDataCatalog.json).

<a id="op-b5a06f299ae17547739877cb"></a>
## GlueDataCatalog

`struct` · `deltalake_catalog_glue::GlueDataCatalog` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct GlueDataCatalog
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L33).

Source: `crates/catalog-glue/src/lib.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A Glue Data Catalog implement of the `Catalog` trait

<a id="op-d671ba542a3b62ef7fea89de"></a>
## Error

`assoc_type` · `deltalake_catalog_glue::GlueDataCatalog::Error` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = DataCatalogError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueDataCatalog", "path": "GlueDataCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [116, 2], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::data_catalog::DataCatalog", "path": "DataCatalog"}, "trait_path": "deltalake_core::data_catalog::DataCatalog"}`

Source: `crates/catalog-glue/src/lib.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f40c58678deabf81062a6f99"></a>
## fmt

`function` · `deltalake_catalog_glue::GlueDataCatalog::fmt` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L53).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueDataCatalog", "path": "GlueDataCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [56, 2], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-glue/src/lib.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c32cfe91977954081b811ac"></a>
## from_env

`function` · `deltalake_catalog_glue::GlueDataCatalog::from_env` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn from_env() -> Result<Self, GlueError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L39).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueDataCatalog", "path": "GlueDataCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [50, 2], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-glue/src/lib.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a new GlueDataCatalog with environmental configuration

<a id="op-b264f7e2ec11965278878bd5"></a>
## get_table_storage_location

`function` · `deltalake_catalog_glue::GlueDataCatalog::get_table_storage_location` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_table_storage_location(&self, catalog_id: Option<String>, database_name: &str, table_name: &str) -> Result<String, DataCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueDataCatalog", "path": "GlueDataCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [116, 2], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::data_catalog::DataCatalog", "path": "DataCatalog"}, "trait_path": "deltalake_core::data_catalog::DataCatalog"}`

Source: `crates/catalog-glue/src/lib.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table storage location from the Glue Data Catalog

<a id="op-6ad0ad4e815f2e77aaf46698"></a>
## with_config

`function` · `deltalake_catalog_glue::GlueDataCatalog::with_config` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_config(config: &SdkConfig) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_glue::GlueDataCatalog", "path": "GlueDataCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [50, 2], "filename": "crates/catalog-glue/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-glue/src/lib.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [GlueDataCatalog](../operations/deltalake_catalog_glue.GlueDataCatalog.md#op-b5a06f299ae17547739877cb) with the given [aws_config::SdkConfig]

Unresolved upstream links (retained, not inferred): `aws_config::SdkConfig`.

<a id="op-881e91949f657be665ff97e3"></a>
## client

`struct_field` · `deltalake_catalog_glue::GlueDataCatalog::client` · deltalake-catalog-glue 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
client: aws_sdk_glue::Client
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-glue/src/lib.rs#L34).

Source: `crates/catalog-glue/src/lib.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
