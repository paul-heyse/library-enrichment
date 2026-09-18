# `datafusion_physical_expr_common::sort_expr::options_compatible`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.options_compatible.json).

<a id="op-b72ca8ec43c3e9b99e47fdd0"></a>
## options_compatible

`function` · `datafusion_physical_expr_common::sort_expr::options_compatible` · datafusion-physical-expr-common 55.1.0

```rust
fn options_compatible(options_lhs: &arrow::compute::kernels::sort::SortOptions, options_rhs: &arrow::compute::kernels::sort::SortOptions, nullable: bool) -> bool
```

Source: `src/sort_expr.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns whether the given two [`SortOptions`](../operations/arrow_schema.SortOptions.md#op-78d98c3e0c6da432658d0949) are compatible. Here,
compatibility means that they are either exactly equal, or they differ only
in whether NULL values come in first/last, which is immaterial because the
column in question is not nullable (specified by the `nullable` parameter).
