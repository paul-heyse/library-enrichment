# `datafusion_physical_expr::equivalence::convert_to_sort_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.convert_to_sort_exprs.json).

<a id="op-50efbd56fc2802f7e40fe3c2"></a>
## convert_to_sort_exprs

`function` · `datafusion_physical_expr::equivalence::convert_to_sort_exprs` · datafusion-physical-expr 55.1.0

```rust
fn convert_to_sort_exprs<T: Borrow<std::sync::Arc<dyn PhysicalExpr>>>(args: &[(T, arrow::compute::SortOptions)]) -> Vec<datafusion_physical_expr_common::sort_expr::PhysicalSortExpr>
```

Source: `src/equivalence/mod.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
