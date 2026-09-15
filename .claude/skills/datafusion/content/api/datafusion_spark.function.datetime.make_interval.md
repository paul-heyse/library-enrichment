# `datafusion_spark::function::datetime::make_interval`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.datetime.make_interval.json`](../model/datafusion_spark.function.datetime.make_interval.json)

## SparkMakeInterval

`struct` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval`

```rust
struct SparkMakeInterval
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

---
