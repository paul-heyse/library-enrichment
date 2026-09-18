# `datafusion_physical_expr_common::datum::compare_with_eq`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.datum.compare_with_eq.json).

<a id="op-5e23fbf13dd5d612460f2358"></a>
## compare_with_eq

`function` · `datafusion_physical_expr_common::datum::compare_with_eq` · datafusion-physical-expr-common 55.1.0

```rust
fn compare_with_eq(lhs: &dyn Datum, rhs: &dyn Datum, is_nested: bool) -> datafusion_common::Result<arrow::array::BooleanArray>
```

Source: `src/datum.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Compare with eq with either nested or non-nested
