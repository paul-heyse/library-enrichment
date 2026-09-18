# `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.scan.exec.DeltaScanExec.json).

<a id="op-d3a2d89c17a4d98bdb060c5d"></a>
## DeltaScanExec

`struct` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaScanExec
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L152).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:152`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Physical execution plan for scanning Delta tables.

Wraps a Parquet reader execution plan and applies Delta Lake protocol transformations
to produce the logical table data. This includes:

- **Column mapping**: Translates physical column names to logical names
- **Partition values**: Materializes partition column values from file paths
- **Deletion vectors**: Filters out deleted rows using per-file selection vectors
- **Schema evolution**: Handles missing columns and type coercion

# Data Flow

1. Inner [`input`](Self::input) plan reads raw Parquet data
2. Per-file [`transforms`](Self::transforms) convert physical to logical schema
3. The scan applies deletion vectors before it returns rows
4. Result is cast to the projected scan contract's result schema

Unresolved upstream links (retained, not inferred): `Self::transforms`, `Self::input`.

<a id="op-f23922696d79472c3f66d6fa"></a>
## apply_expressions

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::apply_expressions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn apply_expressions(&self, _expr_rewriter: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion, DataFusionError>) -> Result<TreeNodeRecursion, DataFusionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L694).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:694`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d864d66414fbabdcf74e0fe9"></a>
## cardinality_effect

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::cardinality_effect` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L591).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:591`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0252fae0f9f85859c6a5e03b"></a>
## child_stats_requests

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::child_stats_requests` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L615).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:615`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4d7072715b622631cffd25f"></a>
## children

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::children` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L460).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:460`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0d7384ddc4e7a473c97f206"></a>
## clone

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaScanExec
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L151).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 10], "end": [151, 15], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:151`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54b6a977458543df82f3b84d"></a>
## execute

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::execute` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L535).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:535`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7916a0f8c87bd4878163f785"></a>
## fetch

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::fetch` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fetch(&self) -> Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L599).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:599`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8d095dc94a8c869157ba32a"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L151).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 17], "end": [151, 22], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:151`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec45adbc78c4b44f05c6cca6"></a>
## fmt_as

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::fmt_as` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L175).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [192, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:175`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7854a0a6b1cd7247a39e809"></a>
## gather_filters_for_pushdown

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::gather_filters_for_pushdown` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L632).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:632`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ad1b6cf0e57b21877bc8c05"></a>
## input_distribution_requirements

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::input_distribution_requirements` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L468).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:468`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-461cd94f54c3c0172204bb42"></a>
## metrics

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::metrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metrics(&self) -> Option<MetricsSet>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L583).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:583`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-853dca487c2d500ab4fc07b0"></a>
## name

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> &'static str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L452).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:452`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32896e02fc15487db1d97179"></a>
## properties

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L456).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:456`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f681e62a7d5e4eff97c04011"></a>
## repartitioned

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::repartitioned` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn repartitioned(&self, target_partitions: usize, config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L515).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:515`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43cc8bda8e0cea35c0cd1585"></a>
## replace_children

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::replace_children` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L485).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:485`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c7b0e52e15f92852820414"></a>
## required_input_distribution

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::required_input_distribution` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L464).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:464`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc5f9e89a6ad950bcc65d8da"></a>
## statistics_from_inputs

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::statistics_from_inputs` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L621).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:621`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ac2993d3c2060de2526e3d3"></a>
## supports_limit_pushdown

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::supports_limit_pushdown` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn supports_limit_pushdown(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L587).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:587`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e970f53e4dc9efb98d86c8b7"></a>
## with_fetch

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::with_fetch` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L607).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:607`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ed69d99bb498a4af7bb324a"></a>
## with_new_children

`function` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::with_new_children` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L505).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec", "path": "DeltaScanExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [702, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:505`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-feb26dcf168b9248f277bd81"></a>
## dv_state

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::dv_state` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
dv_state: DvExecutionState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L159).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:159`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The planner records deletion vector masks and physical file ownership here.

<a id="op-1844d32b67e8925008a3d460"></a>
## file_id_column

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::file_id_column` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_id_column: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L167).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:167`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

User-visible file-id column name when projected in the output.

<a id="op-e0396996c22ac1b5dd75b88a"></a>
## input

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::input` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
input: std::sync::Arc<dyn ExecutionPlan>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L155).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execution plan yielding the raw data read from data files.

<a id="op-a2859295697c5ea30ea9ec02"></a>
## input_file_id_column

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::input_file_id_column` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
input_file_id_column: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L165).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:165`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File id column name carried by the input batches for per file correlation.

<a id="op-b78ba28dd70cb8481f90eaeb"></a>
## metrics

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::metrics` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metrics: datafusion::physical_plan::metrics::ExecutionPlanMetricsSet
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L163).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execution metrics

<a id="op-7bc942905960f34d503c4a42"></a>
## partition_stats

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::partition_stats` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_stats: datafusion::common::HashMap<String, datafusion::common::ColumnStatistics>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L171).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Aggregated partition column statistics

<a id="op-4e10bf3d2984f66035c8be1c"></a>
## properties

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
properties: std::sync::Arc<datafusion::physical_plan::execution_plan::PlanProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L169).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:169`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

plan properties

<a id="op-12f6cfa25de70f691edf2bc4"></a>
## public_file_ids

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::public_file_ids` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
public_file_ids: std::sync::Arc<datafusion::common::HashMap<String, String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L161).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:161`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Public file paths keyed by compact scan file id.

<a id="op-0091d2dfad5d1519357fab89"></a>
## scan_plan

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::scan_plan` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
scan_plan: std::sync::Arc<super::plan::KernelScanPlan>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L153).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:153`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a08bd9b30fcc4bc26dd2564"></a>
## transforms

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec::transforms` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
transforms: std::sync::Arc<datafusion::common::HashMap<String, delta_kernel::ExpressionRef>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L157).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:157`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Transforms to be applied to data eminating from individual files
