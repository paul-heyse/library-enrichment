# `datafusion_physical_expr_common::sort_expr::format_physical_sort_requirement_list`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.format_physical_sort_requirement_list.json).

<a id="op-1da02af88bfc0897c38e18e1"></a>
## format_physical_sort_requirement_list

`function` · `datafusion_physical_expr_common::sort_expr::format_physical_sort_requirement_list` · datafusion-physical-expr-common 55.1.0

```rust
fn format_physical_sort_requirement_list(exprs: &[PhysicalSortRequirement]) -> impl Display + '_
```

Source: `src/sort_expr.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Writes a list of [`PhysicalSortRequirement`](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortRequirement.md#op-37d32c622d1539a2c554e300)s to a `std::fmt::Formatter`.

Example output: `[a + 1, b]`
