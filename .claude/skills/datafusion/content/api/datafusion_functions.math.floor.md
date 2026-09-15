# `datafusion_functions::math::floor`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.math.floor.json`](../model/datafusion_functions.math.floor.json)

## FloorFunc

`struct` · `datafusion_functions::math::floor::FloorFunc`

```rust
struct FloorFunc
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
fn evaluate_bounds(&self, inputs: &[&Interval]) -> Result<Interval>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn is_strict(&self) -> bool
fn name(&self) -> &str
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
fn preimage(&self, args: &[Expr], lit_expr: &Expr, _info: &SimplifyContext) -> Result<PreimageResult>
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
