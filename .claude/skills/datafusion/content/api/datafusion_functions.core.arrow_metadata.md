# `datafusion_functions::core::arrow_metadata`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.arrow_metadata.json`](../model/datafusion_functions.core.arrow_metadata.json)

## ArrowMetadataFunc

`struct` · `datafusion_functions::core::arrow_metadata::ArrowMetadataFunc`

```rust
struct ArrowMetadataFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
