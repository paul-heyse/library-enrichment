# `datafusion_spark::function::aggregate::avg`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.aggregate.avg.json`](../model/datafusion_spark.function.aggregate.avg.json)

## AvgAccumulator

`struct` · `datafusion_spark::function::aggregate::avg::AvgAccumulator`

```rust
struct AvgAccumulator
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.aggregate.avg.AvgAccumulator.md).


An accumulator to compute the average

---

## SparkAvg

`struct` · `datafusion_spark::function::aggregate::avg::SparkAvg`

```rust
struct SparkAvg
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.aggregate.avg.SparkAvg.md).


AVG aggregate expression
Spark average aggregate expression. Differs from standard DataFusion average aggregate
in that it uses an `i64` for the count (DataFusion version uses `u64`); also there is ANSI mode
support planned in the future for Spark version.

---
