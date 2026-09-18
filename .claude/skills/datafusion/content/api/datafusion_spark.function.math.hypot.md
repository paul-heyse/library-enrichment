# `datafusion_spark::function::math::hypot`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.hypot.json`](../model/datafusion_spark.function.math.hypot.json)

## SparkHypot

`struct` · `datafusion_spark::function::math::hypot::SparkHypot`

```rust
struct SparkHypot
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.hypot.SparkHypot.md).


Spark-compatible `hypot` function.

<https://spark.apache.org/docs/latest/api/sql/index.html#hypot>

Returns `sqrt(expr1^2 + expr2^2)` computed without intermediate overflow or
underflow, matching Spark's use of `java.lang.Math.hypot`.

---
