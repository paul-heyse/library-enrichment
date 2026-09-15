# `datafusion_functions_aggregate_common::min_max`

Crate `datafusion-functions-aggregate-common` · 4 public items · structured records in [`model/datafusion_functions_aggregate_common.min_max.json`](../model/datafusion_functions_aggregate_common.min_max.json)

## max_batch

`function` · `datafusion_functions_aggregate_common::min_max::max_batch`

```rust
fn max_batch(values: &arrow::array::ArrayRef) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

dynamically-typed max(array) -> ScalarValue

---

## min_batch

`function` · `datafusion_functions_aggregate_common::min_max::min_batch`

```rust
fn min_batch(values: &arrow::array::ArrayRef) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

dynamically-typed min(array) -> ScalarValue

---

## MaxAccumulator

`struct` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator`

Also reachable as `datafusion_functions_aggregate::min_max::MaxAccumulator`

```rust
struct MaxAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(datatype: &DataType) -> Result<Self>
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

An accumulator to compute the maximum value

---

## MinAccumulator

`struct` · `datafusion_functions_aggregate_common::min_max::MinAccumulator`

Also reachable as `datafusion_functions_aggregate::min_max::MinAccumulator`

```rust
struct MinAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(datatype: &DataType) -> Result<Self>
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

An accumulator to compute the minimum value

---
