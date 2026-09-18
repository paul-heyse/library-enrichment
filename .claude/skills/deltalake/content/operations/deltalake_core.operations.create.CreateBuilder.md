# `deltalake_core::operations::create::CreateBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.create.CreateBuilder.json).

<a id="op-b50887bdd66591575052f481"></a>
## CreateBuilder

`struct` · `deltalake_core::operations::create::CreateBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CreateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L80).

Source: `crates/core/src/operations/create.rs:80`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build an operation to create a new [DeltaTable](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875)

<a id="op-1ba29505cabfa5565cdbf2c6"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::create::CreateBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <CreateBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L403).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [459, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/create.rs:403`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a0707e153b235b5dd8a02c0"></a>
## Output

`assoc_type` · `deltalake_core::operations::create::CreateBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L402).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [459, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/create.rs:402`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb9f7405b1b3b4d76e381a3e"></a>
## clone

`function` · `deltalake_core::operations::create::CreateBuilder::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CreateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L79).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 10], "end": [79, 15], "filename": "crates/core/src/operations/create.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/create.rs:79`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-579009792772d8ef51930dd2"></a>
## default

`function` · `deltalake_core::operations::create::CreateBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L109).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [112, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/create.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dcf6fc6533734a36bce56ad"></a>
## into_future

`function` · `deltalake_core::operations::create::CreateBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L405).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [459, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/create.rs:405`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed88b89ae6067dc7d0af7437"></a>
## new

`function` · `deltalake_core::operations::create::CreateBuilder::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L116).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:116`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`CreateBuilder`](../operations/deltalake_core.operations.create.CreateBuilder.md#op-b50887bdd66591575052f481)

<a id="op-cb95ac4e2336cd2c344584cb"></a>
## with_actions

`function` · `deltalake_core::operations::create::CreateBuilder::with_actions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_actions(self, actions: impl IntoIterator<Item = Action>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L256).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:256`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify additional actions to be added to the commit.

This method is mainly meant for internal use. Manually adding inconsistent
actions to a create operation may have undesired effects - use with caution.

<a id="op-56d033ac7869284100946e17"></a>
## with_column

`function` · `deltalake_core::operations::create::CreateBuilder::with_column` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_column(self, name: impl Into<String>, data_type: DataType, nullable: bool, metadata: Option<HashMap<String, Value>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L161).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:161`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify a column in the table

<a id="op-23966d70514a2049a940e09d"></a>
## with_columns

`function` · `deltalake_core::operations::create::CreateBuilder::with_columns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_columns(self, columns: impl IntoIterator<Item = impl Into<StructField>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L189).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify columns to append to schema

<a id="op-d44edbed8309d66566269f30"></a>
## with_comment

`function` · `deltalake_core::operations::create::CreateBuilder::with_comment` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_comment(self, comment: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L155).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Comment to describe the table.

<a id="op-5e49f8f7392be7b105e90470"></a>
## with_commit_properties

`function` · `deltalake_core::operations::create::CreateBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L241).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:241`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-6351b18831d43d235af86859"></a>
## with_configuration

`function` · `deltalake_core::operations::create::CreateBuilder::with_configuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_configuration(self, configuration: impl IntoIterator<Item = (impl Into<String>, Option<impl Into<String>>)>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L218).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:218`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set configuration on created table

<a id="op-912eb9e55e27967f0be68c2b"></a>
## with_configuration_property

`function` · `deltalake_core::operations::create::CreateBuilder::with_configuration_property` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_configuration_property(self, key: TableProperty, value: Option<impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L230).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:230`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify a table property in the table configuration

<a id="op-ca33c7af3afb42e8dd892cf2"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::create::CreateBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L268).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:268`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-abedde69f213d507f278038c"></a>
## with_location

`function` · `deltalake_core::operations::create::CreateBuilder::with_location` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_location(self, location: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L143).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the path to the location where table data is stored,
which could be a path on distributed storage.

<a id="op-9c393566d636fe7ccb96d2e8"></a>
## with_log_store

`function` · `deltalake_core::operations::create::CreateBuilder::with_log_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_log_store(self, log_store: LogStoreRef) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L262).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:262`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide a [`LogStore`] instance

<a id="op-6c6fdec0e8bdf9b37ccdca90"></a>
## with_partition_columns

`function` · `deltalake_core::operations::create::CreateBuilder::with_partition_columns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_partition_columns(self, partition_columns: impl IntoIterator<Item = impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L198).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:198`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify table partitioning

<a id="op-5d9bff0a25bb7d7e4c718173"></a>
## with_raise_if_key_not_exists

`function` · `deltalake_core::operations::create::CreateBuilder::with_raise_if_key_not_exists` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_raise_if_key_not_exists(self, raise_if_key_not_exists: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L247).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:247`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify whether to raise an error if the table properties in the configuration are not TableProperties

