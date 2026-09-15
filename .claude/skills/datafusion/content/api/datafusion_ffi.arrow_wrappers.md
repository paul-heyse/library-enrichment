# `datafusion_ffi::arrow_wrappers`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.arrow_wrappers.json`](../model/datafusion_ffi.arrow_wrappers.json)

## WrappedArray

`struct` · `datafusion_ffi::arrow_wrappers::WrappedArray`

```rust
struct WrappedArray
```

**Fields**: `array`, `schema`

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(array: &ArrayRef) -> Result<Self, Self::Error>
fn try_from(value: &ScalarValue) -> Result<Self, Self::Error>
```

This is a wrapper struct for FFI_ArrowArray to indicate
that the struct is FFI Safe. For convenience, we also include the
schema needed to create a record batch from the array.

---

## WrappedSchema

`struct` · `datafusion_ffi::arrow_wrappers::WrappedSchema`

```rust
struct WrappedSchema
```

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: SchemaRef) -> Self
```

This is a wrapper struct around FFI_ArrowSchema simply to indicate
that the underlying struct is FFI safe.

---
