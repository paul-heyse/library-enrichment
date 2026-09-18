# `datafusion_spark::function::collection::size`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.collection.size.json`](../model/datafusion_spark.function.collection.size.json)

## SparkSize

`struct` · `datafusion_spark::function::collection::size::SparkSize`

```rust
struct SparkSize
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
fn return_field_from_args(&self, _args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.collection.size.SparkSize.md).


Spark-compatible `size` function.

Returns the number of elements in an array or the number of key-value pairs in a map.
Returns -1 for null input (Spark behavior).

---
