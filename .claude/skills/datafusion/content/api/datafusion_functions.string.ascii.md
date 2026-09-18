# `datafusion_functions::string::ascii`

Crate `datafusion-functions` · 2 public items · structured records in [`model/datafusion_functions.string.ascii.json`](../model/datafusion_functions.string.ascii.json)

## ascii

`function` · `datafusion_functions::string::ascii::ascii`

```rust
fn ascii(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.string.ascii.ascii.md).


Returns the numeric code of the first character of the argument.

---

## AsciiFunc

`struct` · `datafusion_functions::string::ascii::AsciiFunc`

```rust
struct AsciiFunc
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions.string.ascii.AsciiFunc.md).


---
