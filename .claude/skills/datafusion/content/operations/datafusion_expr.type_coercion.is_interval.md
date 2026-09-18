# `datafusion_expr::type_coercion::is_interval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.is_interval.json).

<a id="op-a971d0226cee469b22fcf676"></a>
## is_interval

`function` · `datafusion_expr::type_coercion::is_interval` · datafusion-expr 55.1.0

```rust
fn is_interval(dt: &arrow::datatypes::DataType) -> bool
```

Source: `src/type_coercion/mod.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determine whether the given data type 'dt' is a `Interval`.
