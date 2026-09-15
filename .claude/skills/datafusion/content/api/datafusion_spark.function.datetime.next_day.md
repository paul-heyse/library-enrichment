# `datafusion_spark::function::datetime::next_day`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.datetime.next_day.json`](../model/datafusion_spark.function.datetime.next_day.json)

## SparkNextDay

`struct` · `datafusion_spark::function::datetime::next_day::SparkNextDay`

```rust
struct SparkNextDay
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
fn return_field_from_args(&self, _args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

<https://spark.apache.org/docs/latest/api/sql/index.html#next_day>

---
