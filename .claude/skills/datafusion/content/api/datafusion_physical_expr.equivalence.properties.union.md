# `datafusion_physical_expr::equivalence::properties::union`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.equivalence.properties.union.json`](../model/datafusion_physical_expr.equivalence.properties.union.json)

## calculate_union

`function` · `datafusion_physical_expr::equivalence::properties::union::calculate_union`

Also reachable as `datafusion::physical_expr::calculate_union`, `datafusion_physical_expr::calculate_union`, `datafusion_physical_expr::equivalence::calculate_union`

```rust
fn calculate_union(eqps: Vec<super::EquivalenceProperties>, schema: arrow::datatypes::SchemaRef) -> datafusion_common::Result<super::EquivalenceProperties>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.equivalence.properties.union.calculate_union.md).


Calculates the union (in the sense of `UnionExec`) `EquivalenceProperties`
of the given `EquivalenceProperties` in `eqps` according to the given
output `schema` (which need not be the same with those of `lhs` and `rhs`
as details such as nullability may be different).

---
