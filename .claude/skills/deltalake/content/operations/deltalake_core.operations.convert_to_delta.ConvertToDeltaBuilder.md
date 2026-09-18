# `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.convert_to_delta.ConvertToDeltaBuilder.json).

<a id="op-5c5b486e36a7165398da88e8"></a>
## ConvertToDeltaBuilder

`struct` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ConvertToDeltaBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L107).

Source: `crates/core/src/operations/convert_to_delta.rs:107`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build an operation to convert a Parquet table to a [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) in place

<a id="op-a1baa1a5240d490c71103cfb"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <ConvertToDeltaBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L451).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [473, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/convert_to_delta.rs:451`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f7837bf5255218791d79d9f"></a>
## Output

`assoc_type` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L450).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [473, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/convert_to_delta.rs:450`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd0104e316da05871155e6bb"></a>
## default

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [126, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/convert_to_delta.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e876f7550ead666d6459b885"></a>
## into_future

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L453).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [473, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/convert_to_delta.rs:453`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a780c2b64a69b6769ef16696"></a>
## new

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L141).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:141`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`ConvertToDeltaBuilder`](../operations/deltalake_core.operations.convert_to_delta.ConvertToDeltaBuilder.md#op-5c5b486e36a7165398da88e8)

<a id="op-df9f3b49763ae730140c3f43"></a>
## with_comment

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_comment` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_comment(self, comment: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Comment to describe the table.

<a id="op-2917b42902901266bf7fd6f3"></a>
## with_commit_properties

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L244).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:244`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-2612858bba9a5b36a3d56358"></a>
## with_configuration

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_configuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_configuration(self, configuration: impl IntoIterator<Item = (impl Into<String>, Option<impl Into<String>>)>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L221).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:221`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set configuration on created table

<a id="op-6357f9263f3b225037c99c3b"></a>
## with_configuration_property

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_configuration_property` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_configuration_property(self, key: TableProperty, value: Option<impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L233).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:233`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify a table property in the table configuration

<a id="op-14cdfdc491ef5016770accf1"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L250).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:250`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-0840701bcd3d53af8ae13fd8"></a>
## with_location

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_location` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_location(self, location: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L167).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:167`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the path to the location where table data is stored,
which could be a path on distributed storage.

If an object store is also passed using `with_log_store()`, this path will be ignored.

<a id="op-d616c04f6043a5c35e5d4e5d"></a>
## with_log_store

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_log_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_log_store(self, log_store: Arc<dyn LogStore>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L158).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:158`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide a [`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) instance, that points at table location

<a id="op-71bdcc5339785c2e4c0ea920"></a>
## with_partition_schema

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_partition_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_partition_schema(self, partition_schema: impl IntoIterator<Item = StructField>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L184).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:184`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the partition schema of the Parquet table

<a id="op-12babd458b1f113b5fc38ebc"></a>
## with_partition_strategy

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_partition_strategy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_partition_strategy(self, strategy: PartitionStrategy) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L197).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:197`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the partition strategy of the Parquet table
Currently only hive-partitioning is supported for Parquet paths

<a id="op-313fe6dc0b9103725237f6fa"></a>
## with_save_mode

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_save_mode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_save_mode(self, save_mode: SaveMode) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L202).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:202`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the behavior when a table exists at location

<a id="op-327115a0a64f01eb24b3f31a"></a>
## with_storage_options

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_storage_options(self, storage_options: HashMap<String, String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L178).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:178`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set options used to initialize storage backend

Options may be passed in the HashMap or set as environment variables.

[crate::table::builder::s3_storage_options] describes the available options for the AWS or S3-compliant backend.
If an object store is also passed using `with_log_store()`, these options will be ignored.

<a id="op-27d09b04a2fd6b84b6de7d90"></a>
## with_table_name

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::with_table_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_table_name(self, name: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L209).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [447, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/convert_to_delta.rs:209`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the table name. Optionally qualified with
a database name [database_name.] table_name.

<a id="op-5945f36edc124352c43c4d9c"></a>
## comment

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::comment` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
comment: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L115).

Source: `crates/core/src/operations/convert_to_delta.rs:115`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6bcc3bca7d6981c579275ba"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L118).

Source: `crates/core/src/operations/convert_to_delta.rs:118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-00536056ae3021b281d0e88a"></a>
## configuration

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::configuration` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
configuration: std::collections::HashMap<String, Option<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L116).

Source: `crates/core/src/operations/convert_to_delta.rs:116`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04e18ab16af7478b5ee594a4"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L119).

Source: `crates/core/src/operations/convert_to_delta.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16db08a013240a8b1f8cc8a7"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L134).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [137, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/convert_to_delta.rs:134`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6261403cb7bb3a30053e53be"></a>
## location

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::location` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
location: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L109).

Source: `crates/core/src/operations/convert_to_delta.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3775cd7428a5d8379596993a"></a>
## log_store

`function` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L129).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder", "path": "ConvertToDeltaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [137, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/convert_to_delta.rs:129`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f781fdd4f1e85d23cdd5a611"></a>
## log_store

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: Option<logstore::LogStoreRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L108).

Source: `crates/core/src/operations/convert_to_delta.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfe75d4219b629c3e71fcd7a"></a>
## mode

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
mode: protocol::SaveMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L113).

Source: `crates/core/src/operations/convert_to_delta.rs:113`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b7040e52a93760a8eff126d"></a>
## name

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L114).

Source: `crates/core/src/operations/convert_to_delta.rs:114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bc1060f89468d05e3ad686a"></a>
## partition_schema

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::partition_schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_schema: std::collections::HashMap<String, kernel::StructField>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L111).

Source: `crates/core/src/operations/convert_to_delta.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d71895f500749d2ce852df6"></a>
## partition_strategy

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::partition_strategy` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_strategy: PartitionStrategy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L112).

Source: `crates/core/src/operations/convert_to_delta.rs:112`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7fcc6f5ebe9a8d768dd2eee"></a>
## storage_options

`struct_field` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder::storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
storage_options: Option<std::collections::HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L110).

Source: `crates/core/src/operations/convert_to_delta.rs:110`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
