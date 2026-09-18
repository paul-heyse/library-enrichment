# `datafusion_expr::utils::find_valid_equijoin_key_pair`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.find_valid_equijoin_key_pair.json).

<a id="op-998a5a4863b0b64ce5e9c26d"></a>
## find_valid_equijoin_key_pair

`function` · `datafusion_expr::utils::find_valid_equijoin_key_pair` · datafusion-expr 55.1.0

```rust
fn find_valid_equijoin_key_pair(left_key: &Expr, right_key: &Expr, left_schema: &datafusion_common::DFSchema, right_schema: &datafusion_common::DFSchema) -> datafusion_common::Result<Option<(Expr, Expr)>>
```

Source: `src/utils.rs:1035`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Give two sides of the equijoin predicate, return a valid join key pair.
If there is no valid join key pair, return None.

A valid join means:
1. All referenced column of the left side is from the left schema, and
   all referenced column of the right side is from the right schema.
2. Or opposite. All referenced column of the left side is from the right schema,
   and the right side is from the left schema.
