# `deltalake_core::operations::constraints::ConstraintBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.constraints.ConstraintBuilder.json).

<a id="op-619503865be1d88eb89445da"></a>
## ConstraintBuilder

`struct` · `deltalake_core::operations::constraints::ConstraintBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ConstraintBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L28).

Source: `crates/core/src/operations/constraints.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build a constraint to add to a table

<a id="op-7c5b4b4ecf19767ab8df2a0e"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::constraints::ConstraintBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <ConstraintBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L110).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [266, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/constraints.rs:110`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19fc6dbb2244e5637abd5844"></a>
## Output

`assoc_type` · `deltalake_core::operations::constraints::ConstraintBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L108).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [266, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/constraints.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d375f501caf32be21a4ca5d2"></a>
## into_future

`function` · `deltalake_core::operations::constraints::ConstraintBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L112).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [266, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/constraints.rs:112`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f02d227a66adef5698ca2db5"></a>
## with_commit_properties

`function` · `deltalake_core::operations::constraints::ConstraintBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L95).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [105, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/constraints.rs:95`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-1f1175544e0a01be51111970"></a>
## with_constraint

`function` · `deltalake_core::operations::constraints::ConstraintBuilder::with_constraint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_constraint<S: Into<String>, E: Into<Expression>>(self, name: S, expression: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L65).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [105, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/constraints.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the constraint to be added

<a id="op-a6cbb4e578f76479aa1eb2eb"></a>
## with_constraints

`function` · `deltalake_core::operations::constraints::ConstraintBuilder::with_constraints` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_constraints<S: Into<String>, E: Into<Expression>>(self, constraints: HashMap<S, E>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [105, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/constraints.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify multiple constraints to be added

<a id="op-fe5faf8e513a7509c9212398"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::constraints::ConstraintBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [105, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/constraints.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-6aaea100e8011b51c6b61896"></a>
## with_session_state

`function` · `deltalake_core::operations::constraints::ConstraintBuilder::with_session_state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L89).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [105, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/constraints.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The Datafusion session state to use

<a id="op-31889dbfdf8dfcebf04da054"></a>
## check_constraints

`struct_field` · `deltalake_core::operations::constraints::ConstraintBuilder::check_constraints` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
check_constraints: std::collections::HashMap<String, delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L32).

Source: `crates/core/src/operations/constraints.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Hashmap containing an name of the constraint and expression

<a id="op-bbe357cd9342ff7efa6bfed4"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::constraints::ConstraintBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L38).

Source: `crates/core/src/operations/constraints.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-4a22c9161ceb7a2935f0eebb"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::constraints::ConstraintBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L39).

Source: `crates/core/src/operations/constraints.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64a5d1678cbe9c1cc26ce46b"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::constraints::ConstraintBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [49, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/constraints.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2839999d0ae3fc50fef30a4e"></a>
## log_store

`function` · `deltalake_core::operations::constraints::ConstraintBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L43).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::constraints::ConstraintBuilder", "path": "ConstraintBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [49, 2], "filename": "crates/core/src/operations/constraints.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/constraints.rs:43`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-722bf1e8e65272254de88050"></a>
## log_store

`struct_field` · `deltalake_core::operations::constraints::ConstraintBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L34).

Source: `crates/core/src/operations/constraints.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-90cc60037bce66a87b03f0dc"></a>
## session

`struct_field` · `deltalake_core::operations::constraints::ConstraintBuilder::session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session: Option<std::sync::Arc<dyn Session>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L36).

Source: `crates/core/src/operations/constraints.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion session state relevant for executing the input plan

<a id="op-e3c2a319b5bdcbc5524af8e9"></a>
## snapshot

`struct_field` · `deltalake_core::operations::constraints::ConstraintBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L30).

Source: `crates/core/src/operations/constraints.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state
