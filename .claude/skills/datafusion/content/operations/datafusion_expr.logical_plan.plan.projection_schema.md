# `datafusion_expr::logical_plan::plan::projection_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.projection_schema.json).

<a id="op-bd8549e837ad73fab71e6eab"></a>
## projection_schema

`function` · `datafusion_expr::logical_plan::plan::projection_schema` · datafusion-expr 55.1.0

```rust
fn projection_schema(input: &LogicalPlan, exprs: &[Expr]) -> datafusion_common::Result<std::sync::Arc<datafusion_common::DFSchema>>
```

Source: `src/logical_plan/plan.rs:2528`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Computes the schema of the result produced by applying a projection to the input logical plan.

# Arguments

* `input`: A reference to the input `LogicalPlan` for which the projection schema
  will be computed.
* `exprs`: A slice of `Expr` expressions representing the projection operation to apply.

# Metadata Handling

- **Schema-level metadata**: Passed through unchanged from the input schema
- **Field-level metadata**: Determined by each expression via [`exprlist_to_fields`](../operations/datafusion_expr.utils.exprlist_to_fields.md#op-3c3dc6d54191f75636f0ab8c), which
  calls [`Expr::to_field`](../operations/datafusion_expr.expr.Expr.md#op-77306878ad632fdf80f4fb7d) to handle expression-specific metadata (literals, aliases, etc.)

# Returns

A `Result` containing an `Arc<DFSchema>` representing the schema of the result
produced by the projection operation. If the schema computation is successful,
the `Result` will contain the schema; otherwise, it will contain an error.
