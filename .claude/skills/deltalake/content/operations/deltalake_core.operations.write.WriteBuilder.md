# `deltalake_core::operations::write::WriteBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.write.WriteBuilder.json).

<a id="op-fa55ac4ec569efc5b5c82899"></a>
## WriteBuilder

`struct` · `deltalake_core::operations::write::WriteBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct WriteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L131).

Source: `crates/core/src/operations/write/mod.rs:131`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write data into a DeltaTable

<a id="op-072ecfffadd68f9e851e93a8"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::write::WriteBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <WriteBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L472).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 1], "end": [649, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/write/mod.rs:472`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b3adb069c90d8711cc194f5"></a>
## Output

`assoc_type` · `deltalake_core::operations::write::WriteBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L471).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 1], "end": [649, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/write/mod.rs:471`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e207c0fb1deca836e264322d"></a>
## into_future

`function` · `deltalake_core::operations::write::WriteBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L474).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 1], "end": [649, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/write/mod.rs:474`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e864ea1eb0c491afba02484f"></a>
## new

`function` · `deltalake_core::operations::write::WriteBuilder::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(log_store: LogStoreRef, snapshot: Option<EagerSnapshot>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L198).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:198`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`WriteBuilder`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-fa55ac4ec569efc5b5c82899)

<a id="op-8c216c90e09068c23b8f3afa"></a>
## with_cast_safety

`function` · `deltalake_core::operations::write::WriteBuilder::with_cast_safety` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_cast_safety(self, safe: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L299).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:299`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the safety of the casting operation
how to handle cast failures, either return NULL (safe=true) or return ERR (safe=false)

<a id="op-e9145ea9173ea093f8f03b9c"></a>
## with_commit_properties

`function` · `deltalake_core::operations::write::WriteBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L311).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:311`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-a9bbcc4a404c3e80aa8fe7c3"></a>
## with_configuration

`function` · `deltalake_core::operations::write::WriteBuilder::with_configuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_configuration(self, configuration: impl IntoIterator<Item = (impl Into<String>, Option<impl Into<String>>)>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L336).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:336`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set configuration on created table

<a id="op-a2d7c2a33a0ad647e936c3cb"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::write::WriteBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L330).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:330`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-c5cd53dabebdda02dc12dbbd"></a>
## with_description

