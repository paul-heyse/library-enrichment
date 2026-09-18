# `datafusion_datasource::source::DataSourceExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.source.DataSourceExec.json).

<a id="op-96b0a6eef9f3c044c580d9b6"></a>
## DataSourceExec

`struct` · `datafusion_datasource::source::DataSourceExec` · datafusion-datasource 55.1.0

```rust
struct DataSourceExec
```

Source: `src/source.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) that reads one or more files

`DataSourceExec` implements common functionality such as applying
projections, and caching plan properties.

The [`DataSource`](../operations/datafusion_datasource.source.DataSource.md#op-9da841d166501739a52bf7ac) describes where to find the data for this data source
(for example in files or what in memory partitions).

For file based [`DataSource`](../operations/datafusion_datasource.source.DataSource.md#op-9da841d166501739a52bf7ac)s, format specific behavior is implemented in
the [`FileSource`] trait.

[`FileSource`]: crate::file::FileSource

<a id="op-68e91173a4af239bec861915"></a>
## apply_expressions

`function` · `datafusion_datasource::source::DataSourceExec::apply_expressions` · datafusion-datasource 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f957474668955617ed8b7fe"></a>
## children

`function` · `datafusion_datasource::source::DataSourceExec::children` · datafusion-datasource 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c5d949681c7717356b5d590"></a>
## clone

`function` · `datafusion_datasource::source::DataSourceExec::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> DataSourceExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [365, 10], "end": [365, 15], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/source.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-846ea0ac0bb8ec597538fcb7"></a>
## data_source

`function` · `datafusion_datasource::source::DataSourceExec::data_source` · datafusion-datasource 55.1.0

```rust
fn data_source(&self) -> &Arc<dyn DataSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [697, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:647`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the source object

<a id="op-384adf81b0c14b7e888efdb5"></a>
## downcast_to_file_source

`function` · `datafusion_datasource::source::DataSourceExec::downcast_to_file_source` · datafusion-datasource 55.1.0

```rust
fn downcast_to_file_source<T: FileSource>(&self) -> Option<(&FileScanConfig, &T)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [697, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:685`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Downcast the `DataSourceExec`'s `data_source` to a specific file source

Returns `None` if
1. the datasource is not scanning files (`FileScanConfig`)
2. The [`FileScanConfig::file_source`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-47b12dd2c48dbed38d686054) is not of type `T`

<a id="op-c5c5c081750bcd0a51df5549"></a>
## execute

`function` · `datafusion_datasource::source::DataSourceExec::execute` · datafusion-datasource 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a1af776df4f5cf47dcbf3b2"></a>
## fetch

`function` · `datafusion_datasource::source::DataSourceExec::fetch` · datafusion-datasource 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1c11497b838d1386a92b449"></a>
## fmt

`function` · `datafusion_datasource::source::DataSourceExec::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [365, 17], "end": [365, 22], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/source.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49a7a932703c96ec6c81868e"></a>
## fmt_as

`function` · `datafusion_datasource::source::DataSourceExec::fmt_as` · datafusion-datasource 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [389, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/source.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba6dd5a7d0b14f2522cab762"></a>
## from

`function` · `datafusion_datasource::source::DataSourceExec::from` · datafusion-datasource 55.1.0

```rust
fn from(source: S) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [700, 1], "end": [707, 2], "filename": "src/source.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/source.rs:704`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c69c4a65fbb8ffedb36f4a3"></a>
## from_data_source

`function` · `datafusion_datasource::source::DataSourceExec::from_data_source` · datafusion-datasource 55.1.0

```rust
fn from_data_source(data_source: impl DataSource + 'static) -> Arc<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [697, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:632`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2599ad41766d935510260b7"></a>
## handle_child_pushdown_result

`function` · `datafusion_datasource::source::DataSourceExec::handle_child_pushdown_result` · datafusion-datasource 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:540`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cf66f52daf7e3e24fec15c7"></a>
## metrics

`function` · `datafusion_datasource::source::DataSourceExec::metrics` · datafusion-datasource 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f74d6de9eb0aa686d813d53e"></a>
## name

`function` · `datafusion_datasource::source::DataSourceExec::name` · datafusion-datasource 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:392`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82cb6233007e590caf0ceaa1"></a>
## new

