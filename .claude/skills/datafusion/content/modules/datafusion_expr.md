# `datafusion_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.json).

<a id="op-5006abb37765a7bae842342b"></a>
## datafusion_expr

`module` · `datafusion_expr` · datafusion-expr 55.1.0

```rust
mod datafusion_expr
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

[DataFusion](https://github.com/apache/datafusion)
is an extensible query execution framework that uses
[Apache Arrow](https://arrow.apache.org) as its in-memory format.

This crate is a submodule of DataFusion that provides types representing
logical query plans ([LogicalPlan](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)) and logical expressions ([Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)) as well as utilities for
working with these types.

The [expr_fn](../modules/datafusion_expr.expr_fn.md#op-e7001738a261bab88eecdffd) module contains functions for creating expressions.
