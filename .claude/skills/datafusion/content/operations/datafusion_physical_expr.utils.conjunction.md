# `datafusion_physical_expr::utils::conjunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.conjunction.json).

<a id="op-179756f9bb62853488a6aee1"></a>
## conjunction

`function` · `datafusion_physical_expr::utils::conjunction` · datafusion-physical-expr 55.1.0

```rust
fn conjunction(predicates: impl IntoIterator<Item = std::sync::Arc<dyn PhysicalExpr>>) -> std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/utils/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a conjunction of the given predicates.
If the input is empty, return a literal true.
If the input contains a single predicate, return the predicate.
Otherwise, return a conjunction of the predicates (e.g. `a AND b AND c`).
