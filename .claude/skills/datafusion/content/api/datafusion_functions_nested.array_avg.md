# `datafusion_functions_nested::array_avg`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_avg.json`](../model/datafusion_functions_nested.array_avg.json)

## array_avg

`function` · `datafusion_functions_nested::array_avg::array_avg`

Also reachable as `datafusion::prelude::array_avg`, `datafusion_functions_nested::expr_fn::array_avg`

```rust
fn array_avg(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_avg.array_avg.md).


returns the arithmetic mean of elements in a numeric array.

---

## array_avg_udf

`function` · `datafusion_functions_nested::array_avg::array_avg_udf`

```rust
fn array_avg_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_avg.array_avg_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayAvg

---

## ArrayAvg

`struct` · `datafusion_functions_nested::array_avg::ArrayAvg`

```rust
struct ArrayAvg
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_avg.ArrayAvg.md).


---
