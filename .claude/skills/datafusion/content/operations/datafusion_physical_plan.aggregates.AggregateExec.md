# `datafusion_physical_plan::aggregates::AggregateExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.AggregateExec.json).

<a id="op-6938ad75ef3f2f21fcb0d3c4"></a>
## AggregateExec

`struct` · `datafusion_physical_plan::aggregates::AggregateExec` · datafusion-physical-plan 55.1.0

```rust
struct AggregateExec
```

Source: `src/aggregates/mod.rs:832`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Hash aggregate execution plan

<a id="op-91f6c29f7ba2d4e4f91cefde"></a>
## aggr_expr

`function` · `datafusion_physical_plan::aggregates::AggregateExec::aggr_expr` · datafusion-physical-plan 55.1.0

```rust
fn aggr_expr(&self) -> &[Arc<AggregateFunctionExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1084`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Aggregate expressions

<a id="op-ae95f33517f27db19c8c0e2e"></a>
## apply_expressions

`function` · `datafusion_physical_plan::aggregates::AggregateExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2074`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd86dcdf070ce1fa2085ac50"></a>
## cache

`function` · `datafusion_physical_plan::aggregates::AggregateExec::cache` · datafusion-physical-plan 55.1.0

```rust
fn cache(&self) -> &PlanProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:918`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae7e228ec9af7c48b52aa53b"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::aggregates::AggregateExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2149`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91bf76d63e5265808d8f437e"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::aggregates::AggregateExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2134`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc6944ec68d74a72f1fa46e9"></a>
## children

`function` · `datafusion_physical_plan::aggregates::AggregateExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2031`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b71e683f4481ff45d8f301a"></a>
## clone

`function` · `datafusion_physical_plan::aggregates::AggregateExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> AggregateExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 17], "end": [831, 22], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregates/mod.rs:831`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed5bc679dbfac5e36452cdb4"></a>
## compute_properties

`function` · `datafusion_physical_plan::aggregates::AggregateExec::compute_properties` · datafusion-physical-plan 55.1.0

```rust
fn compute_properties(input: &Arc<dyn ExecutionPlan>, schema: SchemaRef, group_expr_mapping: &ProjectionMapping, is_true_no_grouping: bool, mode: &AggregateMode, input_order_mode: &InputOrderMode, aggr_exprs: &[Arc<AggregateFunctionExpr>]) -> Result<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1346`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

This function creates the cache object that stores the plan properties such as schema, equivalence properties, ordering, partitioning, etc.

<a id="op-b38bffe3a7c574be52ec4853"></a>
## dynamic_expressions_produced

`function` · `datafusion_physical_plan::aggregates::AggregateExec::dynamic_expressions_produced` · datafusion-physical-plan 55.1.0

```rust
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4e4ebe2e3f3fb56ec0f0e75"></a>
## dynamic_filter_expr

`function` · `datafusion_physical_plan::aggregates::AggregateExec::dynamic_filter_expr` · datafusion-physical-plan 55.1.0

```rust
fn dynamic_filter_expr(&self) -> Option<&Arc<DynamicFilterPhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1098`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the dynamic filter expression for this aggregate, if set.

<a id="op-c00b14e4e34c4c08703ed256"></a>
## execute

