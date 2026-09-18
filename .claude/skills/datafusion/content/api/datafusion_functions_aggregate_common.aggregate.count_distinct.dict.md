# `datafusion_functions_aggregate_common::aggregate::count_distinct::dict`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.count_distinct.dict.json`](../model/datafusion_functions_aggregate_common.aggregate.count_distinct.dict.json)

## DictionaryCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::DictionaryCountAccumulator`

```rust
struct DictionaryCountAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Box<dyn Accumulator>) -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.dict.DictionaryCountAccumulator.md).


---
