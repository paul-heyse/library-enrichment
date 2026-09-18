# `datafusion_functions_aggregate_common::noop_accumulator`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.noop_accumulator.json`](../model/datafusion_functions_aggregate_common.noop_accumulator.json)

## NoopAccumulator

`struct` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator`

```rust
struct NoopAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new(evaluate_value: ScalarValue) -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, _states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, _values: &[ArrayRef]) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.noop_accumulator.NoopAccumulator.md).


[`Accumulator`] that does no work and always returns a fixed value (default
of `NULL` but can be customized).

Useful for aggregate functions that need to handle an input of [`DataType::Null`]
that does no work.

[`DataType::Null`]: arrow::datatypes::DataType::Null

---
