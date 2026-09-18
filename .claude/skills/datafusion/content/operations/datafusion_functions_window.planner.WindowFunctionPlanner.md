# `datafusion_functions_window::planner::WindowFunctionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.planner.WindowFunctionPlanner.json).

<a id="op-b4964fe57f781793abd6a5ef"></a>
## WindowFunctionPlanner

`struct` · `datafusion_functions_window::planner::WindowFunctionPlanner` · datafusion-functions-window 55.1.0

```rust
struct WindowFunctionPlanner
```

Source: `src/planner.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2adf17d9e35b85f5a1f768a3"></a>
## fmt

`function` · `datafusion_functions_window::planner::WindowFunctionPlanner::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::planner::WindowFunctionPlanner", "path": "WindowFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05e94acd91ebec597e7b5a7b"></a>
## plan_window

`function` · `datafusion_functions_window::planner::WindowFunctionPlanner::plan_window` · datafusion-functions-window 55.1.0

```rust
fn plan_window(&self, raw_expr: RawWindowExpr) -> Result<PlannerResult<RawWindowExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::planner::WindowFunctionPlanner", "path": "WindowFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [127, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
