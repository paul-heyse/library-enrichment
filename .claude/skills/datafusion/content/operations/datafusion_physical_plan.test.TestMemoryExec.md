# `datafusion_physical_plan::test::TestMemoryExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.TestMemoryExec.json).

<a id="op-491d967989c73022f6134d69"></a>
## TestMemoryExec

`struct` · `datafusion_physical_plan::test::TestMemoryExec` · datafusion-physical-plan 55.1.0

```rust
struct TestMemoryExec
```

Source: `src/test.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

`TestMemoryExec` is a mock equivalent to [`MemorySourceConfig`] with [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) implemented for testing.
i.e. It has some but not all the functionality of [`MemorySourceConfig`].
This implements an in-memory DataSource rather than explicitly implementing a trait.
It is implemented in this manner to keep relevant unit tests in place
while avoiding circular dependencies between `datafusion-physical-plan` and `datafusion-datasource`.

[`MemorySourceConfig`]: https://github.com/apache/datafusion/tree/main/datafusion/datasource/src/memory.rs

<a id="op-6b96cb416ba1127d6b75c2aa"></a>
## apply_expressions

`function` · `datafusion_physical_plan::test::TestMemoryExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85e4ff5a8cfab154e667c3fb"></a>
## children

`function` · `datafusion_physical_plan::test::TestMemoryExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-baf484ede1a33027a7d2b8b2"></a>
## clone

`function` · `datafusion_physical_plan::test::TestMemoryExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> TestMemoryExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "src/test.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/test.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d85420b29d6fb8a3d05e67e6"></a>
## execute

`function` · `datafusion_physical_plan::test::TestMemoryExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a26b40f34408ae7a3301cb9"></a>
## fetch

`function` · `datafusion_physical_plan::test::TestMemoryExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ee8e68e9598953b0cf34ba9"></a>
## fmt

`function` · `datafusion_physical_plan::test::TestMemoryExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 17], "end": [64, 22], "filename": "src/test.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b5ff7618b2b2ff1d6a50622"></a>
## fmt_as

`function` · `datafusion_physical_plan::test::TestMemoryExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [129, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/test.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-634ef07ce3d2268c8e1190c9"></a>
## metrics

`function` · `datafusion_physical_plan::test::TestMemoryExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6110ae67f53f3215e2f14f4e"></a>
## name

`function` · `datafusion_physical_plan::test::TestMemoryExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09721291d61da621ab1803ef"></a>
## original_schema

`function` · `datafusion_physical_plan::test::TestMemoryExec::original_schema` · datafusion-physical-plan 55.1.0

```rust
fn original_schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Arc clone of ref to original schema

<a id="op-e4cacb49c2b38d29ffb4183f"></a>
## partitions

`function` · `datafusion_physical_plan::test::TestMemoryExec::partitions` · datafusion-physical-plan 55.1.0

```rust
fn partitions(&self) -> &[Vec<RecordBatch>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to partitions

<a id="op-2cb279cdd7435e7db4b2a127"></a>
## projection

`function` · `datafusion_physical_plan::test::TestMemoryExec::projection` · datafusion-physical-plan 55.1.0

```rust
fn projection(&self) -> &Option<Vec<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to projection

<a id="op-34bbf6e7fa1cbe63a6ceabf7"></a>
## properties

`function` · `datafusion_physical_plan::test::TestMemoryExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73f6651667fae71a9cd3a8d9"></a>
## repartitioned

`function` · `datafusion_physical_plan::test::TestMemoryExec::repartitioned` · datafusion-physical-plan 55.1.0

```rust
fn repartitioned(&self, _target_partitions: usize, _config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f5e99bd3ac1074638c6ad1c"></a>
## replace_children

`function` · `datafusion_physical_plan::test::TestMemoryExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c64c9ddb171414b8f87b4f36"></a>
## sort_information

`function` · `datafusion_physical_plan::test::TestMemoryExec::sort_information` · datafusion-physical-plan 55.1.0

```rust
fn sort_information(&self) -> &[LexOrdering]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to sort information

<a id="op-94c6d19eb52a4dad34b5c670"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::test::TestMemoryExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d3e76ba79e097c33a53acc1"></a>
## try_new

`function` · `datafusion_physical_plan::test::TestMemoryExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b87026afe3f9bf18bbe8574"></a>
## try_new_exec

`function` · `datafusion_physical_plan::test::TestMemoryExec::try_new_exec` · datafusion-physical-plan 55.1.0

```rust
fn try_new_exec(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Arc<TestMemoryExec>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new `DataSourceExec` Equivalent plan for reading in-memory record batches
The provided `schema` should not have the projection applied.

<a id="op-b7fc82920b106ddc841177dd"></a>
## try_with_sort_information

`function` · `datafusion_physical_plan::test::TestMemoryExec::try_with_sort_information` · datafusion-physical-plan 55.1.0

```rust
fn try_with_sort_information(self, sort_information: Vec<LexOrdering>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

refer to `try_with_sort_information` at MemorySourceConfig for more information.
<https://github.com/apache/datafusion/tree/main/datafusion/datasource/src/memory.rs>

<a id="op-10b49027cec407980cfdc37a"></a>
## update_cache

`function` · `datafusion_physical_plan::test::TestMemoryExec::update_cache` · datafusion-physical-plan 55.1.0

```rust
fn update_cache(source: &Arc<TestMemoryExec>) -> TestMemoryExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2adb3bd2ad6d1c84527db828"></a>
## with_limit

`function` · `datafusion_physical_plan::test::TestMemoryExec::with_limit` · datafusion-physical-plan 55.1.0

```rust
fn with_limit(self, limit: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [370, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the limit of the files

<a id="op-0f382e11556095a2c000e1e5"></a>
## with_new_children

`function` · `datafusion_physical_plan::test::TestMemoryExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestMemoryExec", "path": "TestMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
