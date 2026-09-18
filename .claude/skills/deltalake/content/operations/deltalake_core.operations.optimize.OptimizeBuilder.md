# `deltalake_core::operations::optimize::OptimizeBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.OptimizeBuilder.json).

<a id="op-b5ca9f882a7416ed60fd1d9e"></a>
## OptimizeBuilder

`struct` · `deltalake_core::operations::optimize::OptimizeBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct OptimizeBuilder<'a>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L280).

Source: `crates/core/src/operations/optimize.rs:280`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optimize a Delta table with given options

If a target file size is not provided then `delta.targetFileSize` from the
table's configuration is read. Otherwise a default value is used.

<a id="op-417e3f555be71e8e0fb5370c"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::optimize::OptimizeBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <OptimizeBuilder<'a> as IntoFuture>::Output> + Send + 'a>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L417).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [415, 1], "end": [481, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/optimize.rs:417`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bff0e9f1314e94985cdd4cdc"></a>
## Output

`assoc_type` · `deltalake_core::operations::optimize::OptimizeBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(DeltaTable, Metrics), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L416).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [415, 1], "end": [481, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/optimize.rs:416`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d622ad164bab9fe2514e31de"></a>
## into_future

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L419).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [415, 1], "end": [481, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/optimize.rs:419`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cc71207e73dda51851d9754"></a>
## with_commit_properties

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L360).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:360`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to write to the commit

<a id="op-7a2ad6f4c47a4ef153deec7c"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L387).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:387`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-fac755819c5ed7352b53aa08"></a>
## with_filters

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_filters` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_filters(self, filters: &'a [FilterLiteral<'a>]) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L342).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:342`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only optimize files matching the given conjunction of `(column, op, value)`
partition filter literals

<a id="op-942916fe6e7411b421da6edd"></a>
## with_max_concurrent_tasks

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_max_concurrent_tasks` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_max_concurrent_tasks(self, max_concurrent_tasks: usize) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L375).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:375`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Max number of concurrent tasks

<a id="op-dfe174fd0fe1ffe73d4c98fd"></a>
## with_min_commit_interval

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_min_commit_interval` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_min_commit_interval(self, min_commit_interval: Duration) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L381).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:381`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Min commit interval

<a id="op-b919ffaf708535f969d9a4bd"></a>
## with_preserve_insertion_order

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_preserve_insertion_order` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_preserve_insertion_order(self, _preserve_insertion_order: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L370).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:370`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Deprecated. This setting has no effect.

<a id="op-4a7e86b7d27e5528c2101d82"></a>
## with_session_fallback_policy

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L409).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:409`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Control how delta-rs resolves the provided session when it is not a concrete `SessionState`.

Defaults to `SessionFallbackPolicy::InternalDefaults` to preserve existing behavior.

<a id="op-88d49fdfadcb6d0d6d1775da"></a>
## with_session_state

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_session_state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L401).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:401`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the DataFusion session used for planning and execution.

The provided `session` should wrap a concrete `datafusion::execution::context::SessionState`.

If `session` is not a `SessionState`, the default policy is to log a warning and fall back to
internal defaults. To make this strict (error instead), set
`with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)`.

Example: `Arc::new(create_session().state())`.

<a id="op-17215fd88c454cd2ccdba9cf"></a>
## with_target_size

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_target_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_target_size(self, target: NonZeroU64) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L348).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:348`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the target file size

<a id="op-7197eaf7da542a4421fa2def"></a>
## with_type

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_type` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_type(self, optimize_type: OptimizeType) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L335).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:335`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Choose the type of optimization to perform. Defaults to [OptimizeType::Compact](../operations/deltalake_core.operations.optimize.OptimizeType.md#op-8ac0bd12e19647702a9265ff).

<a id="op-bac9adf6b08123ddaa065dd7"></a>
## with_writer_properties

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::with_writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L354).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [413, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:354`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Writer properties passed to parquet writer

<a id="op-9219b78f48a9ae1864cd5a15"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L292).

Source: `crates/core/src/operations/optimize.rs:292`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Commit properties and configuration

<a id="op-e46a2f43699fc814d0b0cc8a"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L301).

Source: `crates/core/src/operations/optimize.rs:301`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1861842719e58b59ddf3d4f"></a>
## filters

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::filters` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
filters: &'a [FilterLiteral<'a>]
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L286).

Source: `crates/core/src/operations/optimize.rs:286`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Filters to select specific table partitions to be optimized

<a id="op-f6b0c8d3dbdc2edeb657f6e8"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L308).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [311, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/optimize.rs:308`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15f346ca1656e077c715cb12"></a>
## log_store

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L284).

Source: `crates/core/src/operations/optimize.rs:284`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-3f06e784f66bfd6865b6f43d"></a>
## log_store

`function` · `deltalake_core::operations::optimize::OptimizeBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L305).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::operations::optimize::OptimizeBuilder", "path": "OptimizeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [311, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/optimize.rs:305`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e0fc2219b4b009bce8ae479"></a>
## max_concurrent_tasks

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::max_concurrent_tasks` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_concurrent_tasks: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L294).

Source: `crates/core/src/operations/optimize.rs:294`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Maximum number of concurrent tasks (default is number of cpus)

<a id="op-90b849ab3e7105bdb981171c"></a>
## min_commit_interval

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::min_commit_interval` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
min_commit_interval: Option<std::time::Duration>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L300).

Source: `crates/core/src/operations/optimize.rs:300`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92efc6680ec04287d21ba4d8"></a>
## optimize_type

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::optimize_type` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
optimize_type: OptimizeType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L296).

Source: `crates/core/src/operations/optimize.rs:296`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optimize type

<a id="op-ff8307bf720b89ab74d81b09"></a>
## session

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session: Option<std::sync::Arc<dyn Session>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L298).

Source: `crates/core/src/operations/optimize.rs:298`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion session state relevant for executing the input plan

<a id="op-b8d7f41d92b546df84f38b00"></a>
## session_fallback_policy

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session_fallback_policy: delta_datafusion::SessionFallbackPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L299).

Source: `crates/core/src/operations/optimize.rs:299`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46df6795c46bd637746cf5da"></a>
## snapshot

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L282).

Source: `crates/core/src/operations/optimize.rs:282`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the to-be-optimized table's state

<a id="op-4a8a993855a70d692c47a325"></a>
## target_size

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::target_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
target_size: Option<std::num::NonZeroU64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L288).

Source: `crates/core/src/operations/optimize.rs:288`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Desired file size after bin-packing files

<a id="op-fe6440a6a8d328ec1d4755d0"></a>
## writer_properties

`struct_field` · `deltalake_core::operations::optimize::OptimizeBuilder::writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_properties: Option<parquet::file::properties::WriterProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L290).

Source: `crates/core/src/operations/optimize.rs:290`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties passed to underlying parquet writer
