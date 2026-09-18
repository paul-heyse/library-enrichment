# `datafusion_functions::regex::regexpinstr`

Crate `datafusion-functions` · 2 public items · structured records in [`model/datafusion_functions.regex.regexpinstr.json`](../model/datafusion_functions.regex.regexpinstr.json)

## regexp_instr_func

`function` · `datafusion_functions::regex::regexpinstr::regexp_instr_func`

```rust
fn regexp_instr_func(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.regexpinstr.regexp_instr_func.md).


---

## RegexpInstrFunc

`struct` · `datafusion_functions::regex::regexpinstr::RegexpInstrFunc`

```rust
struct RegexpInstrFunc
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.regexpinstr.RegexpInstrFunc.md).


---
