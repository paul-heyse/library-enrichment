# `datafusion_functions_nested::map_entries`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.map_entries.json`](../model/datafusion_functions_nested.map_entries.json)

## map_entries

`function` · `datafusion_functions_nested::map_entries::map_entries`

Also reachable as `datafusion::prelude::map_entries`, `datafusion_functions_nested::expr_fn::map_entries`

```rust
fn map_entries(map: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Return a list of all entries in the map.

---

## map_entries_udf

`function` · `datafusion_functions_nested::map_entries::map_entries_udf`

```rust
fn map_entries_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapEntriesFunc

---

## MapEntriesFunc

`struct` · `datafusion_functions_nested::map_entries::MapEntriesFunc`

```rust
struct MapEntriesFunc
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

---
