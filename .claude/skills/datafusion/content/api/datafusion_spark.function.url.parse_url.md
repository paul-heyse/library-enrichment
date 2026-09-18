# `datafusion_spark::function::url::parse_url`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.url.parse_url.json`](../model/datafusion_spark.function.url.parse_url.json)

## spark_handled_parse_url

`function` · `datafusion_spark::function::url::parse_url::spark_handled_parse_url`

```rust
fn spark_handled_parse_url(args: &[arrow::array::ArrayRef], handler_err: impl Fn(datafusion_common::Result<Option<String>>) -> datafusion_common::Result<Option<String>>) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.parse_url.spark_handled_parse_url.md).


---

## ParseUrl

`struct` · `datafusion_spark::function::url::parse_url::ParseUrl`

```rust
struct ParseUrl
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.parse_url.ParseUrl.md).


---
