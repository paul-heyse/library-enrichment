# `datafusion_spark::function::string::ascii`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.ascii.json`](../model/datafusion_spark.function.string.ascii.json)

## SparkAscii

`struct` · `datafusion_spark::function::string::ascii::SparkAscii`

```rust
struct SparkAscii
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

Spark compatible version of the [ascii] function. Differs from the [default ascii function]
in that it is more permissive of input types, for example casting numeric input to string
before executing the function (default version doesn't allow numeric input).

[ascii]: https://spark.apache.org/docs/latest/api/sql/index.html#ascii
[default ascii function]: datafusion_functions::string::ascii::AsciiFunc

---
