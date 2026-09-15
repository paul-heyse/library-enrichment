# `datafusion_functions::core::greatest`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.greatest.json`](../model/datafusion_functions.core.greatest.json)

## GreatestFunc

`struct` · `datafusion_functions::core::greatest::GreatestFunc`

```rust
struct GreatestFunc
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
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
