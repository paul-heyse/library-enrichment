# `datafusion_expr::expr::physical_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.physical_name.json).

<a id="op-d7d5dcfe3c61906393f13fdf"></a>
## physical_name

`function` · `datafusion_expr::expr::physical_name` · datafusion-expr 55.1.0

```rust
fn physical_name(expr: &Expr) -> datafusion_common::Result<String>
```

Source: `src/expr.rs:3823`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The name of the column (field) that this `Expr` will produce in the physical plan.
The difference from [Expr::schema_name](../operations/datafusion_expr.expr.Expr.md#op-82ef8179c0d4e6c2ae471137) is that top-level columns are unqualified.
