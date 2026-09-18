# `datafusion_expr::logical_plan::builder::add_group_by_exprs_from_dependencies`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.add_group_by_exprs_from_dependencies.json).

<a id="op-3f19c2388e9f8cd71a519005"></a>
## add_group_by_exprs_from_dependencies

`function` · `datafusion_expr::logical_plan::builder::add_group_by_exprs_from_dependencies` · datafusion-expr 55.1.0

```rust
fn add_group_by_exprs_from_dependencies(group_expr: Vec<Expr>, schema: &datafusion_common::DFSchemaRef) -> datafusion_common::Result<Vec<Expr>>
```

Source: `src/logical_plan/builder.rs:1858`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add additional "synthetic" group by expressions based on functional
dependencies.

For example, if we are grouping on `[c1]`, and we know from
functional dependencies that column `c1` determines `c2`, this function
adds `c2` to the group by list.

This allows MySQL style selects like
`SELECT col FROM t WHERE pk = 5` if col is unique
