# `datafusion_functions::unicode::strpos`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.unicode.strpos.json`](../model/datafusion_functions.unicode.strpos.json)

## StrposFunc

`struct` · `datafusion_functions::unicode::strpos::StrposFunc`

```rust
struct StrposFunc
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
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
