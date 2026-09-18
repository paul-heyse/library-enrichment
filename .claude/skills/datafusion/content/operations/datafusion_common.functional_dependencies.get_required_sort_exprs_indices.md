# `datafusion_common::functional_dependencies::get_required_sort_exprs_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.get_required_sort_exprs_indices.json).

<a id="op-c2721a5e69d1f8bfe3ba1536"></a>
## get_required_sort_exprs_indices

`function` · `datafusion_common::functional_dependencies::get_required_sort_exprs_indices` · datafusion-common 55.1.0

```rust
fn get_required_sort_exprs_indices(schema: &DFSchema, sort_expr_names: &[String]) -> Vec<usize>
```

Source: `src/functional_dependencies.rs:597`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns indices for the minimal subset of ORDER BY expressions that are
functionally equivalent to the original set of ORDER BY expressions.
