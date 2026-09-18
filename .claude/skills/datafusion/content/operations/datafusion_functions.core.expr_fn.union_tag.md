# `datafusion_functions::core::expr_fn::union_tag`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.union_tag.json).

<a id="op-0e1f979bbb1d50f3dfa33867"></a>
## union_tag

`function` · `datafusion_functions::core::expr_fn::union_tag` · datafusion-functions 55.1.0

```rust
fn union_tag(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the name of the currently selected field in the union
