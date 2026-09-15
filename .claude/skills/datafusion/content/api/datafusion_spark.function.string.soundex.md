# `datafusion_spark::function::string::soundex`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.soundex.json`](../model/datafusion_spark.function.string.soundex.json)

## SparkSoundex

`struct` · `datafusion_spark::function::string::soundex::SparkSoundex`

```rust
struct SparkSoundex
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `soundex` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#soundex>

---
