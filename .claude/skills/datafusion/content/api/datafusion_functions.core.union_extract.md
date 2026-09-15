# `datafusion_functions::core::union_extract`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.union_extract.json`](../model/datafusion_functions.core.union_extract.json)

## UnionExtractFun

`struct` · `datafusion_functions::core::union_extract::UnionExtractFun`

```rust
struct UnionExtractFun
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
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
