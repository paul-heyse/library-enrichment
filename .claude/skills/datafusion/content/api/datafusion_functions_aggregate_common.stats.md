# `datafusion_functions_aggregate_common::stats`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.stats.json`](../model/datafusion_functions_aggregate_common.stats.json)

## StatsType

`enum` · `datafusion_functions_aggregate_common::stats::StatsType`

Also reachable as `datafusion_physical_expr::expressions::StatsType`, `datafusion_physical_plan::execution_plan::expressions::StatsType`, `datafusion_physical_plan::expressions::StatsType`

```rust
enum StatsType
```

**Variants**: `Population`, `Sample`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

TODO: Move this to functions-aggregate module
Enum used for differentiating population and sample for statistical functions

---
