# `datafusion_expr::type_coercion::functions`

Crate `datafusion-expr` · 9 public items · structured records in [`model/datafusion_expr.type_coercion.functions.json`](../model/datafusion_expr.type_coercion.functions.json)

## can_coerce_from

`function` · `datafusion_expr::type_coercion::functions::can_coerce_from`

> **Deprecated** — since 53.0.0: Unused internal function

```rust
fn can_coerce_from(type_into: &arrow::datatypes::DataType, type_from: &arrow::datatypes::DataType) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.can_coerce_from.md).


Return true if a value of type `type_from` can be coerced
(losslessly converted) into a value of `type_to`

See the module level documentation for more detail on coercion.

---

## data_types

`function` · `datafusion_expr::type_coercion::functions::data_types`

> **Deprecated** — since 52.0.0: use fields_with_udf

```rust
fn data_types(function_name: impl AsRef<str>, current_types: &[arrow::datatypes::DataType], signature: &Signature) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.data_types.md).


Performs type coercion for function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

---

## data_types_with_scalar_udf

`function` · `datafusion_expr::type_coercion::functions::data_types_with_scalar_udf`

> **Deprecated** — since 52.0.0: use fields_with_udf

```rust
fn data_types_with_scalar_udf(current_types: &[arrow::datatypes::DataType], func: &ScalarUDF) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.data_types_with_scalar_udf.md).


Performs type coercion for scalar function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

---

## fields_with_aggregate_udf

`function` · `datafusion_expr::type_coercion::functions::fields_with_aggregate_udf`

> **Deprecated** — since 52.0.0: use fields_with_udf

```rust
fn fields_with_aggregate_udf(current_fields: &[arrow::datatypes::FieldRef], func: &AggregateUDF) -> datafusion_common::Result<Vec<arrow::datatypes::FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.fields_with_aggregate_udf.md).


Performs type coercion for aggregate function arguments.

Returns the fields to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

---

## fields_with_udf

`function` · `datafusion_expr::type_coercion::functions::fields_with_udf`

```rust
fn fields_with_udf<F: UDFCoercionExt>(current_fields: &[arrow::datatypes::FieldRef], func: &F) -> datafusion_common::Result<Vec<arrow::datatypes::FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.fields_with_udf.md).


Performs type coercion for UDF arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

---

## fields_with_window_udf

`function` · `datafusion_expr::type_coercion::functions::fields_with_window_udf`

> **Deprecated** — since 52.0.0: use fields_with_udf

```rust
fn fields_with_window_udf(current_fields: &[arrow::datatypes::FieldRef], func: &WindowUDF) -> datafusion_common::Result<Vec<arrow::datatypes::FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.fields_with_window_udf.md).


Performs type coercion for window function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

---

## value_fields_with_higher_order_udf

`function` · `datafusion_expr::type_coercion::functions::value_fields_with_higher_order_udf`

```rust
fn value_fields_with_higher_order_udf<L: Clone>(current_fields: &[ValueOrLambda<arrow::datatypes::FieldRef, L>], func: &HigherOrderUDF) -> datafusion_common::Result<Vec<ValueOrLambda<arrow::datatypes::FieldRef, L>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.value_fields_with_higher_order_udf.md).


Performs type coercion for higher order function arguments.

For value arguments, returns the field to which each
argument must be coerced to match `signature`.
For lambda arguments, returns a clone of the associated data

Note this does not invokes [crate::HigherOrderUDFImpl::coerce_values_for_lambdas].
If that's required, use [value_fields_with_higher_order_udf_and_lambdas]
instead

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

---

## value_fields_with_higher_order_udf_and_lambdas

`function` · `datafusion_expr::type_coercion::functions::value_fields_with_higher_order_udf_and_lambdas`

```rust
fn value_fields_with_higher_order_udf_and_lambdas(current_fields: &[ValueOrLambda<arrow::datatypes::FieldRef, arrow::datatypes::FieldRef>], func: &HigherOrderUDF) -> datafusion_common::Result<Vec<ValueOrLambda<arrow::datatypes::FieldRef, arrow::datatypes::FieldRef>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.value_fields_with_higher_order_udf_and_lambdas.md).


Performs type coercion for higher order function arguments,
including those defined by [crate::HigherOrderUDFImpl::coerce_values_for_lambdas],
if it returns `Some(...)` instead of the default `None`. Note that
compared to [value_fields_with_higher_order_udf], this function requires
the [ValueOrLambda::Lambda] variant to contain the output field of the lambda.

For value arguments, returns the field to which each
argument must be coerced to match `signature`.
For lambda arguments, returns a clone of the output field

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

---

## UDFCoercionExt

`trait` · `datafusion_expr::type_coercion::functions::UDFCoercionExt`

```rust
trait UDFCoercionExt
```

**Implementors** (3)

- `datafusion_expr::udaf::AggregateUDF`
- `datafusion_expr::udf::ScalarUDF`
- `datafusion_expr::udwf::WindowUDF`

**Methods** (3)

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn name(&self) -> &str
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.type_coercion.functions.UDFCoercionExt.md).


Extension trait to unify common functionality between [`ScalarUDF`], [`AggregateUDF`]
and [`WindowUDF`] for use by signature coercion functions.

---
