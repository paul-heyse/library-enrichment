# `datafusion_functions::unicode::expr_fn::initcap`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.initcap.json).

<a id="op-aba06e832bf328ba7a86a175"></a>
## initcap

`function` · `datafusion_functions::unicode::expr_fn::initcap` · datafusion-functions 55.1.0

```rust
fn initcap(string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

converts the first letter of each word in `string` in uppercase and the remaining characters in lowercase
