# `datafusion_spark::function::array::repeat`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.array.repeat.json`](../model/datafusion_spark.function.array.repeat.json)

## SparkArrayRepeat

`struct` · `datafusion_spark::function::array::repeat::SparkArrayRepeat`

```rust
struct SparkArrayRepeat
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `array_repeat` expression. The difference with DataFusion's `array_repeat` is the handling of NULL count: in Spark if the count is NULL, the result is NULL.
<https://spark.apache.org/docs/latest/api/sql/index.html#array_repeat>

---
