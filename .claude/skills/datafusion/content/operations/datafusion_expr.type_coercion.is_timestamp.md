# `datafusion_expr::type_coercion::is_timestamp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.is_timestamp.json).

<a id="op-223e718ffb796135807a5dd2"></a>
## is_timestamp

`function` · `datafusion_expr::type_coercion::is_timestamp` · datafusion-expr 55.1.0

```rust
fn is_timestamp(dt: &arrow::datatypes::DataType) -> bool
```

Source: `src/type_coercion/mod.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determine whether the given data type `dt` is a `Timestamp`.
