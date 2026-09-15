# `datafusion_functions::datetime::date_part`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.datetime.date_part.json`](../model/datafusion_functions.datetime.date_part.json)

## DatePartFunc

`struct` · `datafusion_functions::datetime::date_part::DatePartFunc`

```rust
struct DatePartFunc
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
fn preimage(&self, args: &[Expr], lit_expr: &Expr, info: &SimplifyContext) -> Result<PreimageResult>
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
