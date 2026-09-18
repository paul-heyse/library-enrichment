# `datafusion_expr_common::statistics::compute_mean`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.compute_mean.json).

<a id="op-9be26bce1732a8ac824eb7a4"></a>
## compute_mean

`function` · `datafusion_expr_common::statistics::compute_mean` · datafusion-expr-common 55.1.0

```rust
fn compute_mean(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

Source: `src/statistics.rs:823`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Computes the mean value for the result of the given binary operation on
two unknown quantities represented by their [`Distribution`](../operations/datafusion_expr_common.statistics.Distribution.md#op-01128278dc758957f40620ba) objects.
