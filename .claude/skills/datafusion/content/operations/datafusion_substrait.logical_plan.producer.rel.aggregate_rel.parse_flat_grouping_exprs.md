# `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::parse_flat_grouping_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.rel.aggregate_rel.parse_flat_grouping_exprs.json).

<a id="op-c73f0c238a31d0fa042c75fa"></a>
## parse_flat_grouping_exprs

`function` · `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::parse_flat_grouping_exprs` · datafusion-substrait 55.1.0

```rust
fn parse_flat_grouping_exprs(producer: &mut impl SubstraitProducer, exprs: &[datafusion::logical_expr::Expr], schema: &datafusion::common::DFSchemaRef, ref_group_exprs: &mut Vec<substrait::proto::Expression>) -> datafusion::common::Result<substrait::proto::aggregate_rel::Grouping>
```

Source: `src/logical_plan/producer/rel/aggregate_rel.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
