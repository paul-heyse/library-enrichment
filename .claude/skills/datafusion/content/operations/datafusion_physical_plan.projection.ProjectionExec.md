# `datafusion_physical_plan::projection::ProjectionExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.ProjectionExec.json).

<a id="op-b46d9dc006ec8aae1caad158"></a>
## ProjectionExec

`struct` · `datafusion_physical_plan::projection::ProjectionExec` · datafusion-physical-plan 55.1.0

```rust
struct ProjectionExec
```

Source: `src/projection.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) for a projection

Computes a set of scalar value expressions for each input row, producing one
output row for each input row.

<a id="op-9b7f38f53f49fb6ab3d5cf57"></a>
## apply_expressions

`function` · `datafusion_physical_plan::projection::ProjectionExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a9fb1f7c6043e5efddb0d96"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::projection::ProjectionExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-111ec920398e36bbd8a5afbc"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::projection::ProjectionExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4df979800af1f08ed042c93d"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::projection::ProjectionExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:446`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b02c1fd66a4c59208cf4b16"></a>
## children

`function` · `datafusion_physical_plan::projection::ProjectionExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-006ab238fe1a6ba8f010ad50"></a>
## clone

`function` · `datafusion_physical_plan::projection::ProjectionExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ProjectionExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 17], "end": [74, 22], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/projection.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea169a57aaf68dba4bd3c634"></a>
## execute

