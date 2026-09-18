# `datafusion_spark::function::string::ilike`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.string.ilike.json`](../model/datafusion_spark.function.string.ilike.json)

## spark_ilike

`function` · `datafusion_spark::function::string::ilike::spark_ilike`

```rust
fn spark_ilike(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.string.ilike.spark_ilike.md).


Returns true if str matches pattern (case insensitive).

---

## SparkILike

`struct` · `datafusion_spark::function::string::ilike::SparkILike`

```rust
struct SparkILike
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
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<Arc<Field>>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.string.ilike.SparkILike.md).


ILIKE function for case-insensitive pattern matching
<https://spark.apache.org/docs/latest/api/sql/index.html#ilike>

---
