# `datafusion_functions_aggregate_common::aggregate::avg_distinct::decimal`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.avg_distinct.decimal.json`](../model/datafusion_functions_aggregate_common.aggregate.avg_distinct.decimal.json)

## DecimalDistinctAvgAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::decimal::DecimalDistinctAvgAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::avg_distinct::DecimalDistinctAvgAccumulator`

```rust
struct DecimalDistinctAvgAccumulator<I: DecimalType + Debug, S: DecimalType + Debug = I>
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn with_decimal_params(sum_scale: i8, target_precision: u8, target_scale: i8) -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Generic implementation of `AVG DISTINCT` for Decimal types.
Handles both all Arrow decimal types (32, 64, 128 and 256 bits).

The distinct values are stored in the input type `I`; only the intermediate
sum is computed in the (never narrower) sum type `S` so it cannot overflow
`I`'s native type.

---
