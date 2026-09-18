# `datafusion_spark::function::math::floor`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.math.floor.json`](../model/datafusion_spark.function.math.floor.json)

## SparkFloor

`struct` · `datafusion_spark::function::math::floor::SparkFloor`

```rust
struct SparkFloor
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> datafusion_common::Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> datafusion_common::Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> datafusion_common::Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.floor.SparkFloor.md).


Spark-compatible `floor` function.

Differences from DataFusion's floor:
- Returns Int64 for float and integer inputs (while DataFusion preserves input type)
- For Decimal128(p, s), returns Decimal128(p-s+1, 0) with scale 0
  (DataFusion preserves original precision and scale)

<https://spark.apache.org/docs/latest/api/sql/index.html#floor>

---
