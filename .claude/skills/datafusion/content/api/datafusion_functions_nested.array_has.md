# `datafusion_functions_nested::array_has`

Crate `datafusion-functions-nested` · 9 public items · structured records in [`model/datafusion_functions_nested.array_has.json`](../model/datafusion_functions_nested.array_has.json)

## array_has

`function` · `datafusion_functions_nested::array_has::array_has`

Also reachable as `datafusion::prelude::array_has`, `datafusion_functions_nested::expr_fn::array_has`

```rust
fn array_has(haystack_array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns true, if the element appears in the first array, otherwise false.

---

## array_has_all

`function` · `datafusion_functions_nested::array_has::array_has_all`

Also reachable as `datafusion::prelude::array_has_all`, `datafusion_functions_nested::expr_fn::array_has_all`

```rust
fn array_has_all(haystack_array: datafusion_expr::Expr, needle_array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns true if each element of the second array appears in the first array; otherwise, it returns false.

---

## array_has_all_udf

`function` · `datafusion_functions_nested::array_has::array_has_all_udf`

```rust
fn array_has_all_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayHasAll

---

## array_has_any

`function` · `datafusion_functions_nested::array_has::array_has_any`

Also reachable as `datafusion::prelude::array_has_any`, `datafusion_functions_nested::expr_fn::array_has_any`

```rust
fn array_has_any(first_array: datafusion_expr::Expr, second_array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns true if at least one element of the second array appears in the first array; otherwise, it returns false.

---

## array_has_any_udf

`function` · `datafusion_functions_nested::array_has::array_has_any_udf`

```rust
fn array_has_any_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayHasAny

---

## array_has_udf

`function` · `datafusion_functions_nested::array_has::array_has_udf`

```rust
fn array_has_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayHas

---

## ArrayHas

`struct` · `datafusion_functions_nested::array_has::ArrayHas`

```rust
struct ArrayHas
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
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, _info: &datafusion_expr::simplify::SimplifyContext) -> Result<ExprSimplifyResult>
```

---

## ArrayHasAll

`struct` · `datafusion_functions_nested::array_has::ArrayHasAll`

```rust
struct ArrayHasAll
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
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---

## ArrayHasAny

`struct` · `datafusion_functions_nested::array_has::ArrayHasAny`

```rust
struct ArrayHasAny
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
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
