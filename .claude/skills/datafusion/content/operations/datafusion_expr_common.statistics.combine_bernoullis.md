# `datafusion_expr_common::statistics::combine_bernoullis`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.combine_bernoullis.json).

<a id="op-cae2e84e1137de1895a1bd60"></a>
## combine_bernoullis

`function` · `datafusion_expr_common::statistics::combine_bernoullis` · datafusion-expr-common 55.1.0

```rust
fn combine_bernoullis(op: &operator::Operator, left: &BernoulliDistribution, right: &BernoulliDistribution) -> datafusion_common::Result<BernoulliDistribution>
```

Source: `src/statistics.rs:635`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

This function takes a logical operator and two Bernoulli distributions,
and it returns a new Bernoulli distribution that represents the result of
the operation. Currently, only `AND` and `OR` operations are supported.
