# `datafusion_spark::function::string::quote`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.quote.json`](../model/datafusion_spark.function.string.quote.json)

## SparkQuote

`struct` · `datafusion_spark::function::string::quote::SparkQuote`

```rust
struct SparkQuote
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.string.quote.SparkQuote.md).


Spark-compatible `quote` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#quote>

---
