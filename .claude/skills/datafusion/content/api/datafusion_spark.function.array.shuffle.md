# `datafusion_spark::function::array::shuffle`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.array.shuffle.json`](../model/datafusion_spark.function.array.shuffle.json)

## SparkShuffle

`struct` · `datafusion_spark::function::array::shuffle::SparkShuffle`

```rust
struct SparkShuffle
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
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.array.shuffle.SparkShuffle.md).


---
