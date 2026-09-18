# `datafusion_physical_plan::display::DisplayableExecutionPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.display.DisplayableExecutionPlan.json).

<a id="op-ff5dd0052d13e66e5b983e42"></a>
## DisplayableExecutionPlan

`struct` · `datafusion_physical_plan::display::DisplayableExecutionPlan` · datafusion-physical-plan 55.1.0

```rust
struct DisplayableExecutionPlan<'a>
```

Source: `src/display.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Wraps an `ExecutionPlan` with various methods for formatting


# Example
```
# use std::sync::Arc;
# use arrow::datatypes::{Field, Schema, DataType};
# use datafusion_expr::Operator;
# use datafusion_physical_expr::expressions::{binary, col, lit};
# use datafusion_physical_plan::{displayable, ExecutionPlan};
# use datafusion_physical_plan::empty::EmptyExec;
# use datafusion_physical_plan::filter::FilterExec;
# let schema = Schema::new(vec![Field::new("i", DataType::Int32, false)]);
# let plan = EmptyExec::new(Arc::new(schema));
# let i = col("i", &plan.schema()).unwrap();
# let predicate = binary(i, Operator::Eq, lit(1), &plan.schema()).unwrap();
# let plan: Arc<dyn ExecutionPlan> = Arc::new(FilterExec::try_new(predicate, Arc::new(plan)).unwrap());
// Get a one line description (Displayable)
let display_plan = displayable(plan.as_ref());

// you can use the returned objects to format plans
// where you can use `Display` such as  format! or println!
assert_eq!(
   &format!("The plan is: {}", display_plan.one_line()),
  "The plan is: FilterExec: i@0 = 1\n"
);
// You can also print out the plan and its children in indented mode
assert_eq!(display_plan.indent(false).to_string(),
  "FilterExec: i@0 = 1\
  \n  EmptyExec\
  \n"
);
```

<a id="op-72719f04fa58e029879288ac"></a>
## clone

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> DisplayableExecutionPlan<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 17], "end": [118, 22], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/display.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37af5291106092a8f7ce6486"></a>
## fmt

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 10], "end": [118, 15], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/display.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-619af65aa8a8255413ee3aeb"></a>
## graphviz

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::graphviz` · datafusion-physical-plan 55.1.0

