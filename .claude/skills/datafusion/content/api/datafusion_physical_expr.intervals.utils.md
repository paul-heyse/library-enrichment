# `datafusion_physical_expr::intervals::utils`

Crate `datafusion-physical-expr` · 6 public items · structured records in [`model/datafusion_physical_expr.intervals.utils.json`](../model/datafusion_physical_expr.intervals.utils.json)

## check_support

`function` · `datafusion_physical_expr::intervals::utils::check_support`

```rust
fn check_support(expr: &std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::SchemaRef) -> bool
```

Indicates whether interval arithmetic is supported for the given expression.
Currently, we do not support all [`PhysicalExpr`]s for interval calculations.
We do not support every type of [`Operator`]s either. Over time, this check
will relax as more types of `PhysicalExpr`s and `Operator`s are supported.
Currently, [`CastExpr`], [`NegativeExpr`], [`BinaryExpr`], [`Column`] and [`Literal`] are supported.

---

## convert_duration_type_to_interval

`function` · `datafusion_physical_expr::intervals::utils::convert_duration_type_to_interval`

```rust
fn convert_duration_type_to_interval(interval: &datafusion_expr::interval_arithmetic::Interval) -> Option<datafusion_expr::interval_arithmetic::Interval>
```

Converts an [`Interval`] of `Duration`s to one of time intervals, if applicable. Otherwise, returns [`None`].

---

## convert_interval_type_to_duration

`function` · `datafusion_physical_expr::intervals::utils::convert_interval_type_to_duration`

```rust
fn convert_interval_type_to_duration(interval: &datafusion_expr::interval_arithmetic::Interval) -> Option<datafusion_expr::interval_arithmetic::Interval>
```

Converts an [`Interval`] of time intervals to one of `Duration`s, if applicable. Otherwise, returns [`None`].

---

## get_inverse_op

`function` · `datafusion_physical_expr::intervals::utils::get_inverse_op`

```rust
fn get_inverse_op(op: datafusion_expr::Operator) -> datafusion_common::Result<datafusion_expr::Operator>
```

---

## is_datatype_supported

`function` · `datafusion_physical_expr::intervals::utils::is_datatype_supported`

```rust
fn is_datatype_supported(data_type: &arrow::datatypes::DataType) -> bool
```

Indicates whether interval arithmetic is supported for the given data type.

---

## is_operator_supported

`function` · `datafusion_physical_expr::intervals::utils::is_operator_supported`

```rust
fn is_operator_supported(op: &datafusion_expr::Operator) -> bool
```

Indicates whether interval arithmetic is supported for the given operator.

---
