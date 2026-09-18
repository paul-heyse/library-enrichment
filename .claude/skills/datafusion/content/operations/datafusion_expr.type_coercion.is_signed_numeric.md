# `datafusion_expr::type_coercion::is_signed_numeric`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.is_signed_numeric.json).

<a id="op-5bb971462bd153b1a14c9069"></a>
## is_signed_numeric

`function` · `datafusion_expr::type_coercion::is_signed_numeric` · datafusion-expr 55.1.0

```rust
fn is_signed_numeric(dt: &arrow::datatypes::DataType) -> bool
```

Source: `src/type_coercion/mod.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determine whether the given data type `dt` represents signed numeric values.
