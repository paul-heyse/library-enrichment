# `datafusion_functions_nested::dimension`

Crate `datafusion-functions-nested` · 5 public items · structured records in [`model/datafusion_functions_nested.dimension.json`](../model/datafusion_functions_nested.dimension.json)

## array_dims

`function` · `datafusion_functions_nested::dimension::array_dims`

Also reachable as `datafusion::prelude::array_dims`, `datafusion_functions_nested::expr_fn::array_dims`

```rust
fn array_dims(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns an array of the array's dimensions.

---

## array_dims_udf

`function` · `datafusion_functions_nested::dimension::array_dims_udf`

```rust
fn array_dims_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayDims

---

## array_ndims

`function` · `datafusion_functions_nested::dimension::array_ndims`

Also reachable as `datafusion::prelude::array_ndims`, `datafusion_functions_nested::expr_fn::array_ndims`

```rust
fn array_ndims(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the number of dimensions of the array.

---

## array_ndims_udf

`function` · `datafusion_functions_nested::dimension::array_ndims_udf`

```rust
fn array_ndims_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayNdims

---

## ArrayDims

`struct` · `datafusion_functions_nested::dimension::ArrayDims`

```rust
struct ArrayDims
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
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
