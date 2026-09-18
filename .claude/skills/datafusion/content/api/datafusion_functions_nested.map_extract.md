# `datafusion_functions_nested::map_extract`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.map_extract.json`](../model/datafusion_functions_nested.map_extract.json)

## map_extract

`function` · `datafusion_functions_nested::map_extract::map_extract`

Also reachable as `datafusion::prelude::map_extract`, `datafusion_functions_nested::expr_fn::map_extract`

```rust
fn map_extract(map: datafusion_expr::Expr, key: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map_extract.map_extract.md).


Return a list containing the value for a given key or an empty list if the key is not contained in the map.

---

## map_extract_udf

`function` · `datafusion_functions_nested::map_extract::map_extract_udf`

```rust
fn map_extract_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map_extract.map_extract_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapExtract

---

## MapExtract

`struct` · `datafusion_functions_nested::map_extract::MapExtract`

```rust
struct MapExtract
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map_extract.MapExtract.md).


---
