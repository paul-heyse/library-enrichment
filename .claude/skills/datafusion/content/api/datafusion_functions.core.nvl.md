# `datafusion_functions::core::nvl`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.nvl.json`](../model/datafusion_functions.core.nvl.json)

## NVLFunc

`struct` · `datafusion_functions::core::nvl::NVLFunc`

```rust
struct NVLFunc
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
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn short_circuits(&self) -> bool
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.core.nvl.NVLFunc.md).


---
