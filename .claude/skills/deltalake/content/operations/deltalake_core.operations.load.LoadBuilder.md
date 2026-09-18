# `deltalake_core::operations::load::LoadBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.load.LoadBuilder.json).

<a id="op-55c2e757d2535f4cf337d261"></a>
## LoadBuilder

`struct` · `deltalake_core::operations::load::LoadBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
struct LoadBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L18).

Source: `crates/core/src/operations/load.rs:18`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-029befe6ffafafe89f5dc666"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::load::LoadBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <LoadBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L73).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [115, 2], "filename": "crates/core/src/operations/load.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/load.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10d61fc5ea060fbd1511386d"></a>
## Output

`assoc_type` · `deltalake_core::operations::load::LoadBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(DeltaTable, Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [115, 2], "filename": "crates/core/src/operations/load.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/load.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84f44d0b7bf49401e3573073"></a>
## clone

`function` · `deltalake_core::operations::load::LoadBuilder::clone` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LoadBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L17).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [17, 10], "end": [17, 15], "filename": "crates/core/src/operations/load.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/load.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de5eb5b7fef5fb1e76cfb8fa"></a>
## columns

`struct_field` · `deltalake_core::operations::load::LoadBuilder::columns` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
columns: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L24).

Source: `crates/core/src/operations/load.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A sub-selection of columns to be loaded

<a id="op-7bd7f1cadf9b1711ecb750c7"></a>
## fmt

`function` · `deltalake_core::operations::load::LoadBuilder::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L30).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [36, 2], "filename": "crates/core/src/operations/load.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/load.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70d264c6762ebf21dcd723f1"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::load::LoadBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L42).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [45, 2], "filename": "crates/core/src/operations/load.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/load.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65d1838b15714e8694ffa3a1"></a>
## into_future

`function` · `deltalake_core::operations::load::LoadBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L75).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [115, 2], "filename": "crates/core/src/operations/load.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/load.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-294085ce4c15f9411d72b0ad"></a>
## log_store

`function` · `deltalake_core::operations::load::LoadBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L39).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [45, 2], "filename": "crates/core/src/operations/load.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/load.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7710d41020e64ef83797ee2"></a>
## log_store

`struct_field` · `deltalake_core::operations::load::LoadBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L22).

Source: `crates/core/src/operations/load.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-faa04c488687b99436347b5e"></a>
## session

`struct_field` · `deltalake_core::operations::load::LoadBuilder::session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session: Option<std::sync::Arc<dyn Session>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L26).

Source: `crates/core/src/operations/load.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion session state relevant for executing the input plan

<a id="op-d09ff3297365f10d088cd6d9"></a>
## snapshot

`struct_field` · `deltalake_core::operations::load::LoadBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L20).

Source: `crates/core/src/operations/load.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the to-be-loaded table's state

<a id="op-3cbadb75441606b267067afa"></a>
## with_columns

`function` · `deltalake_core::operations::load::LoadBuilder::with_columns` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn with_columns(self, columns: impl IntoIterator<Item = impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [69, 2], "filename": "crates/core/src/operations/load.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

**Reference annotation (source_runtime_reconciliation, separate from upstream):** The partition-first schema fixture selects the wrong field through this positional projection path. Provider/DataFrame name-based projection is the tested alternative. [Evidence](../capabilities/delta.read.md).

Specify column selection to load

<a id="op-6995ded20806702f623b60a2"></a>
## with_session_state

`function` · `deltalake_core::operations::load::LoadBuilder::with_session_state` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load.rs#L65).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load::LoadBuilder", "path": "LoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [69, 2], "filename": "crates/core/src/operations/load.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The Datafusion session state to use
