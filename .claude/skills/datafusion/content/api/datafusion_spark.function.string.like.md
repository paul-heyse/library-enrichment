# `datafusion_spark::function::string::like`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.string.like.json`](../model/datafusion_spark.function.string.like.json)

## spark_like

`function` · `datafusion_spark::function::string::like::spark_like`

```rust
fn spark_like(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Returns true if str matches pattern (case sensitive).

---

## SparkLike

`struct` · `datafusion_spark::function::string::like::SparkLike`

```rust
struct SparkLike
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

LIKE function for case-sensitive pattern matching
<https://spark.apache.org/docs/latest/api/sql/index.html#like>

---
