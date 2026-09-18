# `datafusion_functions::core::cast_to_type`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.cast_to_type.json`](../model/datafusion_functions.core.cast_to_type.json)

## CastToTypeFunc

`struct` · `datafusion_functions::core::cast_to_type::CastToTypeFunc`

```rust
struct CastToTypeFunc
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions.core.cast_to_type.CastToTypeFunc.md).


Casts the first argument to the data type of the second argument.

Only the type of the second argument is used; its value is ignored.
This is useful in macros or generic SQL where you need to preserve
or match types dynamically.

For example:
```sql
select cast_to_type('42', NULL::INTEGER);
```

---
