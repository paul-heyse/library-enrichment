# `datafusion_functions::core::file_row_index`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.file_row_index.json`](../model/datafusion_functions.core.file_row_index.json)

## FileRowIndexFunc

`struct` · `datafusion_functions::core::file_row_index::FileRowIndexFunc`

```rust
struct FileRowIndexFunc
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
fn placement(&self, _args: &[ExpressionPlacement]) -> ExpressionPlacement
fn return_type(&self, args: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Scalar UDF implementation for `file_row_index()`.

File sources that can expose per-file row indexes rewrite this placeholder
function into a source-provided physical expression. Direct evaluation
returns an error because there is no file context outside a scan.

---
