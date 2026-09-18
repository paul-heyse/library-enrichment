# `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.scalar_subquery.ScalarSubqueryExec.json).

<a id="op-a478f6274c9e53b0018ce559"></a>
## ScalarSubqueryExec

`struct` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec` · datafusion-physical-plan 55.1.0

```rust
struct ScalarSubqueryExec
```

Source: `src/scalar_subquery.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Manages execution of uncorrelated scalar subqueries for a single plan
level.

From a query-results perspective, this node is a pass-through: it yields
the same batches as its main input and exists only to populate scalar
subquery results as a side effect before those batches are produced.

The first child node is the **main input plan**, whose batches are passed
through unchanged. The remaining children are **subquery plans**, each of
which must produce exactly zero or one row. Before any batches from the main
input are yielded, all subquery plans are executed and their scalar results
are stored in a shared [`ScalarSubqueryResults`](../operations/datafusion_expr.physical_planning_context.ScalarSubqueryResults.md#op-6b0683e0d87a9cc813cb37e5) container owned by this
node. [`ScalarSubqueryExpr`] nodes embedded in the main input's expressions
hold the same container and read from it by index.

All subqueries are evaluated eagerly when the first output partition is
requested, before any rows from the main input are produced.

TODO: Consider overlapping computation of the subqueries with evaluating the
main query.

[`ScalarSubqueryExpr`]: datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr

<a id="op-582a9f988c05a46f7f1f4242"></a>
## apply_expressions

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-905a53099df7b28dc5aa9ce3"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c4f12042918395aa5c3e58f"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23e9b47baab78fb85562eae4"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32f1a3b1eae7e3441b78b644"></a>
## children

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c4f312a0b94de569b415ba2"></a>
## execute

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af3cc4ea51fc7ca9b77487f8"></a>
## fmt

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 15], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/scalar_subquery.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31121aac8b396c2c2808d63e"></a>
## fmt_as

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [151, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/scalar_subquery.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-555f92fa8b7c62f3fa64aee9"></a>
## input

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [134, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc24358837f3ca2a8a4a9ca"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87144731affcd6031ec6c9f5"></a>
## name

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f70567e02b5f8647077be09"></a>
## new

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>, subqueries: Vec<ScalarSubqueryLink>, results: ScalarSubqueryResults) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [134, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e33e92146457c9c7fff93b0"></a>
## properties

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4106a20ad9e625eee4b3bb2"></a>
## replace_children

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-feeac263930197299ee03ed3"></a>
## reset_state

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7872baec7ba7a0605039508"></a>
## results

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::results` · datafusion-physical-plan 55.1.0

```rust
fn results(&self) -> &ScalarSubqueryResults
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [134, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cbdc514541173b4329a4a57"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98bad41333ec87420892c3d0"></a>
## subqueries

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::subqueries` · datafusion-physical-plan 55.1.0

```rust
fn subqueries(&self) -> &[ScalarSubqueryLink]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [134, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eead29fee03fce1b752f2905"></a>
## try_from_proto

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 1], "end": [342, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`ScalarSubqueryExec`](../operations/datafusion_physical_plan.scalar_subquery.ScalarSubqueryExec.md#op-a478f6274c9e53b0018ce559) from its protobuf representation.

<a id="op-a02c89548f296ca12a9e61dc"></a>
## try_to_proto

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:282`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5381c1b9ea9f2f644d67a491"></a>
## with_new_children

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec", "path": "ScalarSubqueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [303, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/scalar_subquery.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
