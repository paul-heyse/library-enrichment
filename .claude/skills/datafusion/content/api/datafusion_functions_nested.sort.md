# `datafusion_functions_nested::sort`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.sort.json`](../model/datafusion_functions_nested.sort.json)

## array_sort

`function` · `datafusion_functions_nested::sort::array_sort`

Also reachable as `datafusion::prelude::array_sort`, `datafusion_functions_nested::expr_fn::array_sort`

```rust
fn array_sort(array: datafusion_expr::Expr, desc: datafusion_expr::Expr, null_first: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.sort.array_sort.md).


returns sorted array.

---

## array_sort_udf

`function` · `datafusion_functions_nested::sort::array_sort_udf`

```rust
fn array_sort_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.sort.array_sort_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArraySort

---

## ArraySort

`struct` · `datafusion_functions_nested::sort::ArraySort`

```rust
struct ArraySort
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.sort.ArraySort.md).


Implementation of `array_sort` function

`array_sort` sorts the elements of an array

# Example

`array_sort([3, 1, 2])` returns `[1, 2, 3]`

---
