# `datafusion_physical_plan::joins::utils::build_join_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.build_join_schema.json).

<a id="op-91ec5ffcee59ebb6b3c79347"></a>
## build_join_schema

`function` · `datafusion_physical_plan::joins::utils::build_join_schema` · datafusion-physical-plan 55.1.0

```rust
fn build_join_schema(left: &arrow::datatypes::Schema, right: &arrow::datatypes::Schema, join_type: &datafusion_common::JoinType) -> (arrow::datatypes::Schema, Vec<ColumnIndex>)
```

Source: `src/joins/utils.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a schema for a join operation.
The fields from the left side are first
