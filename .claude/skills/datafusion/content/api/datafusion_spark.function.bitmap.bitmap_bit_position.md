# `datafusion_spark::function::bitmap::bitmap_bit_position`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.bitmap.bitmap_bit_position.json`](../model/datafusion_spark.function.bitmap.bitmap_bit_position.json)

## bitmap_bit_position_inner

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::bitmap_bit_position_inner`

```rust
fn bitmap_bit_position_inner(arg: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitmap.bitmap_bit_position.bitmap_bit_position_inner.md).


---

## BitmapBitPosition

`struct` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition`

```rust
struct BitmapBitPosition
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitmap.bitmap_bit_position.BitmapBitPosition.md).


Spark-compatible `bitmap_bit_position` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#bitmap_bit_position>

---
