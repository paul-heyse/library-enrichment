# `datafusion_functions::core::getfield`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.getfield.json`](../model/datafusion_functions.core.getfield.json)

## GetFieldFunc

`struct` · `datafusion_functions::core::getfield::GetFieldFunc`

```rust
struct GetFieldFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn display_name(&self, args: &[Expr]) -> Result<String>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn placement(&self, args: &[ExpressionPlacement]) -> ExpressionPlacement
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn schema_name(&self, args: &[Expr]) -> Result<String>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, _info: &datafusion_expr::simplify::SimplifyContext) -> Result<ExprSimplifyResult>
```

---
