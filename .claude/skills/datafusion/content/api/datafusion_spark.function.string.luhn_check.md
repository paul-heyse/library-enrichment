# `datafusion_spark::function::string::luhn_check`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.string.luhn_check.json`](../model/datafusion_spark.function.string.luhn_check.json)

## SparkLuhnCheck

`struct` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck`

```rust
struct SparkLuhnCheck
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

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.string.luhn_check.SparkLuhnCheck.md).


Spark-compatible `luhn_check` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#luhn_check>

---
