# `datafusion_spark::function::array::array_contains`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.array.array_contains.json`](../model/datafusion_spark.function.array.array_contains.json)

## SparkArrayContains

`struct` · `datafusion_spark::function::array::array_contains::SparkArrayContains`

```rust
struct SparkArrayContains
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
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `array_contains` function.

Calls DataFusion's `array_has` and then applies Spark's null semantics:
- If the result from `array_has` is `true`, return `true`.
- If the result is `false` and the input array row contains any null elements,
  return `null` (because the element might have been the null).
- If the result is `false` and the input array row has no null elements,
  return `false`.

---
