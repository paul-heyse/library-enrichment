# `datafusion_substrait::logical_plan::producer::rel::read_rel`

Crate `datafusion-substrait` · 3 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.rel.read_rel.json`](../model/datafusion_substrait.logical_plan.producer.rel.read_rel.json)

## from_empty_relation

`function` · `datafusion_substrait::logical_plan::producer::rel::read_rel::from_empty_relation`

```rust
fn from_empty_relation(producer: &mut impl SubstraitProducer, e: &datafusion::logical_expr::EmptyRelation) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

Encodes an EmptyRelation as a Substrait VirtualTable.

EmptyRelation represents a relation with no input data. When `produce_one_row` is true,
it generates a single row with all fields set to their default values (typically NULL).
This is used for queries without a FROM clause, such as "SELECT 1 AS one" or
"SELECT current_timestamp()".

When `produce_one_row` is false, it represents a truly empty relation with no rows,
used in optimizations or as a placeholder.

---

## from_table_scan

`function` · `datafusion_substrait::logical_plan::producer::rel::read_rel::from_table_scan`

```rust
fn from_table_scan(producer: &mut impl SubstraitProducer, scan: &datafusion::logical_expr::TableScan) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---

## from_values

`function` · `datafusion_substrait::logical_plan::producer::rel::read_rel::from_values`

```rust
fn from_values(producer: &mut impl SubstraitProducer, v: &datafusion::logical_expr::Values) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---
