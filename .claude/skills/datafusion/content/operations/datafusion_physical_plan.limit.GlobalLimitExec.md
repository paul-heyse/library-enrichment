# `datafusion_physical_plan::limit::GlobalLimitExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.limit.GlobalLimitExec.json).

<a id="op-e1b7f44642275634519186d0"></a>
## GlobalLimitExec

`struct` · `datafusion_physical_plan::limit::GlobalLimitExec` · datafusion-physical-plan 55.1.0

```rust
struct GlobalLimitExec
```

Source: `src/limit.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Limit execution plan

<a id="op-bcf30cccb123e63c4ce3431a"></a>
## apply_expressions

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a88d7d3e6551225edbf8f09"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-677f83c5b535b22d267aa55e"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e9ff0ce0f14fc1b751b4926"></a>
## children

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-739051ecb17e3b83b30e770e"></a>
## clone

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> GlobalLimitExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 22], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/limit.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e87ae2b4376751e2ccdd24d2"></a>
## execute

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53e2b1657256ed6b638317ae"></a>
## fetch

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [113, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Maximum number of rows to fetch

<a id="op-66c912a4b570818d180914a1"></a>
## fetch

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8f0d5a52dc7302bcfb6782c"></a>
## fmt

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/limit.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c03c4c808adc42b090362680"></a>
## fmt_as

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [139, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/limit.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c962ba59c3220f331d46073"></a>
## input

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [113, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input execution plan

<a id="op-378746df5d566d599dfe2e32"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d270dbcde8f694814f51f134"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8cc9af49f4bfe5458ae5970"></a>
## metrics

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e715217110a744d0d338d446"></a>
## name

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2950a2c22c539e98f3d37f9f"></a>
## new

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>, skip: usize, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [113, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new GlobalLimitExec

<a id="op-769cf4249377250043eff1f4"></a>
## properties

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-3ee4606a5c947766746dc975"></a>
## replace_children

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b581ef561f2e9bacadf3d46c"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bfa765fc37f63c50bc3aee1"></a>
## required_ordering

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::required_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_ordering(&self) -> &Option<LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [113, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the required ordering from limit

<a id="op-97ff56680d2e1d6a4c562f50"></a>
## set_required_ordering

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::set_required_ordering` · datafusion-physical-plan 55.1.0

```rust
fn set_required_ordering(&mut self, required_ordering: Option<LexOrdering>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [113, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the required ordering for limit

<a id="op-4e89eb6318f4160a22c1c5cb"></a>
## skip

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::skip` · datafusion-physical-plan 55.1.0

```rust
fn skip(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [113, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Number of rows to skip before fetch

<a id="op-bdc437e18c02c26f6f1e1636"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1574d59b867b1904f15975f9"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54ae228176f57ddb2dca04ff"></a>
## try_from_proto

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 1], "end": [335, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-702b297c8ce9090090afadd2"></a>
## try_to_proto

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6108a709d1fc7f65f76e50ca"></a>
## with_new_children

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c5a2b72cd5e443372942371"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::limit::GlobalLimitExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::GlobalLimitExec", "path": "GlobalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [302, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
