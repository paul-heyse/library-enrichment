# `datafusion_spark::function::math::hex`

Crate `datafusion-spark` · 4 public items · structured records in [`model/datafusion_spark.function.math.hex.json`](../model/datafusion_spark.function.math.hex.json)

## compute_hex

`function` · `datafusion_spark::function::math::hex::compute_hex`

```rust
fn compute_hex(args: &[datafusion_expr::ColumnarValue], lowercase: bool) -> Result<datafusion_expr::ColumnarValue, datafusion_common::DataFusionError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.hex.compute_hex.md).


---

## spark_hex

`function` · `datafusion_spark::function::math::hex::spark_hex`

```rust
fn spark_hex(args: &[datafusion_expr::ColumnarValue]) -> Result<datafusion_expr::ColumnarValue, datafusion_common::DataFusionError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.hex.spark_hex.md).


Spark-compatible `hex` function

---

## spark_sha2_hex

`function` · `datafusion_spark::function::math::hex::spark_sha2_hex`

```rust
fn spark_sha2_hex(args: &[datafusion_expr::ColumnarValue]) -> Result<datafusion_expr::ColumnarValue, datafusion_common::DataFusionError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.hex.spark_sha2_hex.md).


Spark-compatible `sha2` function

---

## SparkHex

`struct` · `datafusion_spark::function::math::hex::SparkHex`

```rust
struct SparkHex
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
fn return_type(&self, arg_types: &[DataType]) -> datafusion_common::Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.hex.SparkHex.md).


<https://spark.apache.org/docs/latest/api/sql/index.html#hex>

---
