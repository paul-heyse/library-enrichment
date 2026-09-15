# `datafusion_functions::crypto::sha`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.crypto.sha.json`](../model/datafusion_functions.crypto.sha.json)

## SHAFunc

`struct` · `datafusion_functions::crypto::sha::SHAFunc`

```rust
struct SHAFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn sha224() -> Self
fn sha256() -> Self
fn sha384() -> Self
fn sha512() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
