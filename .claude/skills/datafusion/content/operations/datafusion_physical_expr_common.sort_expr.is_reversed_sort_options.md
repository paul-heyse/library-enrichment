# `datafusion_physical_expr_common::sort_expr::is_reversed_sort_options`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.is_reversed_sort_options.json).

<a id="op-bffd90972449ca9ddf6a6922"></a>
## is_reversed_sort_options

`function` · `datafusion_physical_expr_common::sort_expr::is_reversed_sort_options` · datafusion-physical-expr-common 55.1.0

```rust
fn is_reversed_sort_options(lhs: &arrow::compute::kernels::sort::SortOptions, rhs: &arrow::compute::kernels::sort::SortOptions) -> bool
```

Source: `src/sort_expr.rs:609`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Check if two SortOptions represent reversed orderings.

Returns `true` if both `descending` and `nulls_first` are opposite.

# Example
```
use arrow::compute::SortOptions;
# use datafusion_physical_expr_common::sort_expr::is_reversed_sort_options;

let asc_nulls_last = SortOptions {
    descending: false,
    nulls_first: false,
};
let desc_nulls_first = SortOptions {
    descending: true,
    nulls_first: true,
};

assert!(is_reversed_sort_options(&asc_nulls_last, &desc_nulls_first));
assert!(is_reversed_sort_options(&desc_nulls_first, &asc_nulls_last));
```
