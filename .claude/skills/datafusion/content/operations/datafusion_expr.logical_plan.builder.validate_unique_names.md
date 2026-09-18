# `datafusion_expr::logical_plan::builder::validate_unique_names`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.validate_unique_names.json).

<a id="op-c79a362a7df3044239ea84ad"></a>
## validate_unique_names

`function` · `datafusion_expr::logical_plan::builder::validate_unique_names` · datafusion-expr 55.1.0

```rust
fn validate_unique_names<'a>(node_name: &str, expressions: impl IntoIterator<Item = &'a Expr>) -> datafusion_common::Result<()>
```

Source: `src/logical_plan/builder.rs:1885`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Errors if one or more expressions have equal names.
