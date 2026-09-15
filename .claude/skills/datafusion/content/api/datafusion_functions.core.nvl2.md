# `datafusion_functions::core::nvl2`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.nvl2.json`](../model/datafusion_functions.core.nvl2.json)

## NVL2Func

`struct` · `datafusion_functions::core::nvl2::NVL2Func`

```rust
struct NVL2Func
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
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn short_circuits(&self) -> bool
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

---
