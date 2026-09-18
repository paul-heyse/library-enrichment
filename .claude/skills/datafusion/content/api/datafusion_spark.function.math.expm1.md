# `datafusion_spark::function::math::expm1`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.expm1.json`](../model/datafusion_spark.function.math.expm1.json)

## SparkExpm1

`struct` · `datafusion_spark::function::math::expm1::SparkExpm1`

```rust
struct SparkExpm1
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.expm1.SparkExpm1.md).


<https://spark.apache.org/docs/latest/api/sql/index.html#expm1>

---
