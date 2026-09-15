# `datafusion_physical_expr::simplifier::const_evaluator`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.simplifier.const_evaluator.json`](../model/datafusion_physical_expr.simplifier.const_evaluator.json)

## has_column_references

`function` · `datafusion_physical_expr::simplifier::const_evaluator::has_column_references`

> **Deprecated** — since 53.0.0: This function isn't used internally and is trivial to implement, therefore it will be removed in a future release.

```rust
fn has_column_references(expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Check if this expression has any column references.

---

## simplify_const_expr

`function` · `datafusion_physical_expr::simplifier::const_evaluator::simplify_const_expr`

> **Deprecated** — since 53.0.0: This function will be removed in a future release in favor of a private implementation that depends on other implementation details. Please open an issue if you have a use case for keeping it.

```rust
fn simplify_const_expr(expr: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn PhysicalExpr>>>
```

Simplify expressions that consist only of literals by evaluating them.

This function checks if all children of the given expression are literals.
If so, it evaluates the expression against a dummy RecordBatch and returns
the result as a new Literal.

# Example transformations
- `1 + 2` -> `3`
- `(1 + 2) * 3` -> `9` (with bottom-up traversal)
- `'hello' || ' world'` -> `'hello world'`

---
