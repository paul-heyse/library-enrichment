# `datafusion_functions::core::version`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.version.json`](../model/datafusion_functions.core.version.json)

## VersionFunc

`struct` · `datafusion_functions::core::version::VersionFunc`

```rust
struct VersionFunc
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
fn return_type(&self, args: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
