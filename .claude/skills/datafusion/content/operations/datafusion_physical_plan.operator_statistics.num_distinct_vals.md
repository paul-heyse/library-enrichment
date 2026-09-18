# `datafusion_physical_plan::operator_statistics::num_distinct_vals`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.num_distinct_vals.json).

<a id="op-1e94884fc3c40c054b2b17d1"></a>
## num_distinct_vals

`function` · `datafusion_physical_plan::operator_statistics::num_distinct_vals` · datafusion-physical-plan 55.1.0

```rust
fn num_distinct_vals(domain_size: usize, num_selected: usize) -> usize
```

Source: `src/operator_statistics/mod.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Estimate the number of distinct values when sampling from a population.

Given a domain with `domain_size` distinct values and `num_selected` rows
sampled/filtered from it, estimates how many distinct values will appear
in the sample.

Uses the formula: `Expected distinct = N * [1 - (1 - 1/N)^n]`

# References

Based on Calcite's `RelMdUtil.numDistinctVals()`:
<https://github.com/apache/calcite/blob/main/core/src/main/java/org/apache/calcite/rel/metadata/RelMdUtil.java>
