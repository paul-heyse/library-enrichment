# `datafusion_expr::expr_fn::ExprFunctionExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.ExprFunctionExt.json).

<a id="op-99506f92b515415a11439278"></a>
## ExprFunctionExt

`trait` · `datafusion_expr::expr_fn::ExprFunctionExt` · datafusion-expr 55.1.0

```rust
trait ExprFunctionExt
```

Source: `src/expr_fn.rs:784`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Extensions for configuring [`Expr::AggregateFunction`](../operations/datafusion_expr.expr.Expr.md#op-98c26e3c882c5c14cacb82a2) or [`Expr::WindowFunction`](../operations/datafusion_expr.expr.Expr.md#op-1ab47cd303f22d10dd7f4cec)

Adds methods to [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) that make it easy to set optional options
such as `ORDER BY`, `FILTER` and `DISTINCT`

# Example
```no_run
# use datafusion_common::Result;
# use datafusion_expr::expr::NullTreatment;
# use datafusion_expr::test::function_stub::count;
# use datafusion_expr::{ExprFunctionExt, lit, Expr, col};
# // first_value is an aggregate function in another crate
# fn first_value(_arg: Expr) -> Expr {
unimplemented!() }
# fn main() -> Result<()> {
// Create an aggregate count, filtering on column y > 5
let agg = count(col("x")).filter(col("y").gt(lit(5))).build()?;

// Find the first value in an aggregate sorted by column y
// equivalent to:
// `FIRST_VALUE(x ORDER BY y ASC IGNORE NULLS)`
let sort_expr = col("y").sort(true, true);
let agg = first_value(col("x"))
    .order_by(vec![sort_expr])
    .null_treatment(NullTreatment::IgnoreNulls)
    .build()?;

// Create a window expression for percent rank partitioned on column a
// equivalent to:
// `PERCENT_RANK() OVER (PARTITION BY a ORDER BY b ASC NULLS LAST IGNORE NULLS)`
// percent_rank is an udwf function in another crate
# fn percent_rank() -> Expr {
unimplemented!() }
let window = percent_rank()
    .partition_by(vec![col("a")])
    .order_by(vec![col("b").sort(true, true)])
    .null_treatment(NullTreatment::IgnoreNulls)
    .build()?;
#     Ok(())
# }
```

<a id="op-6a9c00eb246788bcbb40473b"></a>
## distinct

`function` · `datafusion_expr::expr_fn::ExprFunctionExt::distinct` · datafusion-expr 55.1.0

```rust
fn distinct(self) -> ExprFuncBuilder
```

Source: `src/expr_fn.rs:790`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `DISTINCT`

<a id="op-d5293f83797428d55704e265"></a>
## filter

`function` · `datafusion_expr::expr_fn::ExprFunctionExt::filter` · datafusion-expr 55.1.0

```rust
fn filter(self, filter: Expr) -> ExprFuncBuilder
```

Source: `src/expr_fn.rs:788`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `FILTER <filter>`

<a id="op-5f475d2212cf5e359b0583b7"></a>
## null_treatment

`function` · `datafusion_expr::expr_fn::ExprFunctionExt::null_treatment` · datafusion-expr 55.1.0

```rust
fn null_treatment(self, null_treatment: impl Into<Option<NullTreatment>>) -> ExprFuncBuilder
```

Source: `src/expr_fn.rs:792`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `RESPECT NULLS` or `IGNORE NULLS`

<a id="op-d8b15fef2f0a8509568b69b0"></a>
## order_by

`function` · `datafusion_expr::expr_fn::ExprFunctionExt::order_by` · datafusion-expr 55.1.0

```rust
fn order_by(self, order_by: Vec<Sort>) -> ExprFuncBuilder
```

Source: `src/expr_fn.rs:786`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `ORDER BY <order_by>`

<a id="op-afd76d8405511ca0813279f2"></a>
## partition_by

`function` · `datafusion_expr::expr_fn::ExprFunctionExt::partition_by` · datafusion-expr 55.1.0

```rust
fn partition_by(self, partition_by: Vec<Expr>) -> ExprFuncBuilder
```

Source: `src/expr_fn.rs:797`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `PARTITION BY`

<a id="op-9027b7152a6900898ac5813a"></a>
## window_frame

`function` · `datafusion_expr::expr_fn::ExprFunctionExt::window_frame` · datafusion-expr 55.1.0

```rust
fn window_frame(self, window_frame: WindowFrame) -> ExprFuncBuilder
```

Source: `src/expr_fn.rs:799`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add appropriate window frame conditions
