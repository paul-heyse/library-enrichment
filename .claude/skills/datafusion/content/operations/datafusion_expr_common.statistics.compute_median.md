# `datafusion_expr_common::statistics::compute_median`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.compute_median.json).

<a id="op-b30b0da1e083dd8efe46c194"></a>
## compute_median

`function` · `datafusion_expr_common::statistics::compute_median` · datafusion-expr-common 55.1.0

```rust
fn compute_median(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

Source: `src/statistics.rs:859`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Computes the median value for the result of the given binary operation on
two unknown quantities represented by its [`Distribution`](../operations/datafusion_expr_common.statistics.Distribution.md#op-01128278dc758957f40620ba) objects. Currently,
the median is calculable only for addition and subtraction operations on:
- [`Uniform`](../operations/datafusion_expr_common.statistics.Distribution.md#op-526d899958dbd74fbf3ef9bb) and [`Uniform`](../operations/datafusion_expr_common.statistics.Distribution.md#op-526d899958dbd74fbf3ef9bb) distributions, and
- [`Gaussian`](../operations/datafusion_expr_common.statistics.Distribution.md#op-eb391024bc9ca20f52803716) and [`Gaussian`](../operations/datafusion_expr_common.statistics.Distribution.md#op-eb391024bc9ca20f52803716) distributions.
