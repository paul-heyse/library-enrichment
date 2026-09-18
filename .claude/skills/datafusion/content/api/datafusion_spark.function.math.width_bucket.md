# `datafusion_spark::function::math::width_bucket`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.width_bucket.json`](../model/datafusion_spark.function.math.width_bucket.json)

## SparkWidthBucket

`struct` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket`

```rust
struct SparkWidthBucket
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.width_bucket.SparkWidthBucket.md).


---
