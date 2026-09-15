# `datafusion_functions::regex::regexpcount`

Crate `datafusion-functions` · 2 public items · structured records in [`model/datafusion_functions.regex.regexpcount.json`](../model/datafusion_functions.regex.regexpcount.json)

## regexp_count_func

`function` · `datafusion_functions::regex::regexpcount::regexp_count_func`

```rust
fn regexp_count_func(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

---

## RegexpCountFunc

`struct` · `datafusion_functions::regex::regexpcount::RegexpCountFunc`

```rust
struct RegexpCountFunc
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

---
