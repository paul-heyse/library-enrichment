# `datafusion_functions::unicode::expr_fn::substr_index`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.substr_index.json).

<a id="op-ab5a67b4657826acf8b979bf"></a>
## substr_index

`function` · `datafusion_functions::unicode::expr_fn::substr_index` · datafusion-functions 55.1.0

```rust
fn substr_index(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, count: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the substring from str before count occurrences of the delimiter