<a id="op-f0a654b1c7966a63f2bb5dee"></a>
## with_save_mode

`function` · `deltalake_core::operations::create::CreateBuilder::with_save_mode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_save_mode(self, save_mode: SaveMode) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L149).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:149`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the behavior when a table exists at location

<a id="op-7acbe0e7f60d972e30d316de"></a>
## with_storage_options

`function` · `deltalake_core::operations::create::CreateBuilder::with_storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_storage_options(self, storage_options: HashMap<String, String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L212).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set options used to initialize storage backend

Options may be passed in the HashMap or set as environment variables.

[crate::table::builder::s3_storage_options] describes the available options for the AWS or S3-compliant backend.
If an object store is also passed using `with_object_store()` these options will be ignored.

<a id="op-061809d224a244f1d45a5ae2"></a>
## with_table_name

`function` · `deltalake_core::operations::create::CreateBuilder::with_table_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_table_name(self, name: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L136).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [399, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/create.rs:136`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the table name. Optionally qualified with
a database name [database_name.] table_name.

<a id="op-0a76c7005e0c9e0f3fced3e5"></a>
## actions

`struct_field` · `deltalake_core::operations::create::CreateBuilder::actions` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
actions: Vec<kernel::Action>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L88).

Source: `crates/core/src/operations/create.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69ac8cebea7afe178f8ded1d"></a>
## columns

`struct_field` · `deltalake_core::operations::create::CreateBuilder::columns` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
columns: Vec<kernel::StructField>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L85).

Source: `crates/core/src/operations/create.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5216c8b417e6629ee8259e44"></a>
## comment

`struct_field` · `deltalake_core::operations::create::CreateBuilder::comment` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
comment: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L84).

Source: `crates/core/src/operations/create.rs:84`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d682e713c5921374df197a44"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::create::CreateBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L92).

Source: `crates/core/src/operations/create.rs:92`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-a6395039f6c136a4894a3554"></a>
## configuration

`struct_field` · `deltalake_core::operations::create::CreateBuilder::configuration` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
configuration: std::collections::HashMap<String, Option<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L90).

Source: `crates/core/src/operations/create.rs:90`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-550a2e2bee016ad903064439"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::create::CreateBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L94).

Source: `crates/core/src/operations/create.rs:94`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ff081ca7e39d2cf49379912"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::create::CreateBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [106, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/create.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d27cd235f8287ae99bb092b"></a>
## location

`struct_field` · `deltalake_core::operations::create::CreateBuilder::location` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
location: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L82).

Source: `crates/core/src/operations/create.rs:82`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d65b5d9f5785a445523bf79"></a>
## log_store

`function` · `deltalake_core::operations::create::CreateBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateBuilder", "path": "CreateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [106, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/create.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3fba7cb1a8d3adf4f9b95bc"></a>
## log_store

`struct_field` · `deltalake_core::operations::create::CreateBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: Option<logstore::LogStoreRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L89).

Source: `crates/core/src/operations/create.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9867da82b4bb78b54d05fe28"></a>
## mode

`struct_field` · `deltalake_core::operations::create::CreateBuilder::mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
mode: protocol::SaveMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L83).

Source: `crates/core/src/operations/create.rs:83`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-833d486413a764471018c20a"></a>
## name

`struct_field` · `deltalake_core::operations::create::CreateBuilder::name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L81).

Source: `crates/core/src/operations/create.rs:81`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dfc54a597881e656da0cd85"></a>
## partition_columns

`struct_field` · `deltalake_core::operations::create::CreateBuilder::partition_columns` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_columns: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L86).

Source: `crates/core/src/operations/create.rs:86`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b5c30338cbd6c0c705a3102"></a>
## raise_if_key_not_exists

`struct_field` · `deltalake_core::operations::create::CreateBuilder::raise_if_key_not_exists` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
raise_if_key_not_exists: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L93).

Source: `crates/core/src/operations/create.rs:93`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5367a5fc44baa7ee1c1ba7c"></a>
## storage_options

`struct_field` · `deltalake_core::operations::create::CreateBuilder::storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
storage_options: Option<std::collections::HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L87).

Source: `crates/core/src/operations/create.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
