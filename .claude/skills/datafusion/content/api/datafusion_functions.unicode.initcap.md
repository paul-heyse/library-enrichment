# `datafusion_functions::unicode::initcap`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.unicode.initcap.json`](../model/datafusion_functions.unicode.initcap.json)

## InitcapFunc

`struct` · `datafusion_functions::unicode::initcap::InitcapFunc`

```rust
struct InitcapFunc
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
