# `datafusion_physical_expr::intervals::cp_solver::propagate_arithmetic`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.cp_solver.propagate_arithmetic.json).

<a id="op-cb3da384bffc6f11e0392897"></a>
## propagate_arithmetic

`function` · `datafusion_physical_expr::intervals::cp_solver::propagate_arithmetic` · datafusion-physical-expr 55.1.0

```rust
fn propagate_arithmetic(op: &datafusion_expr::Operator, parent: &datafusion_expr::interval_arithmetic::Interval, left_child: &datafusion_expr::interval_arithmetic::Interval, right_child: &datafusion_expr::interval_arithmetic::Interval) -> datafusion_common::Result<Option<(datafusion_expr::interval_arithmetic::Interval, datafusion_expr::interval_arithmetic::Interval)>>
```

Source: `src/intervals/cp_solver.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function refines intervals `left_child` and `right_child` by applying
constraint propagation through `parent` via operation. The main idea is
that we can shrink ranges of variables x and y using parent interval p.

Assuming that x,y and p has ranges `[xL, xU]`, `[yL, yU]`, and `[pL, pU]`, we
apply the following operations:
- For plus operation, specifically, we would first do
    - `[xL, xU]` <- (`[pL, pU]` - `[yL, yU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[pL, pU]` - `[xL, xU]`) ∩ `[yL, yU]`.
- For minus operation, specifically, we would first do
    - `[xL, xU]` <- (`[yL, yU]` + `[pL, pU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[xL, xU]` - `[pL, pU]`) ∩ `[yL, yU]`.
- For multiplication operation, specifically, we would first do
    - `[xL, xU]` <- (`[pL, pU]` / `[yL, yU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[pL, pU]` / `[xL, xU]`) ∩ `[yL, yU]`.
- For division operation, specifically, we would first do
    - `[xL, xU]` <- (`[yL, yU]` * `[pL, pU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[xL, xU]` / `[pL, pU]`) ∩ `[yL, yU]`.
