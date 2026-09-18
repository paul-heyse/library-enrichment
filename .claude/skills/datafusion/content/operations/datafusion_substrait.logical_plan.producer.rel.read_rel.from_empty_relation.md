# `datafusion_substrait::logical_plan::producer::rel::read_rel::from_empty_relation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.rel.read_rel.from_empty_relation.json).

<a id="op-f078fcf34df7d01b866beca2"></a>
## from_empty_relation

`function` · `datafusion_substrait::logical_plan::producer::rel::read_rel::from_empty_relation` · datafusion-substrait 55.1.0

```rust
fn from_empty_relation(producer: &mut impl SubstraitProducer, e: &datafusion::logical_expr::EmptyRelation) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

Source: `src/logical_plan/producer/rel/read_rel.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Encodes an EmptyRelation as a Substrait VirtualTable.

EmptyRelation represents a relation with no input data. When `produce_one_row` is true,
it generates a single row with all fields set to their default values (typically NULL).
This is used for queries without a FROM clause, such as "SELECT 1 AS one" or
"SELECT current_timestamp()".

When `produce_one_row` is false, it represents a truly empty relation with no rows,
used in optimizations or as a placeholder.
