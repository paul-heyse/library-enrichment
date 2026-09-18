# `datafusion_functions::unicode::rpad`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.unicode.rpad.json`](../model/datafusion_functions.unicode.rpad.json)

## RPadFunc

`struct` · `datafusion_functions::unicode::rpad::RPadFunc`

```rust
struct RPadFunc
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions.unicode.rpad.RPadFunc.md).


---
