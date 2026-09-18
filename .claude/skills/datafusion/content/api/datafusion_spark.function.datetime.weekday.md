# `datafusion_spark::function::datetime::weekday`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.datetime.weekday.json`](../model/datafusion_spark.function.datetime.weekday.json)

## SparkWeekDay

`struct` · `datafusion_spark::function::datetime::weekday::SparkWeekDay`

```rust
struct SparkWeekDay
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.weekday.SparkWeekDay.md).


Spark-compatible `weekday` expression.
Returns the day of the week for a date or timestamp as an integer index where
Monday = 0, Tuesday = 1, ..., Sunday = 6.

Note: this differs from `dayofweek`, which is 1-indexed with Sunday = 1.

<https://spark.apache.org/docs/latest/api/sql/index.html#weekday>

---
