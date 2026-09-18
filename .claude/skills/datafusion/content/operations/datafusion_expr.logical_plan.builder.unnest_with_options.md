# `datafusion_expr::logical_plan::builder::unnest_with_options`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.unnest_with_options.json).

<a id="op-e4f05034dbe297db57d843a1"></a>
## unnest_with_options

`function` · `datafusion_expr::logical_plan::builder::unnest_with_options` · datafusion-expr 55.1.0

```rust
fn unnest_with_options(input: logical_plan::LogicalPlan, columns_to_unnest: Vec<datafusion_common::Column>, options: datafusion_common::UnnestOptions) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Source: `src/logical_plan/builder.rs:2291`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a [`LogicalPlan::Unnest`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-1cdf6ed5deb59cb55472f8c2) plan with options
This function receive a list of columns to be unnested
because multiple unnest can be performed on the same column (e.g unnest with different depth)
The new schema will contains post-unnest fields replacing the original field

For example:
Input schema as
```text
+---------------------+-------------------+
| col1                | col2              |
+---------------------+-------------------+
| Struct(INT64,INT32) | List(List(Int64)) |
+---------------------+-------------------+
```



Then unnesting columns with:
- (col1,Struct)
- (col2,List(\[depth=1,depth=2\]))

will generate a new schema as
```text
+---------+---------+---------------------+---------------------+
| col1.c0 | col1.c1 | unnest_col2_depth_1 | unnest_col2_depth_2 |
+---------+---------+---------------------+---------------------+
| Int64   | Int32   | List(Int64)         |  Int64              |
+---------+---------+---------------------+---------------------+
```
