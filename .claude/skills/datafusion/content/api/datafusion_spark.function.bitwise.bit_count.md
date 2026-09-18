# `datafusion_spark::function::bitwise::bit_count`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.bitwise.bit_count.json`](../model/datafusion_spark.function.bitwise.bit_count.json)

## SparkBitCount

`struct` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount`

```rust
struct SparkBitCount
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.bit_count.SparkBitCount.md).


---
