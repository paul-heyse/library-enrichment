# `datafusion_functions::core::arrow_try_cast`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.arrow_try_cast.json`](../model/datafusion_functions.core.arrow_try_cast.json)

## ArrowTryCastFunc

`struct` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc`

```rust
struct ArrowTryCastFunc
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
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.core.arrow_try_cast.ArrowTryCastFunc.md).


Like [`arrow_cast`](super::arrow_cast::ArrowCastFunc) but returns NULL on cast failure instead of erroring.

This is implemented by simplifying `arrow_try_cast(expr, 'Type')` into
`Expr::TryCast` during optimization.

---