```rust
fn graphviz(&self) -> impl fmt::Display + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a `format`able structure that produces graphviz format for execution plan, which can
be directly visualized [here](https://dreampuf.github.io/GraphvizOnline).

An example is
```dot
strict digraph dot_plan {
```

<a id="op-c1ca8b51507fae2f584e69a6"></a>
## indent

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::indent` · datafusion-physical-plan 55.1.0

```rust
fn indent(&self, verbose: bool) -> impl fmt::Display + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a `format`able structure that produces a single line
per node.

```text
ProjectionExec: expr=[a]
  CoalesceBatchesExec: target_batch_size=8192
    FilterExec: a < 5
      RepartitionExec: partitioning=RoundRobinBatch(16)
        DataSourceExec: source=...",
```

<a id="op-8d8f02ad5e68664d1e87cfe2"></a>
## new

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::new` · datafusion-physical-plan 55.1.0

```rust
fn new(inner: &'a dyn ExecutionPlan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a wrapper around an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) which can be
pretty printed in a variety of ways

<a id="op-7982a3b420e5ada00e16c537"></a>
## one_line

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::one_line` · datafusion-physical-plan 55.1.0

```rust
fn one_line(&self) -> impl fmt::Display + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a single-line summary of the root of the plan
Example: `ProjectionExec: expr=[a@0 as a]`.

<a id="op-6755819d423075303ea63cb6"></a>
## pgjson

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::pgjson` · datafusion-physical-plan 55.1.0

```rust
fn pgjson(&self, verbose: bool) -> impl fmt::Display + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a `format`able structure that produces PostgreSQL-style JSON
output, mirroring the logical-plan pgjson format.

Each node is rendered as a JSON object with:
- `"Node Type"` — `ExecutionPlan::name()`
- `"Details"` — the one-line `DisplayAs::Default` rendering
- `"Output"` — schema column names (when `set_show_schema(true)`)
- `"Actual Rows"` / `"Actual Total Time"` — PG-canonical metric keys
  populated from `output_rows` / `elapsed_compute` when available
- `"Extras"` — remaining metrics keyed by DataFusion metric name
- `"Plans"` — array of child nodes

When a summary has been set via [`Self::set_summary`](../operations/datafusion_physical_plan.display.DisplayableExecutionPlan.md#op-35e2f14d691cac9272572943), `"Total Rows"`
and `"Duration"` fields are attached at the root.

<a id="op-53887b3ab66c083874389d70"></a>
## set_metric_categories

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::set_metric_categories` · datafusion-physical-plan 55.1.0

```rust
fn set_metric_categories(self, metric_categories: Option<Vec<MetricCategory>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specify which metric categories to include.

- `None` means show all categories (default).
- `Some(vec![])` means plan-only — suppress all metrics.
- `Some(vec![Rows])` means show only row-count metrics (plus
  uncategorized metrics).

See [`MetricCategory`](../operations/datafusion_common.format.MetricCategory.md#op-d05732d49bd39fe3321965e4) for the determinism properties of each
category.

<a id="op-69ceabf04cd594dfb9017446"></a>
## set_metric_names

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::set_metric_names` · datafusion-physical-plan 55.1.0

```rust
fn set_metric_names(self, metric_names: Vec<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specify which metric names to include.

- An empty vector means plan-only — suppress all metrics.
- `vec!["metric_1"]` means show only the metric named `metric_1`.

Name filtering is intersected with other types of filters, like metric
category and metric type.

<a id="op-1c17cf4aba8de276265a5d85"></a>
## set_metric_types

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::set_metric_types` · datafusion-physical-plan 55.1.0

```rust
fn set_metric_types(self, metric_types: Vec<MetricType>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specify which metric types should be rendered alongside the plan

<a id="op-e1473bd2863f1cda5f527c73"></a>
## set_show_schema

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::set_show_schema` · datafusion-physical-plan 55.1.0

```rust
fn set_show_schema(self, show_schema: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Enable display of schema

If true, plans will be displayed with schema information at the end
of each line. The format is `schema=[[a:Int32;N, b:Int32;N, c:Int32;N]]`

<a id="op-946acd1837705112a4c2dfb9"></a>
## set_show_statistics

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::set_show_statistics` · datafusion-physical-plan 55.1.0

```rust
fn set_show_statistics(self, show_statistics: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Enable display of statistics

<a id="op-35e2f14d691cac9272572943"></a>
## set_summary

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::set_summary` · datafusion-physical-plan 55.1.0

```rust
fn set_summary(self, total_rows: Option<usize>, duration: Option<Duration>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Attach an `EXPLAIN ANALYZE` summary (total output rows and duration)
to the rendered output. Currently only used by [`Self::pgjson`](../operations/datafusion_physical_plan.display.DisplayableExecutionPlan.md#op-6755819d423075303ea63cb6), which
serializes the summary alongside the root plan object.

<a id="op-d0049d3cc3d46e0742e5ac79"></a>
## set_tree_maximum_render_width

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::set_tree_maximum_render_width` · datafusion-physical-plan 55.1.0

```rust
fn set_tree_maximum_render_width(self, width: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the maximum render width for the tree format

<a id="op-899596dc275bc920b2e1ec2b"></a>
## to_stringified

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::to_stringified` · datafusion-physical-plan 55.1.0

```rust
fn to_stringified(&self, verbose: bool, plan_type: PlanType, explain_format: DisplayFormatType) -> StringifiedPlan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b499f1a119ce771e1a7a6c0e"></a>
## tree_render

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::tree_render` · datafusion-physical-plan 55.1.0

```rust
fn tree_render(&self) -> impl fmt::Display + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:388`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Formats the plan using a ASCII art like tree

See [`DisplayFormatType::TreeRender`](../operations/datafusion_physical_plan.display.DisplayFormatType.md#op-7c86717fca73df82a1e6ac1d) for more details.

<a id="op-8e8a11add13395f59424203f"></a>
## with_full_metrics

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::with_full_metrics` · datafusion-physical-plan 55.1.0

```rust
fn with_full_metrics(inner: &'a dyn ExecutionPlan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a wrapper around an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) which can be
pretty printed in a variety of ways that also shows all low
level metrics

<a id="op-9d14eb8d7491bef97987a52e"></a>
## with_metrics

`function` · `datafusion_physical_plan::display::DisplayableExecutionPlan::with_metrics` · datafusion-physical-plan 55.1.0

```rust
fn with_metrics(inner: &'a dyn ExecutionPlan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::DisplayableExecutionPlan", "path": "DisplayableExecutionPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [536, 2], "filename": "src/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/display.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a wrapper around an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) which can be
pretty printed in a variety of ways that also shows aggregated
metrics
