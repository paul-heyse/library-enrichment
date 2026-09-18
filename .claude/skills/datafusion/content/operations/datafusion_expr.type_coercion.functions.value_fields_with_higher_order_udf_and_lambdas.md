# `datafusion_expr::type_coercion::functions::value_fields_with_higher_order_udf_and_lambdas`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.functions.value_fields_with_higher_order_udf_and_lambdas.json).

<a id="op-a3e5da520afe747ec3815f7d"></a>
## value_fields_with_higher_order_udf_and_lambdas

`function` · `datafusion_expr::type_coercion::functions::value_fields_with_higher_order_udf_and_lambdas` · datafusion-expr 55.1.0

```rust
fn value_fields_with_higher_order_udf_and_lambdas(current_fields: &[ValueOrLambda<arrow::datatypes::FieldRef, arrow::datatypes::FieldRef>], func: &HigherOrderUDF) -> datafusion_common::Result<Vec<ValueOrLambda<arrow::datatypes::FieldRef, arrow::datatypes::FieldRef>>>
```

Source: `src/type_coercion/functions.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Performs type coercion for higher order function arguments,
including those defined by [crate::HigherOrderUDFImpl::coerce_values_for_lambdas](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-7b2661f5491cf7d850de20ea),
if it returns `Some(...)` instead of the default `None`. Note that
compared to [value_fields_with_higher_order_udf](../operations/datafusion_expr.type_coercion.functions.value_fields_with_higher_order_udf.md#op-14f057cb00fd572fce3bdba0), this function requires
the [ValueOrLambda::Lambda](../operations/datafusion_expr.higher_order_function.ValueOrLambda.md#op-d68d7d1b330c93c8815d9782) variant to contain the output field of the lambda.

For value arguments, returns the field to which each
argument must be coerced to match `signature`.
For lambda arguments, returns a clone of the output field

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.
