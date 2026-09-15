# `datafusion_functions::datetime::to_char`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.datetime.to_char.json`](../model/datafusion_functions.datetime.to_char.json)

## ToCharFunc

`struct` · `datafusion_functions::datetime::to_char::ToCharFunc`

```rust
struct ToCharFunc
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
