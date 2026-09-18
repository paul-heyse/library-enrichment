# `datafusion_physical_expr::equivalence::properties::joins`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.equivalence.properties.joins.json`](../model/datafusion_physical_expr.equivalence.properties.joins.json)

## join_equivalence_properties

`function` · `datafusion_physical_expr::equivalence::properties::joins::join_equivalence_properties`

Also reachable as `datafusion_physical_expr::equivalence::join_equivalence_properties`

```rust
fn join_equivalence_properties(left: super::EquivalenceProperties, right: super::EquivalenceProperties, join_type: &datafusion_common::JoinType, join_schema: arrow::datatypes::SchemaRef, maintains_input_order: &[bool], probe_side: Option<datafusion_common::JoinSide>, on: &[(PhysicalExprRef, PhysicalExprRef)]) -> datafusion_common::Result<super::EquivalenceProperties>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.equivalence.properties.joins.join_equivalence_properties.md).


Calculate ordering equivalence properties for the given join operation.

---
