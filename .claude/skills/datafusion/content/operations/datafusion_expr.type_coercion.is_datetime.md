# `datafusion_expr::type_coercion::is_datetime`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.is_datetime.json).

<a id="op-687080cc0b21efd2b3675431"></a>
## is_datetime

`function` · `datafusion_expr::type_coercion::is_datetime` · datafusion-expr 55.1.0

```rust
fn is_datetime(dt: &arrow::datatypes::DataType) -> bool
```

Source: `src/type_coercion/mod.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determine whether the given data type `dt` is a `Date` or `Timestamp`.
