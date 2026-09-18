# `datafusion_physical_plan::operator_statistics::ndv_after_selectivity`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.ndv_after_selectivity.json).

<a id="op-020c90a37459d60ca0e9690a"></a>
## ndv_after_selectivity

`function` · `datafusion_physical_plan::operator_statistics::ndv_after_selectivity` · datafusion-physical-plan 55.1.0

```rust
fn ndv_after_selectivity(original_ndv: usize, original_rows: usize, selectivity: f64) -> usize
```

Source: `src/operator_statistics/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Estimate NDV after applying a selectivity factor (filtering).

When filtering rows, each distinct value has multiple rows. If a value
appears `k` times, the probability it survives the filter is `1 - (1-s)^k`
where `s` is the selectivity.

Assuming uniform distribution (each value appears `rows/ndv` times):
```text
NDV_after ~ NDV_before * [1 - (1 - selectivity)^(rows/NDV)]
```
