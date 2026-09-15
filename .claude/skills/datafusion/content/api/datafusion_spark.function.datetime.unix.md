# `datafusion_spark::function::datetime::unix`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.datetime.unix.json`](../model/datafusion_spark.function.datetime.unix.json)

## SparkUnixDate

`struct` · `datafusion_spark::function::datetime::unix::SparkUnixDate`

```rust
struct SparkUnixDate
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Returns the number of days since epoch (1970-01-01) for the given date.
<https://spark.apache.org/docs/latest/api/sql/index.html#unix_date>

---

## SparkUnixTimestamp

`struct` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp`

```rust
struct SparkUnixTimestamp
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn microseconds() -> Self
fn milliseconds() -> Self
fn new(name: &'static str, time_unit: TimeUnit) -> Self
fn seconds() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

---
