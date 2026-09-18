# `datafusion_spark::function::string::space`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.string.space.json`](../model/datafusion_spark.function.string.space.json)

## spark_space

`function` · `datafusion_spark::function::string::space::spark_space`

```rust
fn spark_space(args: &[datafusion_expr::ColumnarValue]) -> datafusion_common::Result<datafusion_expr::ColumnarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.string.space.spark_space.md).


---

## SparkSpace

`struct` · `datafusion_spark::function::string::space::SparkSpace`

```rust
struct SparkSpace
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
fn return_type(&self, args: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.string.space.SparkSpace.md).


Spark-compatible `space` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#space>

---
