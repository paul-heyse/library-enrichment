# `datafusion_spark::function::datetime::date_part`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.datetime.date_part.json`](../model/datafusion_spark.function.datetime.date_part.json)

## SparkDatePart

`struct` · `datafusion_spark::function::datetime::date_part::SparkDatePart`

```rust
struct SparkDatePart
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
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.date_part.SparkDatePart.md).


Wrapper around datafusion date_part function to handle
Spark behavior returning day of the week 1-indexed instead of 0-indexed and different part aliases.
<https://spark.apache.org/docs/latest/api/sql/index.html#date_part>

---
