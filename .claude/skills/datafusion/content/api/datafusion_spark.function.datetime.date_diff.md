# `datafusion_spark::function::datetime::date_diff`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.datetime.date_diff.json`](../model/datafusion_spark.function.datetime.date_diff.json)

## SparkDateDiff

`struct` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff`

```rust
struct SparkDateDiff
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
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.date_diff.SparkDateDiff.md).


<https://spark.apache.org/docs/latest/api/sql/index.html#date_diff>

---
