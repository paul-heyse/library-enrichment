# `datafusion_spark::function::math::negative`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.negative.json`](../model/datafusion_spark.function.math.negative.json)

## SparkNegative

`struct` · `datafusion_spark::function::math::negative::SparkNegative`

```rust
struct SparkNegative
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `negative` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#negative>

Returns the negation of input (equivalent to unary minus)
Returns NULL if input is NULL, returns NaN if input is NaN.

ANSI mode support:
 - When ANSI mode is disabled (`spark.sql.ansi.enabled=false`), negating the minimal
   value of a signed integer wraps around. For example: negative(i32::MIN) returns
   i32::MIN (wraps instead of error).
 - When ANSI mode is enabled (`spark.sql.ansi.enabled=true`), overflow conditions
   throw an ARITHMETIC_OVERFLOW error instead of wrapping.

---
