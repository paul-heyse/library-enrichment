# `datafusion_expr::type_coercion::functions::can_coerce_from`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.functions.can_coerce_from.json).

<a id="op-69fb0bdd3f0e0aa1666ee6b8"></a>
## can_coerce_from

`function` · `datafusion_expr::type_coercion::functions::can_coerce_from` · datafusion-expr 55.1.0

```rust
fn can_coerce_from(type_into: &arrow::datatypes::DataType, type_from: &arrow::datatypes::DataType) -> bool
```

Source: `src/type_coercion/functions.rs:1126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return true if a value of type `type_from` can be coerced
(losslessly converted) into a value of `type_to`

See the module level documentation for more detail on coercion.
