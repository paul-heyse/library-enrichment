# `datafusion_functions_nested::set_ops`

Crate `datafusion-functions-nested` · 9 public items · structured records in [`model/datafusion_functions_nested.set_ops.json`](../model/datafusion_functions_nested.set_ops.json)

## array_distinct

`function` · `datafusion_functions_nested::set_ops::array_distinct`

Also reachable as `datafusion::prelude::array_distinct`, `datafusion_functions_nested::expr_fn::array_distinct`

```rust
fn array_distinct(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.array_distinct.md).


returns distinct values from the array after removing duplicates.

---

## array_distinct_udf

`function` · `datafusion_functions_nested::set_ops::array_distinct_udf`

```rust
fn array_distinct_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.array_distinct_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayDistinct

---

## array_intersect

`function` · `datafusion_functions_nested::set_ops::array_intersect`

Also reachable as `datafusion::prelude::array_intersect`, `datafusion_functions_nested::expr_fn::array_intersect`

```rust
fn array_intersect(first_array: datafusion_expr::Expr, second_array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.array_intersect.md).


returns an array of the elements in the intersection of array1 and array2.

---

## array_intersect_udf

`function` · `datafusion_functions_nested::set_ops::array_intersect_udf`

```rust
fn array_intersect_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.array_intersect_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayIntersect

---

## array_union

`function` · `datafusion_functions_nested::set_ops::array_union`

Also reachable as `datafusion::prelude::array_union`, `datafusion_functions_nested::expr_fn::array_union`

```rust
fn array_union(array1: datafusion_expr::Expr, array2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.array_union.md).


returns an array of the elements in the union of array1 and array2 without duplicates.

---

## array_union_udf

`function` · `datafusion_functions_nested::set_ops::array_union_udf`

```rust
fn array_union_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.array_union_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayUnion

---

## ArrayDistinct

`struct` · `datafusion_functions_nested::set_ops::ArrayDistinct`

```rust
struct ArrayDistinct
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.ArrayDistinct.md).


---

## ArrayIntersect

`struct` · `datafusion_functions_nested::set_ops::ArrayIntersect`

```rust
struct ArrayIntersect
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.ArrayIntersect.md).


---

## ArrayUnion

`struct` · `datafusion_functions_nested::set_ops::ArrayUnion`

```rust
struct ArrayUnion
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.set_ops.ArrayUnion.md).


---
