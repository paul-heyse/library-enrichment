# `datafusion_functions::core::try_cast_to_type`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.try_cast_to_type.json`](../model/datafusion_functions.core.try_cast_to_type.json)

## TryCastToTypeFunc

`struct` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc`

```rust
struct TryCastToTypeFunc
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions.core.try_cast_to_type.TryCastToTypeFunc.md).


Like [`cast_to_type`](super::cast_to_type::CastToTypeFunc) but returns NULL
on cast failure instead of erroring.

This is implemented by simplifying `try_cast_to_type(expr, ref)` into
`Expr::TryCast` during optimization.

---
