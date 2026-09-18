# `datafusion_expr::logical_plan::invariants::assert_expected_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.invariants.assert_expected_schema.json).

<a id="op-013bd139c056a48ddcc662a1"></a>
## assert_expected_schema

`function` · `datafusion_expr::logical_plan::invariants::assert_expected_schema` · datafusion-expr 55.1.0

```rust
fn assert_expected_schema(schema: &datafusion_common::DFSchemaRef, plan: &LogicalPlan) -> datafusion_common::Result<()>
```

Source: `src/logical_plan/invariants.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns an error if the plan does not have the expected schema.
Ignores metadata and nullability.
