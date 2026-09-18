# `datafusion_spark::function::datetime::time_trunc`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.datetime.time_trunc.json`](../model/datafusion_spark.function.datetime.time_trunc.json)

## SparkTimeTrunc

`struct` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc`

```rust
struct SparkTimeTrunc
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
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.time_trunc.SparkTimeTrunc.md).


Spark time_trunc function only handles time inputs.
<https://spark.apache.org/docs/latest/api/sql/index.html#time_trunc>

---
