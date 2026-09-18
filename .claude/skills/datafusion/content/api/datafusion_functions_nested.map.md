# `datafusion_functions_nested::map`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.map.json`](../model/datafusion_functions_nested.map.json)

## map

`function` · `datafusion_functions_nested::map::map`

```rust
fn map(keys: Vec<datafusion_expr::Expr>, values: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map.map.md).


Returns a map created from a key list and a value list

---

## map_udf

`function` · `datafusion_functions_nested::map::map_udf`

```rust
fn map_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map.map_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapFunc

---

## MapFunc

`struct` · `datafusion_functions_nested::map::MapFunc`

```rust
struct MapFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map.MapFunc.md).


---
