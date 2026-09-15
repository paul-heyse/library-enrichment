# `datafusion_functions_nested::cardinality`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.cardinality.json`](../model/datafusion_functions_nested.cardinality.json)

## cardinality

`function` · `datafusion_functions_nested::cardinality::cardinality`

Also reachable as `datafusion::prelude::cardinality`, `datafusion_functions_nested::expr_fn::cardinality`

```rust
fn cardinality(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the total number of elements in the array or map.

---

## cardinality_udf

`function` · `datafusion_functions_nested::cardinality::cardinality_udf`

```rust
fn cardinality_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
Cardinality

---

## Cardinality

`struct` · `datafusion_functions_nested::cardinality::Cardinality`

```rust
struct Cardinality
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
