# `datafusion_functions_nested::range`

Crate `datafusion-functions-nested` · 5 public items · structured records in [`model/datafusion_functions_nested.range.json`](../model/datafusion_functions_nested.range.json)

## gen_series

`function` · `datafusion_functions_nested::range::gen_series`

Also reachable as `datafusion::prelude::gen_series`, `datafusion_functions_nested::expr_fn::gen_series`

```rust
fn gen_series(start: datafusion_expr::Expr, stop: datafusion_expr::Expr, step: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.range.gen_series.md).


create a list of values in the range between start and stop, include upper bound

---

## gen_series_udf

`function` · `datafusion_functions_nested::range::gen_series_udf`

```rust
fn gen_series_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.range.gen_series_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
GenSeries

---

## range

`function` · `datafusion_functions_nested::range::range`

Also reachable as `datafusion::prelude::range`, `datafusion_functions_nested::expr_fn::range`

```rust
fn range(start: datafusion_expr::Expr, stop: datafusion_expr::Expr, step: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.range.range.md).


create a list of values in the range between start and stop

---

## range_udf

`function` · `datafusion_functions_nested::range::range_udf`

```rust
fn range_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.range.range_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
Range

---

## Range

`struct` · `datafusion_functions_nested::range::Range`

```rust
struct Range
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.range.Range.md).


---
