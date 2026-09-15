# `datafusion_functions::core::named_struct`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.named_struct.json`](../model/datafusion_functions.core.named_struct.json)

## NamedStructFunc

`struct` · `datafusion_functions::core::named_struct::NamedStructFunc`

```rust
struct NamedStructFunc
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
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn struct_field_mapping(&self, literal_args: &[Option<ScalarValue>]) -> Option<StructFieldMapping>
```

---
