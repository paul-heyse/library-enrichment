# `datafusion_functions::math::nans`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.math.nans.json`](../model/datafusion_functions.math.nans.json)

## IsNanFunc

`struct` · `datafusion_functions::math::nans::IsNanFunc`

```rust
struct IsNanFunc
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
fn is_strict(&self) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.math.nans.IsNanFunc.md).


---
