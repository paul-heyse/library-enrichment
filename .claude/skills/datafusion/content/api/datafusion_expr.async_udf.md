# `datafusion_expr::async_udf`

Crate `datafusion-expr` · 2 public items · structured records in [`model/datafusion_expr.async_udf.json`](../model/datafusion_expr.async_udf.json)

## AsyncScalarUDF

`struct` · `datafusion_expr::async_udf::AsyncScalarUDF`

```rust
struct AsyncScalarUDF
```

**Implements**: `core::fmt::Display`, `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn ideal_batch_size(&self) -> Option<usize>
fn into_scalar_udf(self) -> ScalarUDF
async fn invoke_async_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn new(inner: Arc<dyn AsyncScalarUDFImpl>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.async_udf.AsyncScalarUDF.md).


A scalar UDF that must be invoked using async methods

Note this is not meant to be used directly, but is meant to be an implementation detail
for AsyncUDFImpl.

---

## AsyncScalarUDFImpl

`trait` · `datafusion_expr::async_udf::AsyncScalarUDFImpl`

```rust
trait AsyncScalarUDFImpl: ScalarUDFImpl
```

**Methods** (2)

```rust
fn ideal_batch_size(&self) -> Option<usize>
async fn invoke_async_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.async_udf.AsyncScalarUDFImpl.md).


A scalar UDF that can invoke using async methods

Note this is less efficient than the ScalarUDFImpl, but it can be used
to register remote functions in the context.

The name is chosen to mirror ScalarUDFImpl

---
