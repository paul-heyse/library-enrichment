# `datafusion_functions::core::planner::CoreFunctionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.planner.CoreFunctionPlanner.json).

<a id="op-de62896e16373bf10f5b558a"></a>
## CoreFunctionPlanner

`struct` · `datafusion_functions::core::planner::CoreFunctionPlanner` · datafusion-functions 55.1.0

```rust
struct CoreFunctionPlanner
```

Source: `src/core/planner.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51b916c4e3e5c0d91f4179ad"></a>
## default

`function` · `datafusion_functions::core::planner::CoreFunctionPlanner::default` · datafusion-functions 55.1.0

```rust
fn default() -> CoreFunctionPlanner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::planner::CoreFunctionPlanner", "path": "CoreFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 17], "filename": "src/core/planner.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/planner.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de45f9495d45b03e78fab286"></a>
## fmt

`function` · `datafusion_functions::core::planner::CoreFunctionPlanner::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::planner::CoreFunctionPlanner", "path": "CoreFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 19], "end": [27, 24], "filename": "src/core/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/planner.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd7fe5fc9409ead0e0963612"></a>
## plan_compound_identifier

`function` · `datafusion_functions::core::planner::CoreFunctionPlanner::plan_compound_identifier` · datafusion-functions 55.1.0

```rust
fn plan_compound_identifier(&self, field: &Field, qualifier: Option<&TableReference>, nested_names: &[String]) -> Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::planner::CoreFunctionPlanner", "path": "CoreFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [89, 2], "filename": "src/core/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/core/planner.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a257f5d9659a8f5761e2bc8"></a>
## plan_dictionary_literal

`function` · `datafusion_functions::core::planner::CoreFunctionPlanner::plan_dictionary_literal` · datafusion-functions 55.1.0

```rust
fn plan_dictionary_literal(&self, expr: RawDictionaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawDictionaryExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::planner::CoreFunctionPlanner", "path": "CoreFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [89, 2], "filename": "src/core/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/core/planner.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3560ea16c89ead1a60196de1"></a>
## plan_overlay

`function` · `datafusion_functions::core::planner::CoreFunctionPlanner::plan_overlay` · datafusion-functions 55.1.0

```rust
fn plan_overlay(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::planner::CoreFunctionPlanner", "path": "CoreFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [89, 2], "filename": "src/core/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/core/planner.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11b942a56f6e0d2bd9c0590d"></a>
## plan_struct_literal

`function` · `datafusion_functions::core::planner::CoreFunctionPlanner::plan_struct_literal` · datafusion-functions 55.1.0

```rust
fn plan_struct_literal(&self, args: Vec<Expr>, is_named_struct: bool) -> Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::planner::CoreFunctionPlanner", "path": "CoreFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [89, 2], "filename": "src/core/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/core/planner.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
