# `datafusion_spark::function::url::try_parse_url`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.url.try_parse_url.json`](../model/datafusion_spark.function.url.try_parse_url.json)

## TryParseUrl

`struct` · `datafusion_spark::function::url::try_parse_url::TryParseUrl`

```rust
struct TryParseUrl
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.try_parse_url.TryParseUrl.md).


TRY_PARSE_URL function for tolerant URL component extraction (never errors; returns NULL on invalid or missing parts).
<https://spark.apache.org/docs/latest/api/sql/index.html#try_parse_url>

---
