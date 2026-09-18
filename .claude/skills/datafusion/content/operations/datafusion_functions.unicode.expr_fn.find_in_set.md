# `datafusion_functions::unicode::expr_fn::find_in_set`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.find_in_set.json).

<a id="op-f33d9c8c9b5631f4ec98999c"></a>
## find_in_set

`function` · `datafusion_functions::unicode::expr_fn::find_in_set` · datafusion-functions 55.1.0

```rust
fn find_in_set(string: datafusion_expr::Expr, strlist: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns a value in the range of 1 to N if the string `str` is in the string list `strlist` consisting of N substrings
