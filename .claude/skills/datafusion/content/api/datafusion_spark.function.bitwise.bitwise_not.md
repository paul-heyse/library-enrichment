# `datafusion_spark::function::bitwise::bitwise_not`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.bitwise.bitwise_not.json`](../model/datafusion_spark.function.bitwise.bitwise_not.json)

## spark_bitwise_not

`function` · `datafusion_spark::function::bitwise::bitwise_not::spark_bitwise_not`

```rust
fn spark_bitwise_not(args: &[ArrayRef]) -> datafusion_common::Result<ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.bitwise_not.spark_bitwise_not.md).


---

## SparkBitwiseNot

`struct` · `datafusion_spark::function::bitwise::bitwise_not::SparkBitwiseNot`

```rust
struct SparkBitwiseNot
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
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.bitwise_not.SparkBitwiseNot.md).


---
