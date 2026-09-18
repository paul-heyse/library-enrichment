# `datafusion_functions::datetime::planner::DatetimeFunctionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.planner.DatetimeFunctionPlanner.json).

<a id="op-7ea2a93e1a30a9409622297e"></a>
## DatetimeFunctionPlanner

`struct` · `datafusion_functions::datetime::planner::DatetimeFunctionPlanner` · datafusion-functions 55.1.0

```rust
struct DatetimeFunctionPlanner
```

Source: `src/datetime/planner.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07b2dbb78a39765a3ab901dc"></a>
## default

`function` · `datafusion_functions::datetime::planner::DatetimeFunctionPlanner::default` · datafusion-functions 55.1.0

```rust
fn default() -> DatetimeFunctionPlanner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::planner::DatetimeFunctionPlanner", "path": "DatetimeFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 17], "filename": "src/datetime/planner.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/planner.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c89a8f318b811f2bbf3acb76"></a>
## fmt

`function` · `datafusion_functions::datetime::planner::DatetimeFunctionPlanner::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::planner::DatetimeFunctionPlanner", "path": "DatetimeFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 19], "end": [23, 24], "filename": "src/datetime/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/planner.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d514e3e695b5f12993078aa"></a>
## plan_extract

`function` · `datafusion_functions::datetime::planner::DatetimeFunctionPlanner::plan_extract` · datafusion-functions 55.1.0

```rust
fn plan_extract(&self, args: Vec<Expr>) -> datafusion_common::Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::planner::DatetimeFunctionPlanner", "path": "DatetimeFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [35, 2], "filename": "src/datetime/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/datetime/planner.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
