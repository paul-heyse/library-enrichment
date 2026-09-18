# `datafusion_physical_plan::analyze::AnalyzeExecBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.analyze.AnalyzeExecBuilder.json).

<a id="op-7a0994ec55008ae5c24cc9c7"></a>
## AnalyzeExecBuilder

`struct` · `datafusion_physical_plan::analyze::AnalyzeExecBuilder` · datafusion-physical-plan 55.1.0

```rust
struct AnalyzeExecBuilder
```

Source: `src/analyze.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Builder for [`AnalyzeExec`](../operations/datafusion_physical_plan.analyze.AnalyzeExec.md#op-1251267e2bca364262b2c889).

Builder for [AnalyzeExec](../operations/datafusion_physical_plan.analyze.AnalyzeExec.md#op-1251267e2bca364262b2c889).

<a id="op-872b4563cd89873838495593"></a>
## build

`function` · `datafusion_physical_plan::analyze::AnalyzeExecBuilder::build` · datafusion-physical-plan 55.1.0

```rust
fn build(self) -> AnalyzeExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExecBuilder", "path": "AnalyzeExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [132, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa6172afff2bee34c07ff04c"></a>
## new

`function` · `datafusion_physical_plan::analyze::AnalyzeExecBuilder::new` · datafusion-physical-plan 55.1.0

```rust
fn new(verbose: bool, show_statistics: bool, input: Arc<dyn ExecutionPlan>, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExecBuilder", "path": "AnalyzeExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [132, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81e323fc019b94d82d35156e"></a>
## with_format

`function` · `datafusion_physical_plan::analyze::AnalyzeExecBuilder::with_format` · datafusion-physical-plan 55.1.0

```rust
fn with_format(self, format: ExplainFormat) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExecBuilder", "path": "AnalyzeExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [132, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deacfc8f562211aa1991ea25"></a>
## with_metric_categories

`function` · `datafusion_physical_plan::analyze::AnalyzeExecBuilder::with_metric_categories` · datafusion-physical-plan 55.1.0

```rust
fn with_metric_categories(self, metric_categories: Option<Vec<MetricCategory>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExecBuilder", "path": "AnalyzeExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [132, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b76cb4ad97b296178fabaa6a"></a>
## with_metric_types

`function` · `datafusion_physical_plan::analyze::AnalyzeExecBuilder::with_metric_types` · datafusion-physical-plan 55.1.0

```rust
fn with_metric_types(self, metric_types: Vec<MetricType>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::analyze::AnalyzeExecBuilder", "path": "AnalyzeExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [132, 2], "filename": "src/analyze.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyze.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
