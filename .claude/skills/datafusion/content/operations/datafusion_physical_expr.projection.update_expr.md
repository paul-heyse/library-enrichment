# `datafusion_physical_expr::projection::update_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.update_expr.json).

<a id="op-5318705f25c637abc4758b4e"></a>
## update_expr

`function` · `datafusion_physical_expr::projection::update_expr` · datafusion-physical-expr 55.1.0

```rust
fn update_expr(expr: &std::sync::Arc<dyn PhysicalExpr>, projected_exprs: &[ProjectionExpr], unproject: bool) -> datafusion_common::Result<Option<std::sync::Arc<dyn PhysicalExpr>>>
```

Source: `src/projection.rs:1076`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The function projects / unprojects an expression with respect to set of
projection expressions.

See also [`ProjectionExprs::unproject_expr`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-17ddaa2d6115725dd89dea92) and [`ProjectionExprs::project_expr`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-f86f5d0b156f02cf391b34f3)

1) When `unproject` is `true`:

   Rewrites an expression with respect to the projection expressions,
   effectively "unprojecting" it to reference the original input columns.

   For example, given
   * the expressions `a@1 + b@2` and `c@0`
   * and projection expressions `c@2, a@0, b@1`

   Then
   * `a@1 + b@2` becomes `a@0 + b@1`
   * `c@0` becomes `c@2`

2) When `unproject` is `false`:

   Rewrites the expression to reference the projected expressions,
   effectively "projecting" it. The resulting expression will reference the
   indices as they appear in the projection.

   If the expression cannot be rewritten after the projection, it returns
   `None`.

   For example, given
   * the expressions `c@0`, `a@1` and `b@2`
   * the projection `a@1 as a, c@0 as c_new`,

   Then
   * `c@0` becomes `c_new@1`
   * `a@1` becomes `a@0`
   * `b@2` results in `None` since the projection does not include `b`.

# Errors
This function returns an error if `unproject` is `true` and if any expression references
an index that is out of bounds for `projected_exprs`.
For example:

- `expr` is `a@3`
- `projected_exprs` is \[`a@0`, `b@1`\]

In this case, `a@3` references index 3, which is out of bounds for `projected_exprs` (which has length 2).
