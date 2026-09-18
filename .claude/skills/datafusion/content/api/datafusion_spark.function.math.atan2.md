# `datafusion_spark::function::math::atan2`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.atan2.json`](../model/datafusion_spark.function.math.atan2.json)

## SparkAtan2

`struct` · `datafusion_spark::function::math::atan2::SparkAtan2`

```rust
struct SparkAtan2
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.atan2.SparkAtan2.md).


Spark-compatible `atan2` function.

<https://spark.apache.org/docs/latest/api/sql/index.html#atan2>

`atan2(exprY, exprX)` returns the angle in radians between the positive
x-axis and the point given by the coordinates (exprX, exprY).

---
