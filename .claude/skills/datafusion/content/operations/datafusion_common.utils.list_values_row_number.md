# `datafusion_common::utils::list_values_row_number`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.list_values_row_number.json).

<a id="op-643f8fbb6b8791b22d6e6e7e"></a>
## list_values_row_number

`function` · `datafusion_common::utils::list_values_row_number` · datafusion-common 55.1.0

```rust
fn list_values_row_number(array: &dyn Array) -> Result<arrow::array::ArrayRef>
```

Source: `src/utils/mod.rs:1320`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If `array` is a list or a map, returns a new array of the same length as it's inner values
where each value is the 1-based index of the sublist it's contained. Example:

`[[1], [2, 3], [4, 5, 6]] =>  [1, 2, 2, 3, 3, 3]`

Otherwise returns an error
