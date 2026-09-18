# `datafusion_spark::function::datetime::to_utc_timestamp`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.datetime.to_utc_timestamp.json`](../model/datafusion_spark.function.datetime.to_utc_timestamp.json)

## SparkToUtcTimestamp

`struct` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp`

```rust
struct SparkToUtcTimestamp
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.to_utc_timestamp.SparkToUtcTimestamp.md).


Apache Spark `to_utc_timestamp` function.

Interprets the given timestamp in the provided timezone and then converts it to UTC.

Timestamp in Apache Spark represents number of microseconds from the Unix epoch, which is not
timezone-agnostic. So in Apache Spark this function just shift the timestamp value from the given
timezone to UTC timezone.

See <https://spark.apache.org/docs/latest/api/sql/index.html#to_utc_timestamp>

---
