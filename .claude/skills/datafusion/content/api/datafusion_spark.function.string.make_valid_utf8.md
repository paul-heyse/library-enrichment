# `datafusion_spark::function::string::make_valid_utf8`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.make_valid_utf8.json`](../model/datafusion_spark.function.string.make_valid_utf8.json)

## SparkMakeValidUtf8

`struct` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8`

```rust
struct SparkMakeValidUtf8
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.string.make_valid_utf8.SparkMakeValidUtf8.md).


Spark-compatible `make_valid_utf8` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#make_valid_utf8>

---
