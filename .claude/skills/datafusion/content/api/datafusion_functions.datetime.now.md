# `datafusion_functions::datetime::now`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.datetime.now.json`](../model/datafusion_functions.datetime.now.json)

## NowFunc

`struct` · `datafusion_functions::datetime::now::NowFunc`

```rust
struct NowFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new() -> Self
fn new_with_config(config: &ConfigOptions) -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, _args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.now.NowFunc.md).


---
