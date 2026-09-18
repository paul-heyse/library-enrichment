# `datafusion_functions_aggregate::string_agg::string_agg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.string_agg.string_agg.json).

<a id="op-eb778799f2432aebcd542591"></a>
## string_agg

`function` · `datafusion_functions_aggregate::string_agg::string_agg` · datafusion-functions-aggregate 55.1.0

```rust
fn string_agg(expr: datafusion_expr::Expr, delimiter: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/string_agg.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Concatenates the values of string expressions and places separator values between them
