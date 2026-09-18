# `datafusion_functions::unicode::expr_fn::instr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.instr.json).

<a id="op-286af1f28274d698efba69f6"></a>
## instr

`function` · `datafusion_functions::unicode::expr_fn::instr` · datafusion-functions 55.1.0

```rust
fn instr(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

finds the position from where the `substring` matches the `string`
