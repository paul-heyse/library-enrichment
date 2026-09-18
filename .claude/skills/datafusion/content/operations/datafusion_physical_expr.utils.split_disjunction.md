# `datafusion_physical_expr::utils::split_disjunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.split_disjunction.json).

<a id="op-26c58f17afacaf35af224042"></a>
## split_disjunction

`function` · `datafusion_physical_expr::utils::split_disjunction` · datafusion-physical-expr 55.1.0

```rust
fn split_disjunction(predicate: &std::sync::Arc<dyn PhysicalExpr>) -> Vec<&std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/utils/mod.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Assume the predicate is in the form of DNF, split the predicate to a Vec of PhysicalExprs.

For example, split "a1 = a2 OR b1 <= b2 OR c1 != c2" into ["a1 = a2", "b1 <= b2", "c1 != c2"]
