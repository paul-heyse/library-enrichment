# `datafusion_physical_expr::physical_expr::physical_exprs_bag_equal`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.physical_expr.physical_exprs_bag_equal.json).

<a id="op-ce6ba40cb2d9ef8a4a898b3c"></a>
## physical_exprs_bag_equal

`function` · `datafusion_physical_expr::physical_expr::physical_exprs_bag_equal` · datafusion-physical-expr 55.1.0

```rust
fn physical_exprs_bag_equal(lhs: &[std::sync::Arc<dyn PhysicalExpr>], rhs: &[std::sync::Arc<dyn PhysicalExpr>]) -> bool
```

Source: `src/physical_expr.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Checks whether the given physical expression slices are equal in the sense
of bags (multi-sets), disregarding their orderings.
