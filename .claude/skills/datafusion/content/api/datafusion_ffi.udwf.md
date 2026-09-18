# `datafusion_ffi::udwf`

Crate `datafusion-ffi` · 4 public items · structured records in [`model/datafusion_ffi.udwf.json`](../model/datafusion_ffi.udwf.json)

## FFI_SortOptions

`struct` · `datafusion_ffi::udwf::FFI_SortOptions`

```rust
struct FFI_SortOptions
```

**Fields**: `descending`, `nulls_first`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**via `core::convert::From`**

```rust
fn from(value: &SortOptions) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.udwf.FFI_SortOptions.md).


---

## FFI_WindowUDF

`struct` · `datafusion_ffi::udwf::FFI_WindowUDF`

```rust
struct FFI_WindowUDF
```

**Fields**: `name`, `aliases`, `volatility`, `partition_evaluator`, `field`, `coerce_types`, `sort_options`, `clone`, `release`, `private_data`, `library_marker_id`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**via `core::convert::From`**

```rust
fn from(udf: Arc<WindowUDF>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.udwf.FFI_WindowUDF.md).


A stable struct for sharing a [`WindowUDF`] across FFI boundaries.

---

## ForeignWindowUDF

`struct` · `datafusion_ffi::udwf::ForeignWindowUDF`

```rust
struct ForeignWindowUDF
```

**Implements**: `datafusion_expr::udwf::WindowUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, Send, Sync

**via `datafusion_expr::udwf::WindowUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
fn name(&self) -> &str
fn partition_evaluator(&self, args: datafusion_expr::function::PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn signature(&self) -> &Signature
fn sort_options(&self) -> Option<SortOptions>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.udwf.ForeignWindowUDF.md).


This struct is used to access an UDF provided by a foreign
library across a FFI boundary.

The ForeignWindowUDF is to be used by the caller of the UDF, so it has
no knowledge or access to the private data. All interaction with the UDF
must occur through the functions defined in FFI_WindowUDF.

---

## WindowUDFPrivateData

`struct` · `datafusion_ffi::udwf::WindowUDFPrivateData`

```rust
struct WindowUDFPrivateData
```

**Fields**: `udf`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.udwf.WindowUDFPrivateData.md).


---
