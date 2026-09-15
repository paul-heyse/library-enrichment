# `datafusion_functions_aggregate_common::aggregate::sum_distinct::numeric`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.sum_distinct.numeric.json`](../model/datafusion_functions_aggregate_common.aggregate.sum_distinct.numeric.json)

## DistinctSumAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::sum_distinct::numeric::DistinctSumAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::sum_distinct::DistinctSumAccumulator`

```rust
struct DistinctSumAccumulator<T: ArrowPrimitiveType>
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (2)

```rust
fn distinct_count(&self) -> usize
fn new(data_type: &DataType) -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Accumulator for computing SUM(DISTINCT expr)

---
