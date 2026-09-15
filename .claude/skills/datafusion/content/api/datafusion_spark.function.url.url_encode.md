# `datafusion_spark::function::url::url_encode`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.url.url_encode.json`](../model/datafusion_spark.function.url.url_encode.json)

## UrlEncode

`struct` · `datafusion_spark::function::url::url_encode::UrlEncode`

```rust
struct UrlEncode
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

---
