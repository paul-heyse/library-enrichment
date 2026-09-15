# `datafusion_spark::function::math::factorial`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.math.factorial.json`](../model/datafusion_spark.function.math.factorial.json)

## spark_factorial

`function` · `datafusion_spark::function::math::factorial::spark_factorial`

```rust
fn spark_factorial(args: &[datafusion_expr::ColumnarValue]) -> datafusion_common::Result<datafusion_expr::ColumnarValue, datafusion_common::DataFusionError>
```

---

## SparkFactorial

`struct` · `datafusion_spark::function::math::factorial::SparkFactorial`

```rust
struct SparkFactorial
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

<https://spark.apache.org/docs/latest/api/sql/index.html#factorial>

---
