# `datafusion_physical_expr_common::physical_expr::format_physical_expr_list`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.format_physical_expr_list.json).

<a id="op-aa2d11b82de2716be1b8f217"></a>
## format_physical_expr_list

`function` · `datafusion_physical_expr_common::physical_expr::format_physical_expr_list` · datafusion-physical-expr-common 55.1.0

```rust
fn format_physical_expr_list<T>(exprs: T) -> impl Display where T: IntoIterator, T::Item: Display, T::IntoIter: Clone
```

Source: `src/physical_expr.rs:847`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns [`Display`] able a list of [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7)

Example output: `[a + 1, b]`

Unresolved upstream links (retained, not inferred): ``Display``.
