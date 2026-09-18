# `datafusion_physical_expr_common::utils::evaluate_expressions_to_arrays`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.utils.evaluate_expressions_to_arrays.json).

<a id="op-540bba951c99c235b314a89b"></a>
## evaluate_expressions_to_arrays

`function` · `datafusion_physical_expr_common::utils::evaluate_expressions_to_arrays` · datafusion-physical-expr-common 55.1.0

```rust
fn evaluate_expressions_to_arrays<'a>(exprs: impl IntoIterator<Item = &'a std::sync::Arc<dyn PhysicalExpr>>, batch: &arrow::record_batch::RecordBatch) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>>
```

Source: `src/utils.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Evaluates expressions against a record batch.
This will convert the resulting ColumnarValues to ArrayRefs,
duplicating any ScalarValues that may have been returned,
and validating that the returned arrays all have the same
number of rows as the input batch.
