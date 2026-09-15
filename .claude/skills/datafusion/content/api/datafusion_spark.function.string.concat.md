# `datafusion_spark::function::string::concat`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.concat.json`](../model/datafusion_spark.function.string.concat.json)

## SparkConcat

`struct` · `datafusion_spark::function::string::concat::SparkConcat`

```rust
struct SparkConcat
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `concat` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#concat>

Concatenates multiple input strings into a single string.
Returns NULL if any input is NULL.

Differences with DataFusion concat:
- Support 0 arguments
- Return NULL if any input is NULL

---
