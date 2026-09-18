# `datafusion_physical_expr::equivalence`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.equivalence.json`](../model/datafusion_physical_expr.equivalence.json)

## convert_to_orderings

`function` · `datafusion_physical_expr::equivalence::convert_to_orderings`

```rust
fn convert_to_orderings<T: Borrow<std::sync::Arc<dyn PhysicalExpr>>>(args: &[Vec<(T, arrow::compute::SortOptions)>]) -> Vec<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.equivalence.convert_to_orderings.md).


---

## convert_to_sort_exprs

`function` · `datafusion_physical_expr::equivalence::convert_to_sort_exprs`

```rust
fn convert_to_sort_exprs<T: Borrow<std::sync::Arc<dyn PhysicalExpr>>>(args: &[(T, arrow::compute::SortOptions)]) -> Vec<datafusion_physical_expr_common::sort_expr::PhysicalSortExpr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.equivalence.convert_to_sort_exprs.md).


---