`function` · `datafusion_datasource::source::DataSourceExec::new` · datafusion-datasource 55.1.0

```rust
fn new(data_source: Arc<dyn DataSource>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [697, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:637`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e037d05c4615d8ae991c807"></a>
## properties

`function` · `datafusion_datasource::source::DataSourceExec::properties` · datafusion-datasource 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95100c34164b5be82cff58c0"></a>
## repartitioned

`function` · `datafusion_datasource::source::DataSourceExec::repartitioned` · datafusion-datasource 55.1.0

```rust
fn repartitioned(&self, target_partitions: usize, config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Implementation of [`ExecutionPlan::repartitioned`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-584645e19b0573a60b990c36) which relies upon the inner [`DataSource::repartitioned`](../operations/datafusion_datasource.source.DataSource.md#op-2c6fe409f79db273197b6979).

If the data source does not support changing its partitioning, returns `Ok(None)` (the default). Refer
to [`ExecutionPlan::repartitioned`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-584645e19b0573a60b990c36) for more details.

<a id="op-47f59b1bf32524b34a19a303"></a>
## replace_children

`function` · `datafusion_datasource::source::DataSourceExec::replace_children` · datafusion-datasource 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17e552e77caa558b7b92757f"></a>
## reset_state

`function` · `datafusion_datasource::source::DataSourceExec::reset_state` · datafusion-datasource 55.1.0

```rust
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:612`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec2115456d8ac91ca5b20c8f"></a>
## statistics_from_inputs

`function` · `datafusion_datasource::source::DataSourceExec::statistics_from_inputs` · datafusion-datasource 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85461231c8efa1e5736a1f1b"></a>
## try_pushdown_sort

`function` · `datafusion_datasource::source::DataSourceExec::try_pushdown_sort` · datafusion-datasource 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:575`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c29084f4598a4757af7ca637"></a>
## try_swapping_with_projection

`function` · `datafusion_datasource::source::DataSourceExec::try_swapping_with_projection` · datafusion-datasource 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e5d4573108db506ae6e00dc"></a>
## try_to_proto

`function` · `datafusion_datasource::source::DataSourceExec::try_to_proto` · datafusion-datasource 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:623`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Delegates serialization to the wrapped [`DataSource`](../operations/datafusion_datasource.source.DataSource.md#op-9da841d166501739a52bf7ac). For file scans the
concrete [`FileSource`](../operations/datafusion_datasource.file.FileSource.md#op-8c1b19f80ea0c73466216a63) emits the node via its
own `try_to_proto` hook, keeping the format-specific wire logic in the
format crate.

<a id="op-99ca5df6c2b8db887ffb48ef"></a>
## with_constraints

`function` · `datafusion_datasource::source::DataSourceExec::with_constraints` · datafusion-datasource 55.1.0

```rust
fn with_constraints(self, constraints: Constraints) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [697, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:659`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Assign constraints

<a id="op-9cf2b4515b5dd1059c78a31f"></a>
## with_data_source

`function` · `datafusion_datasource::source::DataSourceExec::with_data_source` · datafusion-datasource 55.1.0

```rust
fn with_data_source(self, data_source: Arc<dyn DataSource>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [697, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:651`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a727d86877636329f64dc020"></a>
## with_fetch

`function` · `datafusion_datasource::source::DataSourceExec::with_fetch` · datafusion-datasource 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:509`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f515a9eeb63f7bae87f364e8"></a>
## with_new_children

`function` · `datafusion_datasource::source::DataSourceExec::with_new_children` · datafusion-datasource 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c74697b584ec905fde6f5afe"></a>
## with_new_state

`function` · `datafusion_datasource::source::DataSourceExec::with_new_state` · datafusion-datasource 55.1.0

```rust
fn with_new_state(&self, state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdfca8db76f15c37d8006954"></a>
## with_partitioning

`function` · `datafusion_datasource::source::DataSourceExec::with_partitioning` · datafusion-datasource 55.1.0

```rust
fn with_partitioning(self, partitioning: Partitioning) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [697, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:665`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Assign output partitioning

<a id="op-93d456d0bc8ce238169a6fb8"></a>
## with_preserve_order

`function` · `datafusion_datasource::source::DataSourceExec::with_preserve_order` · datafusion-datasource 55.1.0

```rust
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::DataSourceExec", "path": "DataSourceExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [629, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/source.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
