# `datafusion_physical_expr::simplifier::const_evaluator::simplify_const_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.simplifier.const_evaluator.simplify_const_expr.json).

<a id="op-78855871892c70018bfd832a"></a>
## simplify_const_expr

`function` · `datafusion_physical_expr::simplifier::const_evaluator::simplify_const_expr` · datafusion-physical-expr 55.1.0

```rust
fn simplify_const_expr(expr: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn PhysicalExpr>>>
```

Source: `src/simplifier/const_evaluator.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Simplify expressions that consist only of literals by evaluating them.

This function checks if all children of the given expression are literals.
If so, it evaluates the expression against a dummy RecordBatch and returns
the result as a new Literal.

# Example transformations
- `1 + 2` -> `3`
- `(1 + 2) * 3` -> `9` (with bottom-up traversal)
- `'hello' || ' world'` -> `'hello world'`
