# `datafusion_functions::utils::calculate_binary_decimal_math`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.utils.calculate_binary_decimal_math.json).

<a id="op-eb52ed3dfdce17dc5e667916"></a>
## calculate_binary_decimal_math

`function` · `datafusion_functions::utils::calculate_binary_decimal_math` · datafusion-functions 55.1.0

```rust
fn calculate_binary_decimal_math<L, R, O, F>(left: &dyn Array, right: &datafusion_expr::ColumnarValue, fun: F, precision: u8, scale: i8) -> datafusion_common::Result<std::sync::Arc<arrow::array::PrimitiveArray<O>>> where L: DecimalType, R: ArrowPrimitiveType, O: DecimalType, F: Fn(L::Native, R::Native) -> datafusion_common::Result<O::Native, arrow::error::ArrowError>, R::Native: TryFrom<datafusion_common::ScalarValue>
```

Source: `src/utils.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Computes a binary math function for input arrays using a specified function
and applies rescaling to given precision and scale.
Generic types:
- `L`: Left array decimal type
- `R`: Right array primitive type
- `O`: Output array decimal type
- `F`: Functor computing `fun(l: L, r: R) -> Result<OutputType>`
