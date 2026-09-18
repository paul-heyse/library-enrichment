# `datafusion_functions::utils::make_scalar_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.utils.make_scalar_function.json).

<a id="op-df97b97bc153bce20323c918"></a>
## make_scalar_function

`function` · `datafusion_functions::utils::make_scalar_function` · datafusion-functions 55.1.0

```rust
fn make_scalar_function<F>(inner: F, hints: Vec<datafusion_expr::function::Hint>) -> impl Fn(&[datafusion_expr::ColumnarValue]) -> datafusion_common::Result<datafusion_expr::ColumnarValue> where F: Fn(&[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/utils.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Creates a scalar function implementation for the given function.
* `inner` - the function to be executed
* `hints` - hints to be used when expanding scalars to arrays
