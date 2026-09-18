# `datafusion_physical_plan::analyze::AnalyzeExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.analyze.AnalyzeExec.json).

<a id="op-1251267e2bca364262b2c889"></a>
## AnalyzeExec

`struct` · `datafusion_physical_plan::analyze::AnalyzeExec` · datafusion-physical-plan 55.1.0

```rust
struct AnalyzeExec
```

Source: `src/analyze.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

`EXPLAIN ANALYZE` execution plan operator. This operator runs its input,
discards the results, and then prints out an annotated plan with metrics

<a id="op-411426257c414868d1f81290"></a>
## apply_expressions

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff0034cbaf08798ea25ca97a"></a>
## builder

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::builder` · datafusion-physical-plan 55.1.0

```rust
fn builder(verbose: bool, show_statistics: bool, input: Arc<dyn ExecutionPlan>, schema: SchemaRef) -> AnalyzeExecBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [183, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a builder for constructing an [`AnalyzeExec`](../operations/datafusion_physical_plan.analyze.AnalyzeExec.md#op-1251267e2bca364262b2c889).

<a id="op-b37f6427c49080cb25f8e1b0"></a>
## children

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5450253564204affdeaced70"></a>
## clone

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> AnalyzeExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 17], "end": [50, 22], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/analyze.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83221308933bc9fd39ad4947"></a>
## execute

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee25b37cf3064601089cd01c"></a>
## fmt

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 10], "end": [50, 15], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/analyze.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-245e318b9a8dbbdf005b8bce"></a>
## fmt_as

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [201, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/analyze.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93c45434a767e7a0261d3b08"></a>
## format

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::format` · datafusion-physical-plan 55.1.0

```rust
fn format(&self) -> &ExplainFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [183, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Access to format

<a id="op-d217ca4caf913a1b16fbcf6e"></a>
## input

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [183, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The input plan

<a id="op-5dcaf1338190f4bc932339e3"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84aa69b50230450b3685aeee"></a>
## metric_categories

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::metric_categories` · datafusion-physical-plan 55.1.0

```rust
fn metric_categories(&self) -> Option<&[MetricCategory]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [183, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Access to metric_categories

<a id="op-bfa745024ae282671fc84148"></a>
## name

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7821ad5a9a1ebf1d9f6e03a"></a>
## properties

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-0dee1572a8c458bd0d73cbc7"></a>
## replace_children

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc3bdd669b8f9a2d19e4d7e9"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58d6037086cced29613f9c54"></a>
## show_statistics

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::show_statistics` · datafusion-physical-plan 55.1.0

```rust
fn show_statistics(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [183, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Access to show_statistics

<a id="op-1f2329c48a1faf2049be0514"></a>
## try_from_proto

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [452, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct an [`AnalyzeExec`](../operations/datafusion_physical_plan.analyze.AnalyzeExec.md#op-1251267e2bca364262b2c889) from its protobuf representation.

<a id="op-9ece582408eab40263c8b08b"></a>
## try_to_proto

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25ac6f98f6d97814e9c0f61f"></a>
## verbose

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::verbose` · datafusion-physical-plan 55.1.0

```rust
fn verbose(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [183, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Access to verbose

<a id="op-0fd41d0bb588e53874806ed6"></a>
## with_new_children

`function` · `datafusion_physical_plan::analyze::AnalyzeExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExec", "path": "AnalyzeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [384, 2], "filename": "src/analyze.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/analyze.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
