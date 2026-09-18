# `datafusion_functions_nested::cosine_distance`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.cosine_distance.json`](../model/datafusion_functions_nested.cosine_distance.json)

## cosine_distance

`function` · `datafusion_functions_nested::cosine_distance::cosine_distance`

Also reachable as `datafusion::prelude::cosine_distance`, `datafusion_functions_nested::expr_fn::cosine_distance`

```rust
fn cosine_distance(array1: datafusion_expr::Expr, array2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.cosine_distance.cosine_distance.md).


returns the cosine distance between two numeric arrays.

---

## cosine_distance_udf

`function` · `datafusion_functions_nested::cosine_distance::cosine_distance_udf`

```rust
fn cosine_distance_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.cosine_distance.cosine_distance_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
CosineDistance

---

## CosineDistance

`struct` · `datafusion_functions_nested::cosine_distance::CosineDistance`

```rust
struct CosineDistance
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.cosine_distance.CosineDistance.md).


---
