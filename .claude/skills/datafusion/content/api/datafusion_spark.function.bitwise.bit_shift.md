# `datafusion_spark::function::bitwise::bit_shift`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.bitwise.bit_shift.json`](../model/datafusion_spark.function.bitwise.bit_shift.json)

## SparkBitShift

`struct` · `datafusion_spark::function::bitwise::bit_shift::SparkBitShift`

```rust
struct SparkBitShift
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn left() -> Self
fn right() -> Self
fn right_unsigned() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
