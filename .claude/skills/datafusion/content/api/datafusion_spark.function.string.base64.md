# `datafusion_spark::function::string::base64`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.string.base64.json`](../model/datafusion_spark.function.string.base64.json)

## SparkBase64

`struct` · `datafusion_spark::function::string::base64::SparkBase64`

```rust
struct SparkBase64
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

Apache Spark base64 uses padded base64 encoding.
<https://spark.apache.org/docs/latest/api/sql/index.html#base64>

---

## SparkUnBase64

`struct` · `datafusion_spark::function::string::base64::SparkUnBase64`

```rust
struct SparkUnBase64
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

<https://spark.apache.org/docs/latest/api/sql/index.html#unbase64>

---
