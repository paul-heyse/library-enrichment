# `datafusion_expr_common::statistics::compute_variance`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.compute_variance.json).

<a id="op-479c549224efc3ed2b80a967"></a>
## compute_variance

`function` · `datafusion_expr_common::statistics::compute_variance` · datafusion-expr-common 55.1.0

```rust
fn compute_variance(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

Source: `src/statistics.rs:900`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Computes the variance value for the result of the given binary operation on
two unknown quantities represented by their [`Distribution`](../operations/datafusion_expr_common.statistics.Distribution.md#op-01128278dc758957f40620ba) objects.
