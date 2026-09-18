# `datafusion_functions::regex::regexpmatch`

Crate `datafusion-functions` · 2 public items · structured records in [`model/datafusion_functions.regex.regexpmatch.json`](../model/datafusion_functions.regex.regexpmatch.json)

## regexp_match

`function` · `datafusion_functions::regex::regexpmatch::regexp_match`

```rust
fn regexp_match(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.regexpmatch.regexp_match.md).


---

## RegexpMatchFunc

`struct` · `datafusion_functions::regex::regexpmatch::RegexpMatchFunc`

```rust
struct RegexpMatchFunc
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.regexpmatch.RegexpMatchFunc.md).


---
