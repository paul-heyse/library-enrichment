# `datafusion_expr::utils::merge_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.merge_schema.json).

<a id="op-679335f6a4d3d3de93224c01"></a>
## merge_schema

`function` · `datafusion_expr::utils::merge_schema` · datafusion-expr 55.1.0

```rust
fn merge_schema(inputs: &[&LogicalPlan]) -> datafusion_common::DFSchema
```

Source: `src/utils.rs:1403`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

merge inputs schema into a single schema.

This function merges schemas from multiple logical plan inputs using [`DFSchema::merge`].
Refer to that documentation for details on precedence and metadata handling.

Unresolved upstream links (retained, not inferred): ``DFSchema::merge``.
