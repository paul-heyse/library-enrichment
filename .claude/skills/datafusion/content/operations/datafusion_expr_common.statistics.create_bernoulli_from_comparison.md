# `datafusion_expr_common::statistics::create_bernoulli_from_comparison`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.create_bernoulli_from_comparison.json).

<a id="op-412555970337b2fa19e34142"></a>
## create_bernoulli_from_comparison

`function` · `datafusion_expr_common::statistics::create_bernoulli_from_comparison` · datafusion-expr-common 55.1.0

```rust
fn create_bernoulli_from_comparison(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<Distribution>
```

Source: `src/statistics.rs:722`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Creates a new `Bernoulli` distribution by computing the resulting probability.
Expects `op` to be a comparison operator, with `left` and `right` having
numeric distributions. The resulting distribution has the `Float64` data
type.
