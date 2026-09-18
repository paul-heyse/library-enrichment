# `datafusion_spark::function::math::trigonometry`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.math.trigonometry.json`](../model/datafusion_spark.function.math.trigonometry.json)

## SparkCsc

`struct` · `datafusion_spark::function::math::trigonometry::SparkCsc`

```rust
struct SparkCsc
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.trigonometry.SparkCsc.md).


<https://spark.apache.org/docs/latest/api/sql/index.html#csc>

---

## SparkSec

`struct` · `datafusion_spark::function::math::trigonometry::SparkSec`

```rust
struct SparkSec
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.trigonometry.SparkSec.md).


<https://spark.apache.org/docs/latest/api/sql/index.html#sec>

---
