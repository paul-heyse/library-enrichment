# `datafusion_spark::function::aggregate::try_sum`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.aggregate.try_sum.json`](../model/datafusion_spark.function.aggregate.try_sum.json)

## SparkTrySum

`struct` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum`

```rust
struct SparkTrySum
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

---
