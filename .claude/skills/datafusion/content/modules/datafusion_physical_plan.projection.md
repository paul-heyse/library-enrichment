# `datafusion_physical_plan::projection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.json).

<a id="op-7788e58ba719ea007cca214e"></a>
## projection

`module` · `datafusion_physical_plan::projection` · datafusion-physical-plan 55.1.0

```rust
mod projection
```

Source: `src/projection.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Defines the projection execution plan. A projection determines which columns or expressions
are returned from a query. The SQL statement `SELECT a, b, a+b FROM t1` is an example
of a projection on table `t1` where the expressions `a`, `b`, and `a+b` are the
projection expressions. `SELECT` without `FROM` will only evaluate expressions.
