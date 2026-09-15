# `datafusion_spark::function::string::substring`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.substring.json`](../model/datafusion_spark.function.string.substring.json)

## SparkSubstring

`struct` · `datafusion_spark::function::string::substring::SparkSubstring`

```rust
struct SparkSubstring
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `substring` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#substring>

Returns the substring from string starting at position pos with length len.
Position is 1-indexed. If pos is negative, it counts from the end of the string.
Returns NULL if any input is NULL.

---