`function` · `datafusion_physical_plan::projection::ProjectionExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7333fe99daf12cd09cf55d6"></a>
## expr

`function` · `datafusion_physical_plan::projection::ProjectionExec::expr` · datafusion-physical-plan 55.1.0

```rust
fn expr(&self) -> &[ProjectionExpr]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [290, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The projection expressions stored as tuples of (expression, output column name)

<a id="op-3af762c8e90852b097db44f0"></a>
## fmt

`function` · `datafusion_physical_plan::projection::ProjectionExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 10], "end": [74, 15], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-870c8da134a83bdbe79bd067"></a>
## fmt_as

`function` · `datafusion_physical_plan::projection::ProjectionExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [331, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/projection.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b27ff01f94d32b7560674ed"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::projection::ProjectionExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62294dff45ba9fa00fc7bc7c"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::projection::ProjectionExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:512`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6596d512bd3241f337e3fbe1"></a>
## input

`function` · `datafusion_physical_plan::projection::ProjectionExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [290, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The input plan

<a id="op-346ddd79effc88e885cd451f"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::projection::ProjectionExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f5b4c6e8b843fe6b39d6d6d"></a>
## metrics

`function` · `datafusion_physical_plan::projection::ProjectionExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06ea6192d716123bc43895e4"></a>
## name

`function` · `datafusion_physical_plan::projection::ProjectionExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad2b45c2c1f35fff82275f2a"></a>
## projection_expr

`function` · `datafusion_physical_plan::projection::ProjectionExec::projection_expr` · datafusion-physical-plan 55.1.0

```rust
fn projection_expr(&self) -> &ProjectionExprs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [290, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The projection expressions as a [`ProjectionExprs`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-9af61cebb79ab505edeb3c19).

<a id="op-50f21baf88021c3faa8e1a97"></a>
## properties

`function` · `datafusion_physical_plan::projection::ProjectionExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-2d14b974b4ab7863f3b1801a"></a>
## replace_children

`function` · `datafusion_physical_plan::projection::ProjectionExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06a7d1dae39c9299aad41760"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::projection::ProjectionExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:450`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a65bc90bb9ffa311d79bde6f"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::projection::ProjectionExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca34c2672ca1c06c4f4df3a8"></a>
## try_from_proto

`function` · `datafusion_physical_plan::projection::ProjectionExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 1], "end": [663, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:634`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`ProjectionExec`](../operations/datafusion_physical_plan.projection.ProjectionExec.md#op-b46d9dc006ec8aae1caad158) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`]: it takes the whole
[`PhysicalPlanNode`] so every plan's `try_from_proto` shares one
signature. Child plans and expressions are decoded recursively via the
[`ExecutionPlanDecodeCtx`].

[`PhysicalPlanNode`]: datafusion_proto_models::protobuf::PhysicalPlanNode
[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto
[`ExecutionPlanDecodeCtx`]: crate::proto::ExecutionPlanDecodeCtx

<a id="op-b8feea682434076db4708a3a"></a>
## try_new

`function` · `datafusion_physical_plan::projection::ProjectionExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new<I, E>(expr: I, input: Arc<dyn ExecutionPlan>) -> Result<Self> where I: IntoIterator<Item = E>, E: Into<ProjectionExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [290, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a projection on an input

# Example:
Create a `ProjectionExec` to crate `SELECT a, a+b AS sum_ab FROM t1`:

```
# use std::sync::Arc;
# use arrow_schema::{Schema, Field, DataType};
# use datafusion_expr::Operator;
# use datafusion_physical_plan::ExecutionPlan;
# use datafusion_physical_expr::expressions::{col, binary};
# use datafusion_physical_plan::empty::EmptyExec;
# use datafusion_physical_plan::projection::{ProjectionExec, ProjectionExpr};
# fn schema() -> Arc<Schema> {
#  Arc::new(Schema::new(vec![
#   Field::new("a", DataType::Int32, false),
#   Field::new("b", DataType::Int32, false),
# ]))
# }
#
# fn input() -> Arc<dyn ExecutionPlan> {
#  Arc::new(EmptyExec::new(schema()))
# }
#
# fn main() {
let schema = schema();
// Create PhysicalExprs
let a = col("a", &schema).unwrap();
let b = col("b", &schema).unwrap();
let a_plus_b = binary(Arc::clone(&a), Operator::Plus, b, &schema).unwrap();
// create ProjectionExec
let proj = ProjectionExec::try_new(
    [
        ProjectionExpr {
            // expr a produces the column named "a"
            expr: a,
            alias: "a".to_string(),
        },
        ProjectionExpr {
            // expr: a + b produces the column named "sum_ab"
            expr: a_plus_b,
            alias: "sum_ab".to_string(),
        },
    ],
    input(),
)
.unwrap();
# }
```

<a id="op-353f611ca264a96ce22c340f"></a>
## try_new_with_schema_metadata

`function` · `datafusion_physical_plan::projection::ProjectionExec::try_new_with_schema_metadata` · datafusion-physical-plan 55.1.0

```rust
fn try_new_with_schema_metadata<I, E>(expr: I, input: Arc<dyn ExecutionPlan>, projected_schema: &Schema) -> Result<Self> where I: IntoIterator<Item = E>, E: Into<ProjectionExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [290, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a projection using field and schema metadata from
`projected_schema`.

Field names, data types, and nullability are still derived from the physical
projection expressions and the input plan; only field and schema metadata are
taken from `projected_schema`.

# Errors

Returns an error if the projection cannot be applied to the input plan, or if
`projected_schema` has a different number of fields than the projection.

<a id="op-d7b5eec72f85de77a9d88e8a"></a>
## try_pushdown_sort

`function` · `datafusion_physical_plan::projection::ProjectionExec::try_pushdown_sort` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afbbb21db9f1d1e99e586fd6"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::projection::ProjectionExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a2e197f1fa203392e2e1622"></a>
## try_to_proto

`function` · `datafusion_physical_plan::projection::ProjectionExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dcdfffdfa756049a2733e64"></a>
## with_new_children

`function` · `datafusion_physical_plan::projection::ProjectionExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcd02a26773a085eef8137d4"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::projection::ProjectionExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-317b449fac729ee4a67a608d"></a>
## with_preserve_order

`function` · `datafusion_physical_plan::projection::ProjectionExec::with_preserve_order` · datafusion-physical-plan 55.1.0

```rust
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::projection::ProjectionExec", "path": "ProjectionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [333, 1], "end": [620, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/projection.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
