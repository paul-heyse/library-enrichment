# `datafusion_physical_expr_common::physical_expr::is_volatile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.is_volatile.json).

<a id="op-87a14151dca85e57889ff9c0"></a>
## is_volatile

`function` · `datafusion_physical_expr_common::physical_expr::is_volatile` · datafusion-physical-expr-common 55.1.0

```rust
fn is_volatile(expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Source: `src/physical_expr.rs:1016`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns true if the expression is volatile, i.e. whether it can return different
results when evaluated multiple times with the same input.

For example the function call `RANDOM()` is volatile as each call will
return a different value.

This method recursively checks if any sub-expression is volatile, for example
`1 + RANDOM()` will return `true`.
