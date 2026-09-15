# `datafusion_functions_nested::distance`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.distance.json`](../model/datafusion_functions_nested.distance.json)

## array_distance

`function` · `datafusion_functions_nested::distance::array_distance`

Also reachable as `datafusion::prelude::array_distance`, `datafusion_functions_nested::expr_fn::array_distance`

```rust
fn array_distance(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the Euclidean distance between two one-dimensional numeric arrays.

---

## array_distance_udf

`function` · `datafusion_functions_nested::distance::array_distance_udf`

```rust
fn array_distance_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayDistance

---

## ArrayDistance

`struct` · `datafusion_functions_nested::distance::ArrayDistance`

```rust
struct ArrayDistance
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
