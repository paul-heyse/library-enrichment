# `datafusion_spark::function::aggregate::expr_fn`

Crate `datafusion-spark` · 4 public items · structured records in [`model/datafusion_spark.function.aggregate.expr_fn.json`](../model/datafusion_spark.function.aggregate.expr_fn.json)

## avg

`function` · `datafusion_spark::function::aggregate::expr_fn::avg`

Also reachable as `datafusion_spark::expr_fn::avg`

```rust
fn avg(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.aggregate.expr_fn.avg.md).


Returns the average value of a given column

---

## collect_list

`function` · `datafusion_spark::function::aggregate::expr_fn::collect_list`

Also reachable as `datafusion_spark::expr_fn::collect_list`

```rust
fn collect_list(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.aggregate.expr_fn.collect_list.md).


Returns a list created from the values in a column

---

## collect_set

`function` · `datafusion_spark::function::aggregate::expr_fn::collect_set`

Also reachable as `datafusion_spark::expr_fn::collect_set`

```rust
fn collect_set(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.aggregate.expr_fn.collect_set.md).


Returns a set created from the values in a column

---

## try_sum

`function` · `datafusion_spark::function::aggregate::expr_fn::try_sum`

Also reachable as `datafusion_spark::expr_fn::try_sum`

```rust
fn try_sum(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.aggregate.expr_fn.try_sum.md).


Returns the sum of values for a column, or NULL if overflow occurs

---
