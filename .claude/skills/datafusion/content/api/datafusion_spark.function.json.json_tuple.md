# `datafusion_spark::function::json::json_tuple`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.json.json_tuple.json`](../model/datafusion_spark.function.json.json_tuple.json)

## JsonTuple

`struct` · `datafusion_spark::function::json::json_tuple::JsonTuple`

```rust
struct JsonTuple
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
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `json_tuple` expression

<https://spark.apache.org/docs/latest/api/sql/index.html#json_tuple>

Extracts top-level fields from a JSON string and returns them as a struct.

`json_tuple(json_string, field1, field2, ...) -> Struct<c0: Utf8, c1: Utf8, ...>`

Note: In Spark, `json_tuple` is a Generator that produces multiple columns directly.
In DataFusion, a ScalarUDF can only return one value per row, so the result is wrapped
in a Struct. The caller (e.g. Comet) is expected to destructure the struct fields.

- Returns NULL for each field that is missing from the JSON object
- Returns NULL for all fields if the input is NULL or not valid JSON
- Non-string JSON values are converted to their JSON string representation
- JSON `null` values are returned as NULL (not the string "null")

---
