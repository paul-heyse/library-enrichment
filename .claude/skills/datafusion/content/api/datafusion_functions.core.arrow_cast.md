# `datafusion_functions::core::arrow_cast`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.arrow_cast.json`](../model/datafusion_functions.core.arrow_cast.json)

## ArrowCastFunc

`struct` · `datafusion_functions::core::arrow_cast::ArrowCastFunc`

```rust
struct ArrowCastFunc
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
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implements casting to arbitrary arrow types (rather than SQL types)

Note that the `arrow_cast` function is somewhat special in that its
return depends only on the *value* of its second argument (not its type)

It is implemented by calling the same underlying arrow `cast` kernel as
normal SQL casts.

For example to cast to `int` using SQL  (which is then mapped to the arrow
type `Int32`)

```sql
select cast(column_x as int) ...
```

Use the `arrow_cast` function to cast to a specific arrow type

For example
```sql
select arrow_cast(column_x, 'Float64')
```

---
