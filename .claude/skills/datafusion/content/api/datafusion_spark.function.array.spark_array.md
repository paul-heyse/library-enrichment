# `datafusion_spark::function::array::spark_array`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.array.spark_array.json`](../model/datafusion_spark.function.array.spark_array.json)

## make_array_inner

`function` · `datafusion_spark::function::array::spark_array::make_array_inner`

```rust
fn make_array_inner(arrays: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

`make_array_inner` is the implementation of the `make_array` function.
Constructs an array using the input `data` as `ArrayRef`.
Returns a reference-counted `Array` instance result.

---

## SparkArray

`struct` · `datafusion_spark::function::array::spark_array::SparkArray`

```rust
struct SparkArray
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
