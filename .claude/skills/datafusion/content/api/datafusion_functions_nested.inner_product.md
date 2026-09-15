# `datafusion_functions_nested::inner_product`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.inner_product.json`](../model/datafusion_functions_nested.inner_product.json)

## inner_product

`function` · `datafusion_functions_nested::inner_product::inner_product`

Also reachable as `datafusion::prelude::inner_product`, `datafusion_functions_nested::expr_fn::inner_product`

```rust
fn inner_product(array1: datafusion_expr::Expr, array2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the inner product (dot product) of two numeric arrays.

---

## inner_product_udf

`function` · `datafusion_functions_nested::inner_product::inner_product_udf`

```rust
fn inner_product_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
InnerProduct

---

## InnerProduct

`struct` · `datafusion_functions_nested::inner_product::InnerProduct`

```rust
struct InnerProduct
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
