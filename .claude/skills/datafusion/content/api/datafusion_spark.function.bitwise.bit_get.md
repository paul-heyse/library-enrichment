# `datafusion_spark::function::bitwise::bit_get`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.bitwise.bit_get.json`](../model/datafusion_spark.function.bitwise.bit_get.json)

## SparkBitGet

`struct` · `datafusion_spark::function::bitwise::bit_get::SparkBitGet`

```rust
struct SparkBitGet
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
