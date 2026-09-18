# `datafusion_physical_expr::simplifier::const_evaluator::has_column_references`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.simplifier.const_evaluator.has_column_references.json).

<a id="op-669b001e2b9d877e6790be65"></a>
## has_column_references

`function` · `datafusion_physical_expr::simplifier::const_evaluator::has_column_references` · datafusion-physical-expr 55.1.0

```rust
fn has_column_references(expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Source: `src/simplifier/const_evaluator.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Check if this expression has any column references.
