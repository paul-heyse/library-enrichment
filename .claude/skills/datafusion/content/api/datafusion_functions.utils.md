# `datafusion_functions::utils`

Crate `datafusion-functions` · 7 public items · structured records in [`model/datafusion_functions.utils.json`](../model/datafusion_functions.utils.json)

## calculate_binary_decimal_math

`function` · `datafusion_functions::utils::calculate_binary_decimal_math`

> **Deprecated** — since 55.0.0: Use `calculate_binary_decimal_math_cast` instead

```rust
fn calculate_binary_decimal_math<L, R, O, F>(left: &dyn Array, right: &datafusion_expr::ColumnarValue, fun: F, precision: u8, scale: i8) -> datafusion_common::Result<std::sync::Arc<arrow::array::PrimitiveArray<O>>> where L: DecimalType, R: ArrowPrimitiveType, O: DecimalType, F: Fn(L::Native, R::Native) -> datafusion_common::Result<O::Native, arrow::error::ArrowError>, R::Native: TryFrom<datafusion_common::ScalarValue>
```

Computes a binary math function for input arrays using a specified function
and applies rescaling to given precision and scale.
Generic types:
- `L`: Left array decimal type
- `R`: Right array primitive type
- `O`: Output array decimal type
- `F`: Functor computing `fun(l: L, r: R) -> Result<OutputType>`

---

## calculate_binary_decimal_math_cast

`function` · `datafusion_functions::utils::calculate_binary_decimal_math_cast`

```rust
fn calculate_binary_decimal_math_cast<L, R, O, F>(left: &dyn Array, right: &datafusion_expr::ColumnarValue, fun: F, precision: u8, scale: i8, cast_target: &arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<arrow::array::PrimitiveArray<O>>> where L: DecimalType, R: ArrowPrimitiveType, O: DecimalType, F: Fn(L::Native, R::Native) -> datafusion_common::Result<O::Native, arrow::error::ArrowError>, R::Native: TryFrom<datafusion_common::ScalarValue>
```

Computes a binary math function for input arrays using a specified function
and applies rescaling to given precision and scale.

It casts the right operand to `cast_target` instead of the default `R::DATA_TYPE` to preserve
the right operand scale.

# Type Parameters
- `L`: Left array decimal type
- `R`: Right array primitive type
- `O`: Output array decimal type
- `F`: Functor computing `fun(l: L, r: R) -> Result<OutputType>`
# Arguments
- `left`: Left input array
- `right`: Right input array or scalar value
- `fun`: Function of type `F`
- `precision`: Precision to apply to output decimal array
- `scale`: Scale to apply to output decimal array
- `cast_target`: Data type to cast right operand to before applying function

---

## calculate_binary_math

`function` · `datafusion_functions::utils::calculate_binary_math`

```rust
fn calculate_binary_math<L, R, O, F>(left: &dyn Array, right: &datafusion_expr::ColumnarValue, fun: F) -> datafusion_common::Result<std::sync::Arc<arrow::array::PrimitiveArray<O>>> where L: ArrowPrimitiveType, R: ArrowPrimitiveType, O: ArrowPrimitiveType, F: Fn(L::Native, R::Native) -> datafusion_common::Result<O::Native, arrow::error::ArrowError>, R::Native: TryFrom<datafusion_common::ScalarValue>
```

Computes a binary math function for input arrays using a specified function.
Generic types:
- `L`: Left array primitive type
- `R`: Right array primitive type
- `O`: Output array primitive type
- `F`: Functor computing `fun(l: L, r: R) -> Result<OutputType>`

---

## decimal128_to_i128

`function` · `datafusion_functions::utils::decimal128_to_i128`

```rust
fn decimal128_to_i128(value: i128, scale: i8) -> datafusion_common::Result<i128, arrow::error::ArrowError>
```

Converts Decimal128 components (value and scale) to an unscaled i128

---

## decimal32_to_i32

`function` · `datafusion_functions::utils::decimal32_to_i32`

```rust
fn decimal32_to_i32(value: i32, scale: i8) -> datafusion_common::Result<i32, arrow::error::ArrowError>
```

---

## decimal64_to_i64

`function` · `datafusion_functions::utils::decimal64_to_i64`

```rust
fn decimal64_to_i64(value: i64, scale: i8) -> datafusion_common::Result<i64, arrow::error::ArrowError>
```

---

## make_scalar_function

`function` · `datafusion_functions::utils::make_scalar_function`

```rust
fn make_scalar_function<F>(inner: F, hints: Vec<datafusion_expr::function::Hint>) -> impl Fn(&[datafusion_expr::ColumnarValue]) -> datafusion_common::Result<datafusion_expr::ColumnarValue> where F: Fn(&[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Creates a scalar function implementation for the given function.
* `inner` - the function to be executed
* `hints` - hints to be used when expanding scalars to arrays

---
