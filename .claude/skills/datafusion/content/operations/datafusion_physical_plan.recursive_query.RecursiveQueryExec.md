# `datafusion_physical_plan::recursive_query::RecursiveQueryExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.recursive_query.RecursiveQueryExec.json).

<a id="op-8f07673fb7a8398531345f93"></a>
## RecursiveQueryExec

`struct` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec` · datafusion-physical-plan 55.1.0

```rust
struct RecursiveQueryExec
```

Source: `src/recursive_query.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Recursive query execution plan.

This plan has two components: a base part (the static term) and
a dynamic part (the recursive term). The execution will start from
the base, and as long as the previous iteration produced at least
a single new row (taking care of the distinction) the recursive
part will be continuously executed.

Before each execution of the dynamic part, the rows from the previous
iteration will be available in a "working table" (not a real table,
can be only accessed using a continuance operation).

Note that there won't be any limit or checks applied to detect
an infinite recursion, so it is up to the planner to ensure that
it won't happen.

<a id="op-3058d54fd18c0c4261d7fa03"></a>
## apply_expressions

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d36e38eb2d588dd10912703a"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f774d525efb934c9a7fa0f48"></a>
## children

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef72e14879865149dce17796"></a>
## clone

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> RecursiveQueryExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 17], "end": [67, 22], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/recursive_query.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-107fb565c0131ed715b5dc01"></a>
## execute

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-260adf382c0d1c4e59c8ea38"></a>
## fmt

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/recursive_query.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-917b1d6ecda1da23f618db6e"></a>
## fmt_as

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [260, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/recursive_query.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e6ec011b906418e0049f373"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb5966441ec3169a00bcd900"></a>
## is_distinct

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::is_distinct` · datafusion-physical-plan 55.1.0

```rust
fn is_distinct(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [143, 2], "filename": "src/recursive_query.rs"}, "trait": null, "trait_path": null}`

Source: `src/recursive_query.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

is distinct

<a id="op-fb075ac537850b3d10135b70"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a45420d876bb4207ada6b33"></a>
## metrics

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33990a1b4f5db625f0733f90"></a>
## name

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [143, 2], "filename": "src/recursive_query.rs"}, "trait": null, "trait_path": null}`

Source: `src/recursive_query.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to name

<a id="op-d9a450f8f77f10a61cead232"></a>
## name

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-511479336f21ae07290ce824"></a>
## properties

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87f5a43b5a32543767b9142e"></a>
## recursive_term

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::recursive_term` · datafusion-physical-plan 55.1.0

```rust
fn recursive_term(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [143, 2], "filename": "src/recursive_query.rs"}, "trait": null, "trait_path": null}`

Source: `src/recursive_query.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to recursive term

<a id="op-4c94a490344872c0b188394f"></a>
## replace_children

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-981029ffc4d7c3d3be228fc5"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e09a7825cc9d7e3aa90d148"></a>
## static_term

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::static_term` · datafusion-physical-plan 55.1.0

```rust
fn static_term(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [143, 2], "filename": "src/recursive_query.rs"}, "trait": null, "trait_path": null}`

Source: `src/recursive_query.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to static term

<a id="op-c9675edce20120662d9eb002"></a>
## try_new

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(name: String, output_schema: SchemaRef, static_term: Arc<dyn ExecutionPlan>, recursive_term: Arc<dyn ExecutionPlan>, is_distinct: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [143, 2], "filename": "src/recursive_query.rs"}, "trait": null, "trait_path": null}`

Source: `src/recursive_query.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new RecursiveQueryExec

<a id="op-4f8b6498687b3158f6e2818e"></a>
## with_new_children

`function` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::recursive_query::RecursiveQueryExec", "path": "RecursiveQueryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [238, 2], "filename": "src/recursive_query.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/recursive_query.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
