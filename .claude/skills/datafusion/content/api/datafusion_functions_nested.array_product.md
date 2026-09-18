# `datafusion_functions_nested::array_product`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_product.json`](../model/datafusion_functions_nested.array_product.json)

## array_product

`function` · `datafusion_functions_nested::array_product::array_product`

Also reachable as `datafusion::prelude::array_product`, `datafusion_functions_nested::expr_fn::array_product`

```rust
fn array_product(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_product.array_product.md).


returns the product of the elements of a numeric array.

---

## array_product_udf

`function` · `datafusion_functions_nested::array_product::array_product_udf`

```rust
fn array_product_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_product.array_product_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayProduct

---

## ArrayProduct

`struct` · `datafusion_functions_nested::array_product::ArrayProduct`

```rust
struct ArrayProduct
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_product.ArrayProduct.md).


---
