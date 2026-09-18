# `datafusion_physical_expr::equivalence::properties::joins::join_equivalence_properties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.properties.joins.join_equivalence_properties.json).

<a id="op-930ec8da3704bf340513491e"></a>
## join_equivalence_properties

`function` · `datafusion_physical_expr::equivalence::properties::joins::join_equivalence_properties` · datafusion-physical-expr 55.1.0

```rust
fn join_equivalence_properties(left: super::EquivalenceProperties, right: super::EquivalenceProperties, join_type: &datafusion_common::JoinType, join_schema: arrow::datatypes::SchemaRef, maintains_input_order: &[bool], probe_side: Option<datafusion_common::JoinSide>, on: &[(PhysicalExprRef, PhysicalExprRef)]) -> datafusion_common::Result<super::EquivalenceProperties>
```

Source: `src/equivalence/properties/joins.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Calculate ordering equivalence properties for the given join operation.
