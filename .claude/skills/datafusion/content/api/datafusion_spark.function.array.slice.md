# `datafusion_spark::function::array::slice`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.array.slice.json`](../model/datafusion_spark.function.array.slice.json)

## SparkSlice

`struct` · `datafusion_spark::function::array::slice::SparkSlice`

```rust
struct SparkSlice
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, func_args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.array.slice.SparkSlice.md).


Spark slice function implementation
Main difference from DataFusion's array_slice is that the third argument is the length of the slice and not the end index.
<https://spark.apache.org/docs/latest/api/sql/index.html#slice>

---
