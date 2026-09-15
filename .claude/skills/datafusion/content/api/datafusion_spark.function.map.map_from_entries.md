# `datafusion_spark::function::map::map_from_entries`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.function.map.map_from_entries.json`](../model/datafusion_spark.function.map.map_from_entries.json)

## MapFromEntries

`struct` · `datafusion_spark::function::map::map_from_entries::MapFromEntries`

```rust
struct MapFromEntries
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

Spark-compatible `map_from_entries` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#map_from_entries>

---
