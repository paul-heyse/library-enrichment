# `datafusion_physical_optimizer::output_requirements::OutputRequirementExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.output_requirements.OutputRequirementExec.json).

<a id="op-b5659f2738d710cccbf36f7a"></a>
## OutputRequirementExec

`struct` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec` · datafusion-physical-optimizer 55.1.0

```rust
struct OutputRequirementExec
```

Source: `src/output_requirements.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

An ancillary, non-executable operator whose sole purpose is to track global
requirements during optimization. It imposes
- the ordering requirement in its `order_requirement` attribute.
- the distribution requirement in its `dist_requirement` attribute.

See [`OutputRequirements`](../operations/datafusion_physical_optimizer.output_requirements.OutputRequirements.md#op-d48d9c35fd3d8e9786869bdc) for more details

<a id="op-f732cf161b9778f8d028bc52"></a>
## apply_expressions

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::apply_expressions` · datafusion-physical-optimizer 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn datafusion_physical_expr_common::physical_expr::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a27dfce613dd53492aef20a8"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::benefits_from_input_partitioning` · datafusion-physical-optimizer 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e0cc4abfcb986a0a0bcd6f1"></a>
## child_stats_requests

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::child_stats_requests` · datafusion-physical-optimizer 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4176fabf138043f4e38b3123"></a>
## children

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::children` · datafusion-physical-optimizer 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd724cae50f5c371fb95a06c"></a>
## execute

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::execute` · datafusion-physical-optimizer 55.1.0

```rust
fn execute(&self, _partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34ab22fb2ea3c45a04fb0618"></a>
## fetch

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::fetch` · datafusion-physical-optimizer 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69b44a8000e857b13b142c64"></a>
## fetch

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::fetch` · datafusion-physical-optimizer 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [157, 2], "filename": "src/output_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/output_requirements.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Get fetch

<a id="op-387e0b54116ea2c83e76a9a9"></a>
## fmt

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 10], "end": [104, 15], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/output_requirements.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c5314fbd74d835879f26be5"></a>
## fmt_as

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::fmt_as` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [200, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/output_requirements.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21fe7caf3e9e7d0e870145e2"></a>
## input

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::input` · datafusion-physical-optimizer 55.1.0

```rust
fn input(&self) -> Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [157, 2], "filename": "src/output_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/output_requirements.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b719f932c99087c96a296a10"></a>
## input_distribution_requirements

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::input_distribution_requirements` · datafusion-physical-optimizer 55.1.0

```rust
fn input_distribution_requirements(&self) -> datafusion_physical_plan::InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d24410d741d16639daeb3e0"></a>
## maintains_input_order

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::maintains_input_order` · datafusion-physical-optimizer 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c32231daa325fa53552ccfd"></a>
## name

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6688c401bca197fef9bf7bb2"></a>
## new

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>, requirements: Option<OrderingRequirements>, dist_requirement: Distribution, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [157, 2], "filename": "src/output_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/output_requirements.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce53bd9ed09c42ddff11f9c7"></a>
## properties

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::properties` · datafusion-physical-optimizer 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de04a24695b93d21e9ad7ed7"></a>
## replace_children

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::replace_children` · datafusion-physical-optimizer 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41754bd86096866beafdbc69"></a>
## required_input_distribution

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::required_input_distribution` · datafusion-physical-optimizer 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f908529ae3e547879d5b7e1a"></a>
## required_input_ordering

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::required_input_ordering` · datafusion-physical-optimizer 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5acfaf0579305206d9ecaf0f"></a>
## statistics_from_inputs

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::statistics_from_inputs` · datafusion-physical-optimizer 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc23ceb6bcdd7c6622e21aaa"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::try_swapping_with_projection` · datafusion-physical-optimizer 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ec1c8776d7aa44c8c93f3ba"></a>
## with_new_children

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirementExec::with_new_children` · datafusion-physical-optimizer 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirementExec", "path": "OutputRequirementExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [352, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/output_requirements.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
