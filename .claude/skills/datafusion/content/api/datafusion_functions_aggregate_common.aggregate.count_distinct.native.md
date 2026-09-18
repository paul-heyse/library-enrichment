# `datafusion_functions_aggregate_common::aggregate::count_distinct::native`

Crate `datafusion-functions-aggregate-common` · 7 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.count_distinct.native.json`](../model/datafusion_functions_aggregate_common.aggregate.count_distinct.native.json)

## Bitmap65536DistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::Bitmap65536DistinctCountAccumulator`

```rust
struct Bitmap65536DistinctCountAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.native.Bitmap65536DistinctCountAccumulator.md).


Optimized COUNT DISTINCT accumulator for u16 using a 65536-bit bitmap.
Uses 8KB (1024 x u64) to track all possible u16 values.

---

## Bitmap65536DistinctCountAccumulatorI16

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::Bitmap65536DistinctCountAccumulatorI16`

```rust
struct Bitmap65536DistinctCountAccumulatorI16
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.native.Bitmap65536DistinctCountAccumulatorI16.md).


Optimized COUNT DISTINCT accumulator for i16 using a 65536-bit bitmap.
Uses 8KB (1024 x u64) to track all possible i16 values (mapped to 0..65535).

---

## BoolArray256DistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::BoolArray256DistinctCountAccumulator`

```rust
struct BoolArray256DistinctCountAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.native.BoolArray256DistinctCountAccumulator.md).


Optimized COUNT DISTINCT accumulator for u8 using a bool array.
Uses 256 bytes to track all possible u8 values.

---

## BoolArray256DistinctCountAccumulatorI8

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::BoolArray256DistinctCountAccumulatorI8`

```rust
struct BoolArray256DistinctCountAccumulatorI8
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.native.BoolArray256DistinctCountAccumulatorI8.md).


Optimized COUNT DISTINCT accumulator for i8 using a bool array.
Uses 256 bytes to track all possible i8 values (mapped to 0..255).

---

## BooleanDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BooleanDistinctCountAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::BooleanDistinctCountAccumulator`

```rust
struct BooleanDistinctCountAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.native.BooleanDistinctCountAccumulator.md).


Optimized COUNT DISTINCT accumulator for `Boolean` using two flags.

Tracks whether `false` and `true` have been observed; nulls are skipped.
Result is always 0, 1, or 2.

---

## FloatDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::FloatDistinctCountAccumulator`

```rust
struct FloatDistinctCountAccumulator<T: ArrowPrimitiveType>
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.native.FloatDistinctCountAccumulator.md).


---

## PrimitiveDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::PrimitiveDistinctCountAccumulator`

```rust
struct PrimitiveDistinctCountAccumulator<T> where T: ArrowPrimitiveType + Send, T::Native: Eq + Hash
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(data_type: &DataType) -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.native.PrimitiveDistinctCountAccumulator.md).


---
