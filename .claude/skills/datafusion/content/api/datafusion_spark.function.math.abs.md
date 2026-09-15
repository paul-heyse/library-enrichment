# `datafusion_spark::function::math::abs`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.math.abs.json`](../model/datafusion_spark.function.math.abs.json)

## spark_abs

`function` · `datafusion_spark::function::math::abs::spark_abs`

```rust
fn spark_abs(args: &[datafusion_expr::ColumnarValue], enable_ansi_mode: bool) -> datafusion_common::Result<datafusion_expr::ColumnarValue, datafusion_common::DataFusionError>
```

---

## SparkAbs

`struct` · `datafusion_spark::function::math::abs::SparkAbs`

```rust
struct SparkAbs
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

Spark-compatible `abs` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#abs>

Returns the absolute value of input
Returns NULL if input is NULL, returns NaN if input is NaN.

Differences with DataFusion abs:
 - Spark's ANSI-compliant dialect, when off (i.e. `spark.sql.ansi.enabled=false`), taking absolute value on the minimal value of a signed integer returns the value as is. DataFusion's abs throws "DataFusion error: Arrow error: Compute error" on arithmetic overflow

TODOs:
 - Spark's abs also supports ANSI interval types: YearMonthIntervalType and DayTimeIntervalType. DataFusion's abs doesn't.

---
