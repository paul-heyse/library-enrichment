# `datafusion_spark::function::bitmap::bitmap_count`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.bitmap.bitmap_count.json`](../model/datafusion_spark.function.bitmap.bitmap_count.json)

## bitmap_count_inner

`function` · `datafusion_spark::function::bitmap::bitmap_count::bitmap_count_inner`

```rust
fn bitmap_count_inner(arg: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

---

## BitmapCount

`struct` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount`

```rust
struct BitmapCount
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

---
