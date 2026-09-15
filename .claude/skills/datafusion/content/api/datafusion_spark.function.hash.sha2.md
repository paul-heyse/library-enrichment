# `datafusion_spark::function::hash::sha2`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.hash.sha2.json`](../model/datafusion_spark.function.hash.sha2.json)

## SparkSha2

`struct` · `datafusion_spark::function::hash::sha2::SparkSha2`

```rust
struct SparkSha2
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Differs from DataFusion version in allowing array input for bit lengths, and
also hex encoding the output.

<https://spark.apache.org/docs/latest/api/sql/index.html#sha2>

---
