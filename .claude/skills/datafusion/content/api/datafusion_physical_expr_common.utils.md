# `datafusion_physical_expr_common::utils`

Crate `datafusion-physical-expr-common` · 4 public items · structured records in [`model/datafusion_physical_expr_common.utils.json`](../model/datafusion_physical_expr_common.utils.json)

## evaluate_expressions_to_arrays

`function` · `datafusion_physical_expr_common::utils::evaluate_expressions_to_arrays`

```rust
fn evaluate_expressions_to_arrays<'a>(exprs: impl IntoIterator<Item = &'a std::sync::Arc<dyn PhysicalExpr>>, batch: &arrow::record_batch::RecordBatch) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>>
```

Evaluates expressions against a record batch.
This will convert the resulting ColumnarValues to ArrayRefs,
duplicating any ScalarValues that may have been returned,
and validating that the returned arrays all have the same
number of rows as the input batch.

---

## evaluate_expressions_to_arrays_with_metrics

`function` · `datafusion_physical_expr_common::utils::evaluate_expressions_to_arrays_with_metrics`

```rust
fn evaluate_expressions_to_arrays_with_metrics<'a>(exprs: impl IntoIterator<Item = &'a std::sync::Arc<dyn PhysicalExpr>>, batch: &arrow::record_batch::RecordBatch, metrics: Option<&metrics::ExpressionEvaluatorMetrics>) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>>
```

Same as [`evaluate_expressions_to_arrays`] but records optional per-expression metrics.

For metrics tracking, see [`ExpressionEvaluatorMetrics`] for details.

---

## scatter

`function` · `datafusion_physical_expr_common::utils::scatter`

```rust
fn scatter(mask: &arrow::array::BooleanArray, truthy: &dyn Array) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Scatter `truthy` array by boolean mask. When the mask evaluates `true`, next values of `truthy`
are taken, when the mask evaluates `false` values null values are filled.

# Arguments
* `mask` - Boolean values used to determine where to put the `truthy` values
* `truthy` - All values of this array are to scatter according to `mask` into final result.

---

## ExprPropertiesNode

`type_alias` · `datafusion_physical_expr_common::utils::ExprPropertiesNode`

```rust
type ExprPropertiesNode = tree_node::ExprContext<datafusion_expr_common::sort_properties::ExprProperties>
```

Represents a [`PhysicalExpr`] node with associated properties (order and
range) in a context where properties are tracked.

---
