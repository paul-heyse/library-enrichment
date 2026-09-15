# `datafusion_spark::function::string::length`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.length.json`](../model/datafusion_spark.function.string.length.json)

## SparkLengthFunc

`struct` · `datafusion_spark::function::string::length::SparkLengthFunc`

```rust
struct SparkLengthFunc
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
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> datafusion_common::Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> datafusion_common::Result<FieldRef>
fn return_type(&self, _args: &[DataType]) -> datafusion_common::Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `length` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#length>

---
