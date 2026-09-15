# `datafusion_spark::function::array::expr_fn`

Crate `datafusion-spark` · 5 public items · structured records in [`model/datafusion_spark.function.array.expr_fn.json`](../model/datafusion_spark.function.array.expr_fn.json)

## array

`function` · `datafusion_spark::function::array::expr_fn::array`

Also reachable as `datafusion_spark::expr_fn::array`

```rust
fn array(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns an array with the given elements.

---

## array_repeat

`function` · `datafusion_spark::function::array::expr_fn::array_repeat`

Also reachable as `datafusion_spark::expr_fn::array_repeat`

```rust
fn array_repeat(element: datafusion_expr::Expr, count: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns an array containing element count times.

---

## shuffle

`function` · `datafusion_spark::function::array::expr_fn::shuffle`

Also reachable as `datafusion_spark::expr_fn::shuffle`

```rust
fn shuffle(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns a random permutation of the given array.

---

## slice

`function` · `datafusion_spark::function::array::expr_fn::slice`

Also reachable as `datafusion_spark::expr_fn::slice`

```rust
fn slice(array: datafusion_expr::Expr, start: datafusion_expr::Expr, length: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns a slice of the array from the start index with the given length.

---

## spark_array_contains

`function` · `datafusion_spark::function::array::expr_fn::spark_array_contains`

Also reachable as `datafusion_spark::expr_fn::spark_array_contains`

```rust
fn spark_array_contains(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns true if the array contains the element (Spark semantics).

---
