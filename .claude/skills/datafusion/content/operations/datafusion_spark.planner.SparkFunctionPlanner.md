# `datafusion_spark::planner::SparkFunctionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.planner.SparkFunctionPlanner.json).

<a id="op-e6b63867ad6e41fad9c0157a"></a>
## SparkFunctionPlanner

`struct` · `datafusion_spark::planner::SparkFunctionPlanner` · datafusion-spark 55.1.0

```rust
struct SparkFunctionPlanner
```

Source: `src/planner.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13057f84098dc8314d6cca90"></a>
## default

`function` · `datafusion_spark::planner::SparkFunctionPlanner::default` · datafusion-spark 55.1.0

```rust
fn default() -> SparkFunctionPlanner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::planner::SparkFunctionPlanner", "path": "SparkFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 17], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/planner.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a846ce1b54805c178001aa74"></a>
## fmt

`function` · `datafusion_spark::planner::SparkFunctionPlanner::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::planner::SparkFunctionPlanner", "path": "SparkFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 19], "end": [22, 24], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49294a03a35d60eef5be7f16"></a>
## plan_extract

`function` · `datafusion_spark::planner::SparkFunctionPlanner::plan_extract` · datafusion-spark 55.1.0

```rust
fn plan_extract(&self, args: Vec<Expr>) -> datafusion_common::Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::planner::SparkFunctionPlanner", "path": "SparkFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [43, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24c4ca74892dc9d5d3f0c7c2"></a>
## plan_substring

`function` · `datafusion_spark::planner::SparkFunctionPlanner::plan_substring` · datafusion-spark 55.1.0

```rust
fn plan_substring(&self, args: Vec<Expr>) -> datafusion_common::Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::planner::SparkFunctionPlanner", "path": "SparkFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [43, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
