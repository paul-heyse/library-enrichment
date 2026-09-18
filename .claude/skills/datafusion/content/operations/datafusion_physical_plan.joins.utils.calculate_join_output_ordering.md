# `datafusion_physical_plan::joins::utils::calculate_join_output_ordering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.calculate_join_output_ordering.json).

<a id="op-7777b13ee32416e81d071d80"></a>
## calculate_join_output_ordering

`function` · `datafusion_physical_plan::joins::utils::calculate_join_output_ordering` · datafusion-physical-plan 55.1.0

```rust
fn calculate_join_output_ordering(left_ordering: Option<&datafusion_physical_expr::LexOrdering>, right_ordering: Option<&datafusion_physical_expr::LexOrdering>, join_type: datafusion_common::JoinType, left_columns_len: usize, maintains_input_order: &[bool], probe_side: Option<datafusion_common::JoinSide>) -> datafusion_common::Result<Option<datafusion_physical_expr::LexOrdering>>
```

Source: `src/joins/utils.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Calculate the output ordering of a given join operation.
