# `datafusion_expr::planner::PlannerResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.PlannerResult.json).

<a id="op-614b5204940cdee82fa3228a"></a>
## PlannerResult

`enum` · `datafusion_expr::planner::PlannerResult` · datafusion-expr 55.1.0

```rust
enum PlannerResult<T>
```

Source: `src/planner.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Result of planning a raw expr with [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd)

<a id="op-86b4613cf3984cad1ecf44c9"></a>
## Original

`variant` · `datafusion_expr::planner::PlannerResult::Original` · datafusion-expr 55.1.0

```rust
Original
```

Source: `src/planner.rs:353`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The raw expression could not be planned, and is returned unmodified

<a id="op-0abfcee760beb63f9579ed26"></a>
## Planned

`variant` · `datafusion_expr::planner::PlannerResult::Planned` · datafusion-expr 55.1.0

```rust
Planned
```

Source: `src/planner.rs:351`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The raw expression was successfully planned as a new [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)

<a id="op-4320b2f65e6417ddb05d2d06"></a>
## clone

`function` · `datafusion_expr::planner::PlannerResult::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> PlannerResult<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_expr::planner::PlannerResult", "path": "PlannerResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 17], "end": [348, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-954b7006a461c58ee7103837"></a>
## fmt

`function` · `datafusion_expr::planner::PlannerResult::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_expr::planner::PlannerResult", "path": "PlannerResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 10], "end": [348, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
