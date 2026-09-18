# `datafusion_functions_aggregate_common::utils::get_accum_scalar_values_as_arrays`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.utils.get_accum_scalar_values_as_arrays.json).

<a id="op-3c20fe84323c7327e7e5851c"></a>
## get_accum_scalar_values_as_arrays

`function` · `datafusion_functions_aggregate_common::utils::get_accum_scalar_values_as_arrays` · datafusion-functions-aggregate-common 55.1.0

```rust
fn get_accum_scalar_values_as_arrays(accum: &mut dyn Accumulator) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>>
```

Source: `src/utils.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Convert scalar values from an accumulator into arrays.
