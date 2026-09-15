# `datafusion_spark::function::bitmap::bitmap_bucket_number`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.bitmap.bitmap_bucket_number.json`](../model/datafusion_spark.function.bitmap.bitmap_bucket_number.json)

## bitmap_bucket_number_inner

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::bitmap_bucket_number_inner`

```rust
fn bitmap_bucket_number_inner(arg: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

---

## BitmapBucketNumber

`struct` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber`

```rust
struct BitmapBucketNumber
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
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `bitmap_bucket_number` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#bitmap_bucket_number>

---
