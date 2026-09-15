# `datafusion_functions::datetime::date_bin`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.datetime.date_bin.json`](../model/datafusion_functions.datetime.date_bin.json)

## DateBinFunc

`struct` · `datafusion_functions::datetime::date_bin::DateBinFunc`

```rust
struct DateBinFunc
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
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
