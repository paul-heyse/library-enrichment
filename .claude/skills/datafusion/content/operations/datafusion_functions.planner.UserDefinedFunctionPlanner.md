# `datafusion_functions::planner::UserDefinedFunctionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.planner.UserDefinedFunctionPlanner.json).

<a id="op-d8bfbba04325c0d9632b1ee2"></a>
## UserDefinedFunctionPlanner

`struct` · `datafusion_functions::planner::UserDefinedFunctionPlanner` · datafusion-functions 55.1.0

```rust
struct UserDefinedFunctionPlanner
```

Source: `src/planner.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c8504890cac922bd888c61e"></a>
## default

`function` · `datafusion_functions::planner::UserDefinedFunctionPlanner::default` · datafusion-functions 55.1.0

```rust
fn default() -> UserDefinedFunctionPlanner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::planner::UserDefinedFunctionPlanner", "path": "UserDefinedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 17], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/planner.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fc67a54a25eb2ac287bf956"></a>
## fmt

`function` · `datafusion_functions::planner::UserDefinedFunctionPlanner::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::planner::UserDefinedFunctionPlanner", "path": "UserDefinedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 19], "end": [31, 24], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2394e25b01170083e7fdc8bb"></a>
## plan_extract

`function` · `datafusion_functions::planner::UserDefinedFunctionPlanner::plan_extract` · datafusion-functions 55.1.0

```rust
fn plan_extract(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::planner::UserDefinedFunctionPlanner", "path": "UserDefinedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [56, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63eabe757b622897d6b9603b"></a>
## plan_position

`function` · `datafusion_functions::planner::UserDefinedFunctionPlanner::plan_position` · datafusion-functions 55.1.0

```rust
fn plan_position(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::planner::UserDefinedFunctionPlanner", "path": "UserDefinedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [56, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aea0dbdb3e6ad274e2d2d9cb"></a>
## plan_substring

`function` · `datafusion_functions::planner::UserDefinedFunctionPlanner::plan_substring` · datafusion-functions 55.1.0

```rust
fn plan_substring(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::planner::UserDefinedFunctionPlanner", "path": "UserDefinedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [56, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
