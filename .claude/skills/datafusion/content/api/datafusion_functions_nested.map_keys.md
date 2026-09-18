# `datafusion_functions_nested::map_keys`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.map_keys.json`](../model/datafusion_functions_nested.map_keys.json)

## map_keys

`function` · `datafusion_functions_nested::map_keys::map_keys`

Also reachable as `datafusion::prelude::map_keys`, `datafusion_functions_nested::expr_fn::map_keys`

```rust
fn map_keys(map: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map_keys.map_keys.md).


Return a list of all keys in the map.

---

## map_keys_udf

`function` · `datafusion_functions_nested::map_keys::map_keys_udf`

```rust
fn map_keys_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map_keys.map_keys_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapKeysFunc

---

## MapKeysFunc

`struct` · `datafusion_functions_nested::map_keys::MapKeysFunc`

```rust
struct MapKeysFunc
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.map_keys.MapKeysFunc.md).


---
