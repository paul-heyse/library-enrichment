# `datafusion_physical_plan::common::project_plan_to_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.common.project_plan_to_schema.json).

<a id="op-1a1fb6bb177d44acd5025756"></a>
## project_plan_to_schema

`function` · `datafusion_physical_plan::common::project_plan_to_schema` · datafusion-physical-plan 55.1.0

```rust
fn project_plan_to_schema(input: std::sync::Arc<dyn ExecutionPlan>, expected_schema: &arrow::datatypes::SchemaRef) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/common.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Align `input`'s physical plan schema with `expected_schema`.

This helper is intended for operators that combine independently planned children but
expose a single declared output schema. It returns `input` unchanged when schemas already
match exactly. Otherwise, it validates that projection can safely produce the expected
schema, then wraps `input` in a [`ProjectionExec`](../operations/datafusion_physical_plan.projection.ProjectionExec.md#op-b46d9dc006ec8aae1caad158) that keeps columns in their existing
positional order and aliases them to `expected_schema`'s field names.

[`ProjectionExec`](../operations/datafusion_physical_plan.projection.ProjectionExec.md#op-b46d9dc006ec8aae1caad158) can rename fields. When the expected field is nullable and the input
field is not, this helper also widens nullability with a same-type [`CastExpr`](../operations/datafusion_physical_expr.expressions.cast.CastExpr.md#op-b2ec9a951f65fbe4161b17ed). It rejects
differences that projection cannot safely normalize exactly, such as data type, metadata,
schema metadata, and nullability narrowing.
