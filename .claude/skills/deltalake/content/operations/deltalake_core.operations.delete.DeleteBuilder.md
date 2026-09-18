# `deltalake_core::operations::delete::DeleteBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.delete.DeleteBuilder.json).

<a id="op-ac479c23ed9c87e115cccbc0"></a>
## DeleteBuilder

`struct` · `deltalake_core::operations::delete::DeleteBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeleteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L97).

Source: `crates/core/src/operations/delete.rs:97`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete Records from the Delta Table.
See this module's documentation for more information

<a id="op-29b821496110c0d9c02e4273"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::delete::DeleteBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <DeleteBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L291).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [378, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/delete.rs:291`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e74c7e65dd2b30f69a94a0bb"></a>
## Output

`assoc_type` · `deltalake_core::operations::delete::DeleteBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(DeltaTable, DeleteMetrics), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L290).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [378, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/delete.rs:290`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16ea2eb6d9e2314cb300ec53"></a>
## clone

`function` · `deltalake_core::operations::delete::DeleteBuilder::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeleteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L96).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 10], "end": [96, 15], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/delete.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfa8537064a604bc3eaedfa1"></a>
## fmt

`function` · `deltalake_core::operations::delete::DeleteBuilder::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L115).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [123, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/delete.rs:115`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-922d0f703d0faf42689e2f6f"></a>
## into_future

`function` · `deltalake_core::operations::delete::DeleteBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L293).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [378, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/delete.rs:293`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c102668a752bf82c578f8e5e"></a>
## with_commit_properties

`function` · `deltalake_core::operations::delete::DeleteBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L271).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [287, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/delete.rs:271`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to write to the commit

<a id="op-71e45344e99dba850d281562"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::delete::DeleteBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L283).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [287, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/delete.rs:283`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-4043454af6cd14b4202939f0"></a>
## with_predicate

`function` · `deltalake_core::operations::delete::DeleteBuilder::with_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_predicate<E: Into<Expression>>(self, predicate: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L243).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [287, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/delete.rs:243`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A predicate that determines if a record is deleted

<a id="op-b755f3fb37b446eca89d1872"></a>
## with_session_fallback_policy

`function` · `deltalake_core::operations::delete::DeleteBuilder::with_session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L265).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [287, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/delete.rs:265`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Control how delta-rs resolves the provided session when it is not a concrete `SessionState`.

Defaults to `SessionFallbackPolicy::InternalDefaults` to preserve existing behavior.

<a id="op-3d3420d967124d71544d1557"></a>
## with_session_state

`function` · `deltalake_core::operations::delete::DeleteBuilder::with_session_state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L257).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [287, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/delete.rs:257`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the DataFusion session used for planning and execution.

The provided `session` should wrap a concrete `datafusion::execution::context::SessionState`.

If `session` is not a `SessionState`, the default policy is to log a warning and fall back to
internal defaults. To make this strict (error instead), set
`with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)`.

Example: `Arc::new(create_session().state())`.

<a id="op-139bef07733b675b079c7145"></a>
## with_writer_properties

`function` · `deltalake_core::operations::delete::DeleteBuilder::with_writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L277).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [287, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/delete.rs:277`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Writer properties passed to parquet writer for when files are rewritten

<a id="op-a15b41a0b9a4fec0b3a166f5"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::delete::DeleteBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L110).

Source: `crates/core/src/operations/delete.rs:110`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Commit properties and configuration

<a id="op-73a3f93ce59dc41c062f844d"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::delete::DeleteBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L111).

Source: `crates/core/src/operations/delete.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-502b18e32c6c081ef384db95"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::delete::DeleteBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 1], "end": [225, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/delete.rs:222`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06ce8512045b1b3039120b37"></a>
## log_store

`function` · `deltalake_core::operations::delete::DeleteBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L219).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 1], "end": [225, 2], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/delete.rs:219`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbd413238d61a24378d07a57"></a>
## log_store

`struct_field` · `deltalake_core::operations::delete::DeleteBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L103).

Source: `crates/core/src/operations/delete.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-ec99b648708b207756e06b25"></a>
## predicate

`struct_field` · `deltalake_core::operations::delete::DeleteBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L99).

Source: `crates/core/src/operations/delete.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Which records to delete

<a id="op-7c2cfc5cb1bb3396862953cc"></a>
## session

`struct_field` · `deltalake_core::operations::delete::DeleteBuilder::session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session: Option<std::sync::Arc<dyn Session>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L105).

Source: `crates/core/src/operations/delete.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion session state relevant for executing the input plan

<a id="op-eb656cfaa7ab00a8e3610da2"></a>
## session_fallback_policy

`struct_field` · `deltalake_core::operations::delete::DeleteBuilder::session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session_fallback_policy: delta_datafusion::SessionFallbackPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L106).

Source: `crates/core/src/operations/delete.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dcc0b5904ba7df07e0a7823"></a>
## snapshot

`struct_field` · `deltalake_core::operations::delete::DeleteBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L101).

Source: `crates/core/src/operations/delete.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state

<a id="op-273fe340513831026bcd15e9"></a>
## writer_properties

`struct_field` · `deltalake_core::operations::delete::DeleteBuilder::writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_properties: Option<parquet::file::properties::WriterProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L108).

Source: `crates/core/src/operations/delete.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties passed to underlying parquet writer for when files are rewritten
