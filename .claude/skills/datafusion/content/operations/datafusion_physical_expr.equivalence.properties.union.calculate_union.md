# `datafusion_physical_expr::equivalence::properties::union::calculate_union`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.properties.union.calculate_union.json).

<a id="op-3cf2d54a96c001d64c2739ff"></a>
## calculate_union

`function` · `datafusion_physical_expr::equivalence::properties::union::calculate_union` · datafusion-physical-expr 55.1.0

```rust
fn calculate_union(eqps: Vec<super::EquivalenceProperties>, schema: arrow::datatypes::SchemaRef) -> datafusion_common::Result<super::EquivalenceProperties>
```

Source: `src/equivalence/properties/union.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Calculates the union (in the sense of `UnionExec`) `EquivalenceProperties`
of the given `EquivalenceProperties` in `eqps` according to the given
output `schema` (which need not be the same with those of `lhs` and `rhs`
as details such as nullability may be different).