`function` · `deltalake_core::operations::write::WriteBuilder::with_description` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_description(self, description: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:324`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Comment to describe the table.

<a id="op-51d57ac4ecfdcbc85f4daf47"></a>
## with_input_batches

`function` · `deltalake_core::operations::write::WriteBuilder::with_input_batches` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_input_batches(self, batches: impl IntoIterator<Item = RecordBatch>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L348).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:348`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execution plan that produces the data to be written to the delta table

<a id="op-ae52f6af0c315495372b5b85"></a>
## with_input_execution_plan

`function` · `deltalake_core::operations::write::WriteBuilder::with_input_execution_plan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_input_execution_plan(self, plan: Arc<LogicalPlan>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L253).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:253`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Logical execution plan that produces the data to be written to the delta table

<a id="op-96e3a149764f7356c388c2fc"></a>
## with_input_plan

`function` · `deltalake_core::operations::write::WriteBuilder::with_input_plan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_input_plan(self, plan: LogicalPlan) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L258).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:258`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Logical plan that produces the data to be written to the delta table

<a id="op-3c7a11e509dfe629ee2c3f13"></a>
## with_partition_columns

`function` · `deltalake_core::operations::write::WriteBuilder::with_partition_columns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_partition_columns(self, partition_columns: impl IntoIterator<Item = impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L243).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:243`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

(Optional) Specify table partitioning. For existing tables this must match the
current partitioning, except full table overwrite with schema overwrite and
no replaceWhere predicate may replace the partitioning. For new tables, the
partitioning is applied.

<a id="op-fe70b99393fab335b2454bd5"></a>
## with_replace_where

`function` · `deltalake_core::operations::write::WriteBuilder::with_replace_where` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_replace_where(self, predicate: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L234).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:234`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When using `Overwrite` mode, replace data that matches a predicate

<a id="op-a2d377816dc436cf1c93443b"></a>
## with_save_mode

`function` · `deltalake_core::operations::write::WriteBuilder::with_save_mode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_save_mode(self, save_mode: SaveMode) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:222`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

**Reference annotation (source_runtime_reconciliation, separate from upstream):** The retained existing-table write probe shows SaveMode::Ignore appending rows at this commit. The enum name is not a skip-existing guarantee for this operation. [Evidence](../capabilities/delta.write.md).

Specify the behavior when a table exists at location

<a id="op-290b8fe3fd386ffca8f17f0b"></a>
## with_schema_mode

`function` · `deltalake_core::operations::write::WriteBuilder::with_schema_mode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema_mode(self, schema_mode: SchemaMode) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L228).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:228`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add Schema Write Mode

<a id="op-b97917aa9ff6c7d66ed99cbd"></a>
## with_session_fallback_policy

`function` · `deltalake_core::operations::write::WriteBuilder::with_session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L280).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:280`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Control how delta-rs resolves the provided session when it is not a concrete `SessionState`.

Defaults to `SessionFallbackPolicy::InternalDefaults` to preserve existing behavior.

<a id="op-c287ad75f18e28c8f06ebb86"></a>
## with_session_state

`function` · `deltalake_core::operations::write::WriteBuilder::with_session_state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L272).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:272`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the DataFusion session used for planning and execution.

The provided `session` should wrap a concrete `datafusion::execution::context::SessionState`.

If `session` is not a `SessionState`, the default policy is to log a warning and fall back to
internal defaults. To make this strict (error instead), set
`with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)`.

Example: `Arc::new(create_session().state())`.

<a id="op-e05a7c386b4282f26e83c88c"></a>
## with_table_name

`function` · `deltalake_core::operations::write::WriteBuilder::with_table_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_table_name(self, name: impl Into<String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L318).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:318`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the table name. Optionally qualified with
a database name [database_name.] table_name.

<a id="op-c37d0fc2debcbca282d6cb24"></a>
## with_target_file_size

`function` · `deltalake_core::operations::write::WriteBuilder::with_target_file_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_target_file_size(self, target_file_size: Option<NonZeroU64>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L286).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:286`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the target file size for data files written to the delta table.

<a id="op-4c5dc6c082a0a935cba468e1"></a>
## with_write_batch_size

`function` · `deltalake_core::operations::write::WriteBuilder::with_write_batch_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_write_batch_size(self, write_batch_size: usize) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L292).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:292`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the target batch size for row groups written to parquet files.

<a id="op-c717fd4ab339af34d1162338"></a>
## with_writer_properties

`function` · `deltalake_core::operations::write::WriteBuilder::with_writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L305).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [468, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/mod.rs:305`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the writer properties to use when writing a parquet file

<a id="op-1bfa9199a68d0d70bd6d28b3"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::write::WriteBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L159).

Source: `crates/core/src/operations/write/mod.rs:159`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-da23c0af7115bc6053ebca80"></a>
## configuration

`struct_field` · `deltalake_core::operations::write::WriteBuilder::configuration` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
configuration: std::collections::HashMap<String, Option<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L165).

Source: `crates/core/src/operations/write/mod.rs:165`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configurations of the delta table, only used when table doesn't exist

<a id="op-39a5c191c60a443ec8cd739e"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::write::WriteBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L166).

Source: `crates/core/src/operations/write/mod.rs:166`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb8fdfc3834b6b3dcce0ab69"></a>
## description

`struct_field` · `deltalake_core::operations::write::WriteBuilder::description` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
description: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L163).

Source: `crates/core/src/operations/write/mod.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Description of the table, only used when table doesn't exist yet

<a id="op-ef10b62f633ab33b7b47260f"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::write::WriteBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L191).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [194, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/write/mod.rs:191`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e7d9168d152ef6c36c1c67f"></a>
## input

`struct_field` · `deltalake_core::operations::write::WriteBuilder::input` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
input: Option<datafusion::logical_expr::LogicalPlan>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L137).

Source: `crates/core/src/operations/write/mod.rs:137`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The input plan

<a id="op-1690f2ad0759111a154241b4"></a>
## log_store

`function` · `deltalake_core::operations::write::WriteBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L188).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteBuilder", "path": "WriteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [194, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/write/mod.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce3150872410b3c534f1e28d"></a>
## log_store

`struct_field` · `deltalake_core::operations::write::WriteBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L135).

Source: `crates/core/src/operations/write/mod.rs:135`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-e22ef3e9473f73640ea322f9"></a>
## mode

`struct_field` · `deltalake_core::operations::write::WriteBuilder::mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
mode: protocol::SaveMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L142).

