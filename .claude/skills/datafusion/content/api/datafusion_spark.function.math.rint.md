# `datafusion_spark::function::math::rint`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.math.rint.json`](../model/datafusion_spark.function.math.rint.json)

## spark_rint

`function` · `datafusion_spark::function::math::rint::spark_rint`

```rust
fn spark_rint(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.rint.spark_rint.md).


---

## SparkRint

`struct` · `datafusion_spark::function::math::rint::SparkRint`

```rust
struct SparkRint
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
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.rint.SparkRint.md).


---
