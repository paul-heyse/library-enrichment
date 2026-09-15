# `datafusion_spark::function::string::is_valid_utf8`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.is_valid_utf8.json`](../model/datafusion_spark.function.string.is_valid_utf8.json)

## SparkIsValidUtf8

`struct` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8`

```rust
struct SparkIsValidUtf8
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
fn return_field_from_args(&self, _args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `is_valid_utf8` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#is_valid_utf8>

---
