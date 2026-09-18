# `datafusion_common::functional_dependencies::get_required_group_by_exprs_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.get_required_group_by_exprs_indices.json).

<a id="op-a46599a37ec8d38222ff661e"></a>
## get_required_group_by_exprs_indices

`function` · `datafusion_common::functional_dependencies::get_required_group_by_exprs_indices` · datafusion-common 55.1.0

```rust
fn get_required_group_by_exprs_indices(schema: &DFSchema, group_by_expr_names: &[String]) -> Option<Vec<usize>>
```

Source: `src/functional_dependencies.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns indices for the minimal subset of GROUP BY expressions that are
functionally equivalent to the original set of GROUP BY expressions.
