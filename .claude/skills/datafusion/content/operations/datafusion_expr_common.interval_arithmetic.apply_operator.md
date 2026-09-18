# `datafusion_expr_common::interval_arithmetic::apply_operator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.interval_arithmetic.apply_operator.json).

<a id="op-39d8f2beb0f1714324da632f"></a>
## apply_operator

`function` · `datafusion_expr_common::interval_arithmetic::apply_operator` · datafusion-expr-common 55.1.0

```rust
fn apply_operator(op: &operator::Operator, lhs: &Interval, rhs: &Interval) -> datafusion_common::Result<Interval>
```

Source: `src/interval_arithmetic.rs:1052`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Applies the given binary operator the `lhs` and `rhs` arguments.
