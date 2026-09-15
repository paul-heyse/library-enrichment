# `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes`

Crate `datafusion-functions-aggregate-common` · 2 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.count_distinct.bytes.json`](../model/datafusion_functions_aggregate_common.aggregate.count_distinct.bytes.json)

## BytesDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::BytesDistinctCountAccumulator`

```rust
struct BytesDistinctCountAccumulator<O: OffsetSizeTrait>
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(output_type: OutputType) -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Specialized implementation of
`COUNT DISTINCT` for [`StringArray`] [`LargeStringArray`],
[`BinaryArray`] and [`LargeBinaryArray`].

[`StringArray`]: arrow::array::StringArray
[`LargeStringArray`]: arrow::array::LargeStringArray
[`BinaryArray`]: arrow::array::BinaryArray
[`LargeBinaryArray`]: arrow::array::LargeBinaryArray

---

## BytesViewDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::BytesViewDistinctCountAccumulator`

```rust
struct BytesViewDistinctCountAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(output_type: OutputType) -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Specialized implementation of
`COUNT DISTINCT` for [`StringViewArray`] and [`BinaryViewArray`].

[`StringViewArray`]: arrow::array::StringViewArray
[`BinaryViewArray`]: arrow::array::BinaryViewArray

---
