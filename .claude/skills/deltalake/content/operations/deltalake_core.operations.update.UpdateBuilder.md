# `deltalake_core::operations::update::UpdateBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.update.UpdateBuilder.json).

<a id="op-c3bb2cab22e07b61bb4f5c72"></a>
## UpdateBuilder

`struct` · `deltalake_core::operations::update::UpdateBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UpdateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L87).

Source: `crates/core/src/operations/update.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Updates records in the Delta Table.
See this module's documentation for more information

<a id="op-0892a8803e2c80281a98a642"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::update::UpdateBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <UpdateBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L462).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [557, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update.rs:462`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92fb88dd0cb8c29f60b9d8d9"></a>
## Output

`assoc_type` · `deltalake_core::operations::update::UpdateBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(DeltaTable, UpdateMetrics), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L461).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [557, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update.rs:461`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebd291283e2b495bd77be4e2"></a>
## into_future

`function` · `deltalake_core::operations::update::UpdateBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L464).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [557, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/update.rs:464`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bc794dc7cacc8ecb28ed9fd"></a>
## with_commit_properties

`function` · `deltalake_core::operations::update::UpdateBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L191).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [221, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update.rs:191`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-b9d5c2523de349a95791a8b1"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::update::UpdateBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L217).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [221, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update.rs:217`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-adca0d60d504d5c0698a731a"></a>
## with_predicate

`function` · `deltalake_core::operations::update::UpdateBuilder::with_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_predicate<E: Into<Expression>>(self, predicate: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L153).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [221, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update.rs:153`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Which records to update

<a id="op-f4e7be8ae8e8b03300a0730d"></a>
## with_safe_cast

`function` · `deltalake_core::operations::update::UpdateBuilder::with_safe_cast` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_safe_cast(self, safe_cast: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L211).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [221, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update.rs:211`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the cast options to use when casting columns that do not match
the table's schema.  When `cast_options.safe` is set true then any
failures to cast a datatype will use null instead of returning an error
to the user.

Example (column's type is int):
Input               Output
123         ->      123
Test123     ->      null

<a id="op-ebf48f5ea02e3a681ca18ccd"></a>
## with_session_fallback_policy

`function` · `deltalake_core::operations::update::UpdateBuilder::with_session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L185).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [221, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update.rs:185`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Control how delta-rs resolves the provided session when it is not a concrete `SessionState`.

Defaults to `SessionFallbackPolicy::InternalDefaults` to preserve existing behavior.

<a id="op-cbe8b3e5dd582530dfe8dc82"></a>
## with_session_state

`function` · `deltalake_core::operations::update::UpdateBuilder::with_session_state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L177).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [221, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update.rs:177`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the DataFusion session used for planning and execution.

The provided `session` should wrap a concrete `datafusion::execution::context::SessionState`.

If `session` is not a `SessionState`, the default policy is to log a warning and fall back to
internal defaults. To make this strict (error instead), set
`with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)`.

Example: `Arc::new(create_session().state())`.

<a id="op-4ce68114cef28f80c1ce94a7"></a>
## with_update

`function` · `deltalake_core::operations::update::UpdateBuilder::with_update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_update<S: Into<DeltaColumn>, E: Into<Expression>>(self, column: S, expression: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L159).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [221, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update.rs:159`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Perform an additional update expression during the operation

<a id="op-07a30be7daab7c13aaa90b93"></a>
## with_writer_properties

`function` · `deltalake_core::operations::update::UpdateBuilder::with_writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L197).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [221, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/update.rs:197`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Writer properties passed to parquet writer for when files are rewritten

<a id="op-6f8bc9f5456f4625603b48c2"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L102).

Source: `crates/core/src/operations/update.rs:102`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-a25c2711c3a44a76a3401301"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L106).

Source: `crates/core/src/operations/update.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7eb7b19ebd4b093b0d9b1447"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::update::UpdateBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L130).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [133, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/update.rs:130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54bf70fa19c92bb667287485"></a>
## log_store

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L95).

Source: `crates/core/src/operations/update.rs:95`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-9bd648943cb5cf0a48bdbed3"></a>
## log_store

`function` · `deltalake_core::operations::update::UpdateBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [133, 2], "filename": "crates/core/src/operations/update.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/update.rs:127`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2de8ca7ef62f793101a182f5"></a>
## predicate

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L89).

Source: `crates/core/src/operations/update.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Which records to update

<a id="op-1b0fb739bf644227c09f66ee"></a>
## safe_cast

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::safe_cast` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
safe_cast: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L105).

Source: `crates/core/src/operations/update.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

safe_cast determines how data types that do not match the underlying table are handled
By default an error is returned

<a id="op-eacf57bc1ebe44422af8a100"></a>
## session

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session: Option<std::sync::Arc<dyn Session>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L97).

Source: `crates/core/src/operations/update.rs:97`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion session state relevant for executing the input plan

<a id="op-c50b238b4ff5ce1d12681a30"></a>
## session_fallback_policy

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session_fallback_policy: delta_datafusion::SessionFallbackPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L98).

Source: `crates/core/src/operations/update.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d208b9ea5fdaef465df1b26e"></a>
## snapshot

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L93).

Source: `crates/core/src/operations/update.rs:93`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state

<a id="op-3b00c204a8818cb79f8caae8"></a>
## updates

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::updates` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
updates: std::collections::HashMap<datafusion::common::Column, delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L91).

Source: `crates/core/src/operations/update.rs:91`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

How to update columns in a record that match the predicate

<a id="op-8e1d10a3afd9e8eadd49a593"></a>
## writer_properties

`struct_field` · `deltalake_core::operations::update::UpdateBuilder::writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_properties: Option<parquet::file::properties::WriterProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L100).

Source: `crates/core/src/operations/update.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties passed to underlying parquet writer for when files are rewritten
