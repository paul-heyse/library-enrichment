# `datafusion_spark::function::math::round`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.round.json`](../model/datafusion_spark.function.math.round.json)

## SparkRound

`struct` · `datafusion_spark::function::math::round::SparkRound`

```rust
struct SparkRound
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.round.SparkRound.md).


Spark-compatible `round` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#round>

Rounds the value of `expr` to `scale` decimal places using HALF_UP rounding mode.
Returns the same type as the input expression.

- `round(expr)` rounds to 0 decimal places (default scale = 0)
- `round(expr, scale)` rounds to `scale` decimal places
- For integer types with negative scale: `round(25, -1)` → `30`
- Uses HALF_UP rounding: 2.5 → 3, -2.5 → -3 (away from zero)

Supported types: Int8, Int16, Int32, Int64, UInt8, UInt16, UInt32, UInt64,
Float16, Float32, Float64, Decimal32, Decimal64, Decimal128, Decimal256

---
