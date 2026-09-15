# `datafusion_functions::math::gcd`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.math.gcd.json`](../model/datafusion_functions.math.gcd.json)

## GcdFunc

`struct` · `datafusion_functions::math::gcd::GcdFunc`

```rust
struct GcdFunc
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
fn is_strict(&self) -> bool
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
