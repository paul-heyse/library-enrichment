# `datafusion_expr::type_coercion::functions::value_fields_with_higher_order_udf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.functions.value_fields_with_higher_order_udf.json).

<a id="op-14f057cb00fd572fce3bdba0"></a>
## value_fields_with_higher_order_udf

`function` · `datafusion_expr::type_coercion::functions::value_fields_with_higher_order_udf` · datafusion-expr 55.1.0

```rust
fn value_fields_with_higher_order_udf<L: Clone>(current_fields: &[ValueOrLambda<arrow::datatypes::FieldRef, L>], func: &HigherOrderUDF) -> datafusion_common::Result<Vec<ValueOrLambda<arrow::datatypes::FieldRef, L>>>
```

Source: `src/type_coercion/functions.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Performs type coercion for higher order function arguments.

For value arguments, returns the field to which each
argument must be coerced to match `signature`.
For lambda arguments, returns a clone of the associated data

Note this does not invokes [crate::HigherOrderUDFImpl::coerce_values_for_lambdas](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-7b2661f5491cf7d850de20ea).
If that's required, use [value_fields_with_higher_order_udf_and_lambdas](../operations/datafusion_expr.type_coercion.functions.value_fields_with_higher_order_udf_and_lambdas.md#op-a3e5da520afe747ec3815f7d)
instead

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.