`function` · `datafusion_physical_plan::aggregates::AggregateExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-461f77bb10dcd12276386168"></a>
## filter_expr

`function` · `datafusion_physical_plan::aggregates::AggregateExec::filter_expr` · datafusion-physical-plan 55.1.0

```rust
fn filter_expr(&self) -> &[Option<Arc<dyn PhysicalExpr>>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1089`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

FILTER (WHERE clause) expression for each aggregate expression

<a id="op-508ea9e442f09e61cda89df4"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::AggregateExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [831, 10], "end": [831, 15], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/mod.rs:831`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35bd639595e33280d5b94772"></a>
## fmt_as

`function` · `datafusion_physical_plan::aggregates::AggregateExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1837, 1], "end": [1961, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/aggregates/mod.rs:1838`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe308e8fcc8d1b073b3df991"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::aggregates::AggregateExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2155`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Push down parent filters when possible (see implementation comment for details),
and also pushdown self dynamic filters (see `AggrDynFilter` for details)

<a id="op-136c384c4a2cd8121c93b4e3"></a>
## get_minmax_desc

`function` · `datafusion_physical_plan::aggregates::AggregateExec::get_minmax_desc` · datafusion-physical-plan 55.1.0

```rust
fn get_minmax_desc(&self) -> Option<(FieldRef, bool)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1300`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Finds the DataType and SortDirection for this Aggregate, if there is one

<a id="op-3bcc0afaf3cd22cbfbbd0578"></a>
## group_expr

`function` · `datafusion_physical_plan::aggregates::AggregateExec::group_expr` · datafusion-physical-plan 55.1.0

```rust
fn group_expr(&self) -> &PhysicalGroupBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1074`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Grouping expressions

<a id="op-a21faa5f442e054ab8c69232"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::aggregates::AggregateExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2213`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If child accepts self's dynamic filter, keep `self.dynamic_filter` with Some,
otherwise clear it to None.

<a id="op-b22fe4adea47f1054492dfbf"></a>
## input

`function` · `datafusion_physical_plan::aggregates::AggregateExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1139`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input plan

<a id="op-cceb5896f6ef968cfbf8ab6d"></a>
## input

`struct_field` · `datafusion_physical_plan::aggregates::AggregateExec::input` · datafusion-physical-plan 55.1.0

```rust
input: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/aggregates/mod.rs:847`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input plan, could be a partial aggregate or the input to the aggregate

<a id="op-054160345a33f7c16f4ecf6e"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::aggregates::AggregateExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2000`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b072586d7f1f32d2b6e4383"></a>
## input_order_mode

`function` · `datafusion_physical_plan::aggregates::AggregateExec::input_order_mode` · datafusion-physical-plan 55.1.0

```rust
fn input_order_mode(&self) -> &InputOrderMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1418`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08b1e64114e5692aeb342fb0"></a>
## input_schema

`function` · `datafusion_physical_plan::aggregates::AggregateExec::input_schema` · datafusion-physical-plan 55.1.0

```rust
fn input_schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1144`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the input schema before any aggregates are applied

<a id="op-56eaf9eebe36dcaad00b53a1"></a>
## input_schema

`struct_field` · `datafusion_physical_plan::aggregates::AggregateExec::input_schema` · datafusion-physical-plan 55.1.0

```rust
input_schema: arrow::datatypes::SchemaRef
```

Source: `src/aggregates/mod.rs:856`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input schema before any aggregation is applied. For partial aggregate this will be the
same as input.schema() but for the final aggregate it will be the same as the input
to the partial aggregate, i.e., partial and final aggregates have same `input_schema`.
We need the input schema of partial aggregate to be able to deserialize aggregate
expressions from protobuf for final aggregate.

<a id="op-69244b850b75b43b667c77d2"></a>
## is_unordered_unfiltered_group_by_distinct

`function` · `datafusion_physical_plan::aggregates::AggregateExec::is_unordered_unfiltered_group_by_distinct` · datafusion-physical-plan 55.1.0

```rust
fn is_unordered_unfiltered_group_by_distinct(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1309`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

true, if this Aggregate has a group-by with no required or explicit ordering,
no filtering and no aggregate expressions
This method qualifies the use of the LimitedDistinctAggregation rewrite rule
on an AggregateExec.

<a id="op-ef9143a583b0aa5a571ac479"></a>
## limit_options

`function` · `datafusion_physical_plan::aggregates::AggregateExec::limit_options` · datafusion-physical-plan 55.1.0

```rust
fn limit_options(&self) -> Option<LimitOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the limit options (if set)

<a id="op-081f9d1fd91ff2603397ee72"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::aggregates::AggregateExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2027`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The output ordering of [`AggregateExec`](../operations/datafusion_physical_plan.aggregates.AggregateExec.md#op-6938ad75ef3f2f21fcb0d3c4) is determined by its `group_by`
columns. Although this method is not explicitly used by any optimizer
rules yet, overriding the default implementation ensures that it
accurately reflects the actual behavior.

If the [`InputOrderMode`](../operations/datafusion_physical_plan.ordering.InputOrderMode.md#op-ef30a392d6cfc56b340387f0) is `Linear`, the `group_by` columns don't have
an ordering, which means the results do not either. However, in the
`Ordered` and `PartiallyOrdered` cases, the `group_by` columns do have
an ordering, which is preserved in the output.

<a id="op-52119583032004ec8e074bcc"></a>
## metrics

`function` · `datafusion_physical_plan::aggregates::AggregateExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2130`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad980af2841db905951f497e"></a>
## mode

`function` · `datafusion_physical_plan::aggregates::AggregateExec::mode` · datafusion-physical-plan 55.1.0

```rust
fn mode(&self) -> &AggregateMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1058`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Aggregation mode (full, partial)

<a id="op-8df09514bdb4ec89c8e86bcd"></a>
## name

`function` · `datafusion_physical_plan::aggregates::AggregateExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:1987`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4f0bbbaf90e0e406a1b2427"></a>
## output_group_expr

`function` · `datafusion_physical_plan::aggregates::AggregateExec::output_group_expr` · datafusion-physical-plan 55.1.0

```rust
fn output_group_expr(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1079`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Grouping expressions as they occur in the output schema

<a id="op-03525a4c3441571fad32fc18"></a>
## properties

`function` · `datafusion_physical_plan::aggregates::AggregateExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:1992`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for down-casting

<a id="op-e4a64515d6943b8bd674ec5d"></a>
## replace_children

`function` · `datafusion_physical_plan::aggregates::AggregateExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2035`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8eb085011a745b4a32d93b3"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::aggregates::AggregateExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:1996`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4d184fa4c5ba3f5295e8c59"></a>
## required_input_ordering

`function` · `datafusion_physical_plan::aggregates::AggregateExec::required_input_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2014`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7483ca31cf3fa324375545ad"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::aggregates::AggregateExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2138`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd1495f6897b5a293954ac72"></a>
## try_from_proto

`function` · `datafusion_physical_plan::aggregates::AggregateExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2437, 1], "end": [2651, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:2443`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct an [`AggregateExec`](../operations/datafusion_physical_plan.aggregates.AggregateExec.md#op-6938ad75ef3f2f21fcb0d3c4) from its protobuf representation.

Grouping expressions are decoded against the child schema. Aggregate
arguments, ordering, filters, and the dynamic filter are decoded against
the aggregate input schema carried in the protobuf node.

<a id="op-94e6bd492716b029917cd6ff"></a>
## try_new

`function` · `datafusion_physical_plan::aggregates::AggregateExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(mode: AggregateMode, group_by: impl Into<Arc<PhysicalGroupBy>>, aggr_expr: Vec<Arc<AggregateFunctionExpr>>, filter_expr: Vec<Option<Arc<dyn PhysicalExpr>>>, input: Arc<dyn ExecutionPlan>, input_schema: SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:923`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new hash aggregate execution plan

<a id="op-a80337ed719731166076e897"></a>
## try_to_proto

`function` · `datafusion_physical_plan::aggregates::AggregateExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2248`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e54618637d26e4e8cc1f452"></a>
## with_dynamic_filter_expr

`function` · `datafusion_physical_plan::aggregates::AggregateExec::with_dynamic_filter_expr` · datafusion-physical-plan 55.1.0

```rust
fn with_dynamic_filter_expr(self, filter: Arc<DynamicFilterPhysicalExpr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Replace the dynamic filter expression. This method errors if the aggregate does not
support dynamic filtering or if the filter expression is incompatible with this
[`AggregateExec`](../operations/datafusion_physical_plan.aggregates.AggregateExec.md#op-6938ad75ef3f2f21fcb0d3c4).

<a id="op-4255c2e3a003f3159bb9a425"></a>
## with_limit_options

`function` · `datafusion_physical_plan::aggregates::AggregateExec::with_limit_options` · datafusion-physical-plan 55.1.0

```rust
fn with_limit_options(self, limit_options: Option<LimitOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:1063`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the limit options for this AggExec

<a id="op-e6c10e451c5361055bac47f7"></a>
## with_new_aggr_exprs

`function` · `datafusion_physical_plan::aggregates::AggregateExec::with_new_aggr_exprs` · datafusion-physical-plan 55.1.0

```rust
fn with_new_aggr_exprs(&self, aggr_expr: impl Into<Arc<[Arc<AggregateFunctionExpr>]>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:876`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Function used in `OptimizeAggregateOrder` optimizer rule,
where we need parts of the new value, others cloned from the old one
Rewrites aggregate exec with new aggregate expressions.

<a id="op-db7191611634c83591c37673"></a>
## with_new_children

`function` · `datafusion_physical_plan::aggregates::AggregateExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2064`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2b515cb08cb00e635241a1a"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::aggregates::AggregateExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 1], "end": [2359, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/aggregates/mod.rs:2111`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-897b2339923f0474c0c878fa"></a>
## with_new_limit_options

`function` · `datafusion_physical_plan::aggregates::AggregateExec::with_new_limit_options` · datafusion-physical-plan 55.1.0

```rust
fn with_new_limit_options(&self, limit_options: Option<LimitOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateExec", "path": "AggregateExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 1], "end": [1835, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:899`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Clone this exec, overriding only the limit hint.
