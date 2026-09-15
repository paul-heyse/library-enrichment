# `datafusion_physical_optimizer::output_requirements`

Crate `datafusion-physical-optimizer` · 2 public items · structured records in [`model/datafusion_physical_optimizer.output_requirements.json`](../model/datafusion_physical_optimizer.output_requirements.json)

## OutputRequirementExec

`struct` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec`

```rust
struct OutputRequirementExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Debug

**Methods** (3)

```rust
fn fetch(&self) -> Option<usize>
fn input(&self) -> Arc<dyn ExecutionPlan>
fn new(input: Arc<dyn ExecutionPlan>, requirements: Option<OrderingRequirements>, dist_requirement: Distribution, fetch: Option<usize>) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn datafusion_physical_expr_common::physical_expr::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, _partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn input_distribution_requirements(&self) -> datafusion_physical_plan::InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

An ancillary, non-executable operator whose sole purpose is to track global
requirements during optimization. It imposes
- the ordering requirement in its `order_requirement` attribute.
- the distribution requirement in its `dist_requirement` attribute.

See [`OutputRequirements`] for more details

---

## OutputRequirements

`struct` · `datafusion_physical_optimizer::output_requirements::OutputRequirements`

```rust
struct OutputRequirements
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug

**Methods** (2)

```rust
fn new_add_mode() -> Self
fn new_remove_mode() -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

This rule either adds or removes [`OutputRequirements`]s to/from the physical
plan according to its `mode` attribute, which is set by the constructors
`new_add_mode` and `new_remove_mode`. With this rule, we can keep track of
the global requirements (ordering and distribution) across rules.

The primary use case of this node and rule is to specify and preserve the desired output
ordering and distribution the entire plan. When sending to a single client, a single partition may
be desirable, but when sending to a multi-partitioned writer, keeping multiple partitions may be
better.

---
