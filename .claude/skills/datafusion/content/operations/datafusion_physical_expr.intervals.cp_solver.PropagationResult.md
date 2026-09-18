# `datafusion_physical_expr::intervals::cp_solver::PropagationResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.cp_solver.PropagationResult.json).

<a id="op-afac560a3e2e02bb5d86d7b0"></a>
## PropagationResult

`enum` · `datafusion_physical_expr::intervals::cp_solver::PropagationResult` · datafusion-physical-expr 55.1.0

```rust
enum PropagationResult
```

Source: `src/intervals/cp_solver.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This object encapsulates all possible constraint propagation results.

<a id="op-6f4eb2730dc632427ae4a40c"></a>
## CannotPropagate

`variant` · `datafusion_physical_expr::intervals::cp_solver::PropagationResult::CannotPropagate` · datafusion-physical-expr 55.1.0

```rust
CannotPropagate
```

Source: `src/intervals/cp_solver.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdffc76720c0d3624573def6"></a>
## Infeasible

`variant` · `datafusion_physical_expr::intervals::cp_solver::PropagationResult::Infeasible` · datafusion-physical-expr 55.1.0

```rust
Infeasible
```

Source: `src/intervals/cp_solver.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c37b768c2ef0e51bce7331"></a>
## Success

`variant` · `datafusion_physical_expr::intervals::cp_solver::PropagationResult::Success` · datafusion-physical-expr 55.1.0

```rust
Success
```

Source: `src/intervals/cp_solver.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b66467eeb344426687ad2978"></a>
## eq

`function` · `datafusion_physical_expr::intervals::cp_solver::PropagationResult::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &PropagationResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::PropagationResult", "path": "PropagationResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 10], "end": [174, 19], "filename": "src/intervals/cp_solver.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/intervals/cp_solver.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf54685db482914ac27f868e"></a>
## fmt

`function` · `datafusion_physical_expr::intervals::cp_solver::PropagationResult::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::PropagationResult", "path": "PropagationResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 21], "end": [174, 26], "filename": "src/intervals/cp_solver.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/intervals/cp_solver.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
