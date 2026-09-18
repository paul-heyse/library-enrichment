# `datafusion_physical_expr_common::datum::apply`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.datum.apply.json).

<a id="op-c18e9737b5da45bd1178fd12"></a>
## apply

`function` · `datafusion_physical_expr_common::datum::apply` · datafusion-physical-expr-common 55.1.0

```rust
fn apply(lhs: &datafusion_expr_common::columnar_value::ColumnarValue, rhs: &datafusion_expr_common::columnar_value::ColumnarValue, f: impl Fn(&dyn Datum, &dyn Datum) -> datafusion_common::Result<arrow::array::ArrayRef, arrow::error::ArrowError>) -> datafusion_common::Result<datafusion_expr_common::columnar_value::ColumnarValue>
```

Source: `src/datum.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Applies a binary [`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f) kernel `f` to `lhs` and `rhs`

This maps arrow-rs' [`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f) kernels to DataFusion's [`ColumnarValue`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d5fec5b0e8fcb4446aea8c7f) abstraction
