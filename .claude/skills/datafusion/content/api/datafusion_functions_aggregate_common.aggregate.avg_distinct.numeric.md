# `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.avg_distinct.numeric.json`](../model/datafusion_functions_aggregate_common.aggregate.avg_distinct.numeric.json)

## Float64DistinctAvgAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::avg_distinct::Float64DistinctAvgAccumulator`

```rust
struct Float64DistinctAvgAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug, Default

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.avg_distinct.numeric.Float64DistinctAvgAccumulator.md).


Specialized implementation of `AVG DISTINCT` for Float64 values, leveraging
the existing DistinctSumAccumulator implementation.

---
