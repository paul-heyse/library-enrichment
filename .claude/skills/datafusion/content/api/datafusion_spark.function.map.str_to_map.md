# `datafusion_spark::function::map::str_to_map`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.map.str_to_map.json`](../model/datafusion_spark.function.map.str_to_map.json)

## SparkStrToMap

`struct` · `datafusion_spark::function::map::str_to_map::SparkStrToMap`

```rust
struct SparkStrToMap
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

Spark-compatible `str_to_map` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#str_to_map>

Creates a map from a string by splitting on delimiters.
str_to_map(text[, pairDelim[, keyValueDelim]]) -> Map<String, String>

- text: The input string
- pairDelim: Delimiter between key-value pairs (default: ',')
- keyValueDelim: Delimiter between key and value (default: ':')

# Duplicate Key Handling
Mirrors Spark's [`spark.sql.mapKeyDedupPolicy`](https://github.com/apache/spark/blob/v4.0.0/sql/catalyst/src/main/scala/org/apache/spark/sql/internal/SQLConf.scala#L4502-L4511),
wired through DataFusion's `datafusion.spark.map_key_dedup_policy`:
- `EXCEPTION` (default): error on duplicate keys.
- `LAST_WIN`: keep the last occurrence of each duplicate key.

---
