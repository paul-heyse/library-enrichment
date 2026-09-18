# `deltalake_core::table::builder::DeltaTableBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.builder.DeltaTableBuilder.json).

<a id="op-f72aa62ca71b1bc0dba2363e"></a>
## DeltaTableBuilder

`struct` · `deltalake_core::table::builder::DeltaTableBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaTableBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L97).

Source: `crates/core/src/table/builder.rs:97`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

builder for configuring a delta table load.

<a id="op-ce34f4c8678c321fcc13b373"></a>
## build

`function` · `deltalake_core::table::builder::DeltaTableBuilder::build` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DeltaResult<DeltaTable>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L286).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:286`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build the [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) from specified options.

This will not load the log, i.e. the table is not initialized. To get an initialized
table use the `load` function

<a id="op-5f7c559a1c8286873c73463a"></a>
## build_storage

`function` · `deltalake_core::table::builder::DeltaTableBuilder::build_storage` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build_storage(&self) -> DeltaResult<LogStoreRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L261).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:261`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build a delta storage backend for the given config

<a id="op-99df433cf19e4ead64584683"></a>
## fmt

`function` · `deltalake_core::table::builder::DeltaTableBuilder::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L96).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 10], "end": [96, 15], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/table/builder.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5de65c37350770e7dbf0f500"></a>
## from_url

`function` · `deltalake_core::table::builder::DeltaTableBuilder::from_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_url(table_url: Url) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L119).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates `DeltaTableBuilder` from table URL

```rust
# use deltalake_core::table::builder::*;
# use url::Url;
let url = Url::parse("memory:///test").unwrap();
let builder = DeltaTableBuilder::from_url(url);
```

<a id="op-7e06e6511ff81a276fce5093"></a>
## load

`function` · `deltalake_core::table::builder::DeltaTableBuilder::load` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn load(self) -> DeltaResult<DeltaTable>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L291).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:291`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build the [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) and load its state

<a id="op-1c2982d1a1fe3b26e24e0137"></a>
## storage_options

`function` · `deltalake_core::table::builder::DeltaTableBuilder::storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn storage_options(&self) -> HashMap<String, String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L249).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:249`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Storage options for configuring backend object store

<a id="op-b31c48b50686c25866ec68b0"></a>
## with_allow_http

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_allow_http` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_allow_http(self, allow_http: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L237).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:237`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allows insecure connections via http.

This setting is most useful for testing / development when connecting to emulated services.

<a id="op-e878436b6c3c1818f8adf11f"></a>
## with_datestring

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_datestring` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_datestring(self, date_string: impl AsRef<str>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

specify the timestamp given as ISO-8601/RFC-3339 timestamp

<a id="op-39da8863fbead219e6ce264b"></a>
## with_io_runtime

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_io_runtime` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_io_runtime(self, io_runtime: IORuntime) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L243).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:243`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide a custom runtime handle or runtime config

<a id="op-df196fb53cbab8ce0522fa40"></a>
## with_log_buffer_size

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_log_buffer_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_log_buffer_size(self, log_buffer_size: usize) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L160).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:160`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets `log_buffer_size` to the builder

<a id="op-acaaf52a3d8fab8b1b89d8c6"></a>
## with_skip_stats

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_skip_stats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_skip_stats(self, skip_stats: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L148).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:148`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets `skip_stats` to the builder. See [`DeltaTableConfig::skip_stats`](../operations/deltalake_core.table.builder.DeltaTableConfig.md#op-0fb203006d2c47dd36967e0a)
for the impact on predicated queries.

<a id="op-b234c6f80df8cf4340cfa4fc"></a>
## with_storage_backend

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_storage_backend` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_storage_backend(self, root_storage: Arc<DynObjectStore>, location: Url) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L193).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:193`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the storage backend.

If a backend is not provided then it is derived from `location`.

# Arguments

* `root_storage` - A shared reference to an [`ObjectStore`](object_store::ObjectStore) with
  "/" pointing at the root of the object store.
* `location` - A url corresponding to the storage location of the delta table.

Unresolved upstream links (retained, not inferred): `object_store::ObjectStore`.

<a id="op-b1ea50678ecca859dcd514a2"></a>
## with_storage_options

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_storage_options(self, storage_options: HashMap<String, String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L214).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:214`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set options used to initialize storage backend

Options may be passed in the HashMap or set as environment variables. See documentation of
underlying object store implementation for details. Trailing slash will be trimmed in
the option's value to avoid failures. Trimming will only be done if one or more of below
conditions are met:
- key ends with `_URL` (e.g., `ENDPOINT_URL`, `S3_URL`, `JDBC_URL`, etc.)
- value starts with `http://`` or `https://` (e.g., `http://localhost:8000/`)

- [Azure options](https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variants)
- [S3 options](https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variants)
- [Google options](https://docs.rs/object_store/latest/object_store/gcp/enum.GoogleConfigKey.html#variants)

<a id="op-2cd46ebd827feac271dfd17c"></a>
## with_timestamp

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_timestamp(self, timestamp: DateTime<Utc>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L179).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:179`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

specify a timestamp

<a id="op-33c4712ad720696ec08669b0"></a>
## with_version

`function` · `deltalake_core::table::builder::DeltaTableBuilder::with_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_version(self, version: Version) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L154).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:154`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets `version` to the builder

<a id="op-4ef7f9ef2f4fe7bda0a97201"></a>
## without_files

`function` · `deltalake_core::table::builder::DeltaTableBuilder::without_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn without_files(self) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L141).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableBuilder", "path": "DeltaTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [301, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/builder.rs:141`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets `require_files=false` to the builder

<a id="op-779142a8779ac3a7b06acc2e"></a>
## allow_http

`struct_field` · `deltalake_core::table::builder::DeltaTableBuilder::allow_http` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
allow_http: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L106).

Source: `crates/core/src/table/builder.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-936f5d21c775c638e1278006"></a>
## storage_backend

`struct_field` · `deltalake_core::table::builder::DeltaTableBuilder::storage_backend` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
storage_backend: Option<(std::sync::Arc<object_store::DynObjectStore>, url::Url)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L101).

Source: `crates/core/src/table/builder.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

backend to access storage system

<a id="op-bcea81399b143de8d765fdae"></a>
## storage_options

`struct_field` · `deltalake_core::table::builder::DeltaTableBuilder::storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
storage_options: Option<std::collections::HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L105).

Source: `crates/core/src/table/builder.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b52423ef7b7b704c4d708cdb"></a>
## table_config

`struct_field` · `deltalake_core::table::builder::DeltaTableBuilder::table_config` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_config: DeltaTableConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L107).

Source: `crates/core/src/table/builder.rs:107`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d48e8826df98f8f0e4f8796"></a>
## table_url

`struct_field` · `deltalake_core::table::builder::DeltaTableBuilder::table_url` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_url: url::Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L99).

Source: `crates/core/src/table/builder.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

table root uri

<a id="op-665fb68393519bc36b1ff648"></a>
## version

`struct_field` · `deltalake_core::table::builder::DeltaTableBuilder::version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version: DeltaVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L104).

Source: `crates/core/src/table/builder.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

specify the version we are going to load: a time stamp, a version, or just the newest
available version
