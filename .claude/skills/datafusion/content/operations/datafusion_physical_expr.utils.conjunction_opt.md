# `datafusion_physical_expr::utils::conjunction_opt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.conjunction_opt.json).

<a id="op-b26426793f5138da84439e9a"></a>
## conjunction_opt

`function` · `datafusion_physical_expr::utils::conjunction_opt` · datafusion-physical-expr 55.1.0

```rust
fn conjunction_opt(predicates: impl IntoIterator<Item = std::sync::Arc<dyn PhysicalExpr>>) -> Option<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/utils/mod.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a conjunction of the given predicates.
If the input is empty or the return None.
If the input contains a single predicate, return Some(predicate).
Otherwise, return a Some(..) of a conjunction of the predicates (e.g. `Some(a AND b AND c)`).
