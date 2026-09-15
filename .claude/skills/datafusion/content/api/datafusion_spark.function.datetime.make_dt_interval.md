# `datafusion_spark::function::datetime::make_dt_interval`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.datetime.make_dt_interval.json`](../model/datafusion_spark.function.datetime.make_dt_interval.json)

## SparkMakeDtInterval

`struct` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval`

```rust
struct SparkMakeDtInterval
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

---
