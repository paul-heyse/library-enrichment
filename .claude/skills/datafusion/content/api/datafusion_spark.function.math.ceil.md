# `datafusion_spark::function::math::ceil`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.ceil.json`](../model/datafusion_spark.function.math.ceil.json)

## SparkCeil

`struct` · `datafusion_spark::function::math::ceil::SparkCeil`

```rust
struct SparkCeil
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `ceil` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#ceil>

Differences with DataFusion ceil:
 - Spark's ceil returns Int64 for float inputs; DataFusion preserves
   the input type (Float32→Float32, Float64→Float64)
 - Spark's ceil on Decimal128(p, s) returns Decimal128(p−s+1, 0), reducing scale
   to 0; DataFusion preserves the original precision and scale
 - Spark only supports Decimal128; DataFusion also supports Decimal32/64/256
 - Spark does not check for decimal overflow; DataFusion errors on overflow

2-argument ceil(value, scale) is not yet implemented
<https://github.com/apache/datafusion/issues/21560>

---
