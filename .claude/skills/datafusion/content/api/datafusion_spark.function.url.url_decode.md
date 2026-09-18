# `datafusion_spark::function::url::url_decode`

Crate `datafusion-spark` · 3 public items · structured records in [`model/datafusion_spark.function.url.url_decode.json`](../model/datafusion_spark.function.url.url_decode.json)

## OnDecodeError

`enum` · `datafusion_spark::function::url::url_decode::OnDecodeError`

```rust
enum OnDecodeError
```

**Variants**: `Fail`, `Null`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.url_decode.OnDecodeError.md).


How [`spark_handled_url_decode`] reacts to a malformed input value.

---

## spark_handled_url_decode

`function` · `datafusion_spark::function::url::url_decode::spark_handled_url_decode`

```rust
fn spark_handled_url_decode(args: &[arrow::array::ArrayRef], on_error: OnDecodeError) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.url_decode.spark_handled_url_decode.md).


---

## UrlDecode

`struct` · `datafusion_spark::function::url::url_decode::UrlDecode`

```rust
struct UrlDecode
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.url_decode.UrlDecode.md).


---
