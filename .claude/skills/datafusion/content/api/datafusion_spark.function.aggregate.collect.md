# `datafusion_spark::function::aggregate::collect`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.aggregate.collect.json`](../model/datafusion_spark.function.aggregate.collect.json)

## SparkCollectList

`struct` · `datafusion_spark::function::aggregate::collect::SparkCollectList`

```rust
struct SparkCollectList
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
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

---

## SparkCollectSet

`struct` · `datafusion_spark::function::aggregate::collect::SparkCollectSet`

```rust
struct SparkCollectSet
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
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

---
