# `datafusion_expr_common::statistics::combine_gaussians`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.combine_gaussians.json).

<a id="op-2197c047490479fe8d5fcf03"></a>
## combine_gaussians

`function` · `datafusion_expr_common::statistics::combine_gaussians` · datafusion-expr-common 55.1.0

```rust
fn combine_gaussians(op: &operator::Operator, left: &GaussianDistribution, right: &GaussianDistribution) -> datafusion_common::Result<Option<GaussianDistribution>>
```

Source: `src/statistics.rs:694`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Applies the given operation to the given Gaussian distributions. Currently,
this function handles only addition and subtraction operations. If the
result is not a Gaussian random variable, it returns `None`. For details,
see:

<https://en.wikipedia.org/wiki/Sum_of_normally_distributed_random_variables>
