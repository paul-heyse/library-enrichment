# `deltalake_core::operations::optimize::MergePlan`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.MergePlan.json).

<a id="op-27c049e04d4d01e894959701"></a>
## MergePlan

`struct` · `deltalake_core::operations::optimize::MergePlan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MergePlan
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L558).

Source: `crates/core/src/operations/optimize.rs:558`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Encapsulates the operations required to optimize a Delta Table

<a id="op-2c7b80243caab2033a995bf2"></a>
## execute

`function` · `deltalake_core::operations::optimize::MergePlan::execute` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn execute(self, log_store: LogStoreRef, snapshot: &EagerSnapshot, max_concurrent_tasks: usize, min_commit_interval: Option<Duration>, commit_properties: CommitProperties, operation_id: Uuid, handle: Option<&Arc<dyn CustomExecuteHandler>>) -> Result<Metrics, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L820).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MergePlan", "path": "MergePlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [660, 1], "end": [1015, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:820`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Perform the operations outlined in the plan.

<a id="op-b5766fe0ef678a213e30ca2f"></a>
## fmt

`function` · `deltalake_core::operations::optimize::MergePlan::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L556).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MergePlan", "path": "MergePlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [556, 10], "end": [556, 15], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/optimize.rs:556`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-333064296e8d3eb11af998f8"></a>
## metrics

`struct_field` · `deltalake_core::operations::optimize::MergePlan::metrics` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metrics: Metrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L561).

Source: `crates/core/src/operations/optimize.rs:561`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics collected during operation

<a id="op-b8fea59331b1a4f6fd20264f"></a>
## operations

`struct_field` · `deltalake_core::operations::optimize::MergePlan::operations` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operations: OptimizeOperations
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L559).

Source: `crates/core/src/operations/optimize.rs:559`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-932c16a8fa7b88121c3ea689"></a>
## planner_stats

`struct_field` · `deltalake_core::operations::optimize::MergePlan::planner_stats` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
planner_stats: PlannerStats
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L563).

Source: `crates/core/src/operations/optimize.rs:563`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Planner metadata copied into buffered and total metrics

<a id="op-a9136f48849731814bf780fa"></a>
## read_session

`struct_field` · `deltalake_core::operations::optimize::MergePlan::read_session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
read_session: std::sync::Arc<datafusion::execution::context::SessionState>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L569).

Source: `crates/core/src/operations/optimize.rs:569`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Session state used for provider owned rewrite scans.

<a id="op-84cbd4ee29d61b00b10acaf3"></a>
## read_table_version

`struct_field` · `deltalake_core::operations::optimize::MergePlan::read_table_version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
read_table_version: kernel::Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L567).

Source: `crates/core/src/operations/optimize.rs:567`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version of the table at beginning of optimization. Used for conflict resolution.

<a id="op-9eaff2e7c6557c0c1147180c"></a>
## task_parameters

`struct_field` · `deltalake_core::operations::optimize::MergePlan::task_parameters` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
task_parameters: std::sync::Arc<MergeTaskParameters>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L565).

Source: `crates/core/src/operations/optimize.rs:565`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parameters passed down to merge tasks
