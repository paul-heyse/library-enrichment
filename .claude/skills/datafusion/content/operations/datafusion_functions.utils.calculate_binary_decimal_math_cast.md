# `datafusion_functions::utils::calculate_binary_decimal_math_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.utils.calculate_binary_decimal_math_cast.json).

<a id="op-bb534df7355ccf0b91929278"></a>
## calculate_binary_decimal_math_cast

`function` · `datafusion_functions::utils::calculate_binary_decimal_math_cast` · datafusion-functions 55.1.0

```rust
fn calculate_binary_decimal_math_cast<L, R, O, F>(left: &dyn Array, right: &datafusion_expr::ColumnarValue, fun: F, precision: u8, scale: i8, cast_target: &arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<arrow::array::PrimitiveArray<O>>> where L: DecimalType, R: ArrowPrimitiveType, O: DecimalType, F: Fn(L::Native, R::Native) -> datafusion_common::Result<O::Native, arrow::error::ArrowError>, R::Native: TryFrom<datafusion_common::ScalarValue>
```

Source: `src/utils.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

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
