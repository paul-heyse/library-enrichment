# `datafusion_functions_aggregate_common::utils::get_sort_options`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.utils.get_sort_options.json).

<a id="op-5fc306d598001473fd1219c4"></a>
## get_sort_options

`function` · `datafusion_functions_aggregate_common::utils::get_sort_options` · datafusion-functions-aggregate-common 55.1.0

```rust
fn get_sort_options(ordering_req: &datafusion_physical_expr_common::sort_expr::LexOrdering) -> Vec<arrow::compute::SortOptions>
```

Source: `src/utils.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Selects the sort option attribute from all the given `PhysicalSortExpr`s.
