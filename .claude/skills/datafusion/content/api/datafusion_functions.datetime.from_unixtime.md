# `datafusion_functions::datetime::from_unixtime`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.datetime.from_unixtime.json`](../model/datafusion_functions.datetime.from_unixtime.json)

## FromUnixtimeFunc

`struct` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc`

```rust
struct FromUnixtimeFunc
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
fn output_ordering(&self, inputs: &[ExprProperties]) -> Result<SortProperties>
fn preserves_lex_ordering(&self, _inputs: &[ExprProperties]) -> Result<bool>
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn strictly_order_preserving(&self, _inputs: &[ExprProperties]) -> Result<bool>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.from_unixtime.FromUnixtimeFunc.md).


---