Source: `crates/core/src/operations/write/mod.rs:142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

SaveMode defines how to treat data already written to table location

<a id="op-ec3ca4288fb86d4690d1f966"></a>
## name

`struct_field` · `deltalake_core::operations::write::WriteBuilder::name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L161).

Source: `crates/core/src/operations/write/mod.rs:161`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of the table, only used when table doesn't exist yet

<a id="op-034205579424ff13bb46edce"></a>
## partition_columns

`struct_field` · `deltalake_core::operations::write::WriteBuilder::partition_columns` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_columns: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L144).

Source: `crates/core/src/operations/write/mod.rs:144`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Column names for table partitioning

<a id="op-cad6df2ace99ca9ee4e64f8d"></a>
## predicate

`struct_field` · `deltalake_core::operations::write::WriteBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L146).

Source: `crates/core/src/operations/write/mod.rs:146`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When using `Overwrite` mode, replace data that matches a predicate

<a id="op-7f6d96fbb36bdf48da859281"></a>
## safe_cast

`struct_field` · `deltalake_core::operations::write::WriteBuilder::safe_cast` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
safe_cast: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L155).

Source: `crates/core/src/operations/write/mod.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

how to handle cast failures, either return NULL (safe=true) or return ERR (safe=false)

<a id="op-fe365459e77940fda330282b"></a>
## schema_mode

`struct_field` · `deltalake_core::operations::write::WriteBuilder::schema_mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
schema_mode: Option<SchemaMode>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L153).

Source: `crates/core/src/operations/write/mod.rs:153`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

whether to overwrite the schema or to merge it. None means to fail on schmema drift

<a id="op-38126ec3544cddd70393198e"></a>
## session

`struct_field` · `deltalake_core::operations::write::WriteBuilder::session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session: Option<std::sync::Arc<dyn Session>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L139).

Source: `crates/core/src/operations/write/mod.rs:139`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion session state relevant for executing the input plan

<a id="op-c00c7752d2f37566478fa89f"></a>
## session_fallback_policy

`struct_field` · `deltalake_core::operations::write::WriteBuilder::session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session_fallback_policy: delta_datafusion::SessionFallbackPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L140).

Source: `crates/core/src/operations/write/mod.rs:140`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecc386bf85f4758211d23e8d"></a>
## snapshot

`struct_field` · `deltalake_core::operations::write::WriteBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L133).

Source: `crates/core/src/operations/write/mod.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the to-be-loaded table's state

<a id="op-16b92715b0a0f6a497cd14c6"></a>
## target_file_size

`struct_field` · `deltalake_core::operations::write::WriteBuilder::target_file_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
target_file_size: Option<Option<std::num::NonZeroU64>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L149).

Source: `crates/core/src/operations/write/mod.rs:149`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Size above which we will write a buffered parquet file to disk.
If None, the writer will not create a new file until the writer is closed.

<a id="op-cd46da2b9eaf329b0a025944"></a>
## write_batch_size

`struct_field` · `deltalake_core::operations::write::WriteBuilder::write_batch_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
write_batch_size: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L151).

Source: `crates/core/src/operations/write/mod.rs:151`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of records to be written in single batch to underlying writer

<a id="op-9880ffdd13b8c69b970f5f0d"></a>
## writer_properties

`struct_field` · `deltalake_core::operations::write::WriteBuilder::writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_properties: Option<parquet::file::properties::WriterProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L157).

Source: `crates/core/src/operations/write/mod.rs:157`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parquet writer properties
