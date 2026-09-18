# `datafusion_physical_expr::utils::split_conjunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.split_conjunction.json).

<a id="op-c35eebd293c849ebe30f357c"></a>
## split_conjunction

`function` · `datafusion_physical_expr::utils::split_conjunction` · datafusion-physical-expr 55.1.0

```rust
fn split_conjunction(predicate: &std::sync::Arc<dyn PhysicalExpr>) -> Vec<&std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/utils/mod.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Assume the predicate is in the form of CNF, split the predicate to a Vec of PhysicalExprs.

For example, split "a1 = a2 AND b1 <= b2 AND c1 != c2" into ["a1 = a2", "b1 <= b2", "c1 != c2"]
