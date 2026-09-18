# `datafusion_physical_plan::repartition::REPARTITION_RANDOM_STATE`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.repartition.REPARTITION_RANDOM_STATE.json).

<a id="op-ab77f078d0adc5aab471eb51"></a>
## REPARTITION_RANDOM_STATE

`constant` · `datafusion_physical_plan::repartition::REPARTITION_RANDOM_STATE` · datafusion-physical-plan 55.1.0

```rust
const REPARTITION_RANDOM_STATE: joins::SeededRandomState = _
```

Source: `src/repartition/mod.rs:650`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Fixed RandomState used for hash repartitioning to ensure consistent behavior across
executions and runs.
