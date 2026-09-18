# `datafusion_functions::utils::calculate_binary_math`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.utils.calculate_binary_math.json).

<a id="op-df13b6d3951d43c9da950d5a"></a>
## calculate_binary_math

`function` · `datafusion_functions::utils::calculate_binary_math` · datafusion-functions 55.1.0

```rust
fn calculate_binary_math<L, R, O, F>(left: &dyn Array, right: &datafusion_expr::ColumnarValue, fun: F) -> datafusion_common::Result<std::sync::Arc<arrow::array::PrimitiveArray<O>>> where L: ArrowPrimitiveType, R: ArrowPrimitiveType, O: ArrowPrimitiveType, F: Fn(L::Native, R::Native) -> datafusion_common::Result<O::Native, arrow::error::ArrowError>, R::Native: TryFrom<datafusion_common::ScalarValue>
```

Source: `src/utils.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Computes a binary math function for input arrays using a specified function.
Generic types:
- `L`: Left array primitive type
- `R`: Right array primitive type
- `O`: Output array primitive type
- `F`: Functor computing `fun(l: L, r: R) -> Result<OutputType>`
