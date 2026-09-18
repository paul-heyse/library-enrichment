# `datafusion_expr::logical_plan::builder::build_join_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.build_join_schema.json).

<a id="op-e1ef76959b8818ec70e0dd70"></a>
## build_join_schema

`function` · `datafusion_expr::logical_plan::builder::build_join_schema` · datafusion-expr 55.1.0

```rust
fn build_join_schema(left: &datafusion_common::DFSchema, right: &datafusion_common::DFSchema, join_type: &logical_plan::JoinType) -> datafusion_common::Result<datafusion_common::DFSchema>
```

Source: `src/logical_plan/builder.rs:1675`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a schema for a join operation.
The fields from the left side are first
