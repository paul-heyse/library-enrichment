# `datafusion_ffi::udf`

Crate `datafusion-ffi` · 3 public items · structured records in [`model/datafusion_ffi.udf.json`](../model/datafusion_ffi.udf.json)

## FFI_ScalarUDF

`struct` · `datafusion_ffi::udf::FFI_ScalarUDF`

```rust
struct FFI_ScalarUDF
```

**Fields**: `name`, `aliases`, `volatility`, `return_field_from_args`, `invoke_with_args`, `short_circuits`, `coerce_types`, `placement`, `clone`, `release`, `private_data`, `library_marker_id`, `preserves_lex_ordering`, `with_updated_config`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**via `core::convert::From`**

```rust
fn from(udf: Arc<ScalarUDF>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing a [`ScalarUDF`] across FFI boundaries.

---

## ForeignScalarUDF

`struct` · `datafusion_ffi::udf::ForeignScalarUDF`

```rust
struct ForeignScalarUDF
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, Send, Sync

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn invoke_with_args(&self, invoke_args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn placement(&self, args: &[ExpressionPlacement]) -> ExpressionPlacement
fn preserves_lex_ordering(&self, inputs: &[ExprProperties]) -> Result<bool>
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn short_circuits(&self) -> bool
fn signature(&self) -> &Signature
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

This struct is used to access an UDF provided by a foreign
library across a FFI boundary.

The ForeignScalarUDF is to be used by the caller of the UDF, so it has
no knowledge or access to the private data. All interaction with the UDF
must occur through the functions defined in FFI_ScalarUDF.

---

## ScalarUDFPrivateData

`struct` · `datafusion_ffi::udf::ScalarUDFPrivateData`

```rust
struct ScalarUDFPrivateData
```

**Fields**: `udf`

---
