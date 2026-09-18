# `datafusion_common::utils::find_bisect_point`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.find_bisect_point.json).

<a id="op-a466a06c337ffd513585861c"></a>
## find_bisect_point

`function` · `datafusion_common::utils::find_bisect_point` · datafusion-common 55.1.0

```rust
fn find_bisect_point<F>(item_columns: &[arrow::array::ArrayRef], target: &[ScalarValue], compare_fn: F, low: usize, high: usize) -> Result<usize> where F: Fn(&[ScalarValue], &[ScalarValue]) -> Result<bool>
```

Source: `src/utils/mod.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function searches for a tuple of given values (`target`) among a slice of
the given rows (`item_columns`) using the bisection algorithm. The slice starts
at the index `low` and ends at the index `high`. The boolean-valued function
`compare_fn` specifies whether we bisect on the left (by returning `false`),
or on the right (by returning `true`) when we compare the target value with
the current value as we iteratively bisect the input.
