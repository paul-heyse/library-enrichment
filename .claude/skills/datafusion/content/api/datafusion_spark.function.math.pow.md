# `datafusion_spark::function::math::pow`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.pow.json`](../model/datafusion_spark.function.math.pow.json)

## SparkPow

`struct` · `datafusion_spark::function::math::pow::SparkPow`

```rust
struct SparkPow
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
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.pow.SparkPow.md).


Spark-compatible implementation of `pow` / `power`.

Behavioural difference from the DataFusion default:
- `pow(0, <negative>)` → `Infinity`  (IEEE 754 / Spark semantics)
  The default raises `"zero raised to a negative power is undefined"` to
  match PostgreSQL.

---
