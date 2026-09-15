# `datafusion_ffi::udf::return_type_args`

Crate `datafusion-ffi` · 3 public items · structured records in [`model/datafusion_ffi.udf.return_type_args.json`](../model/datafusion_ffi.udf.return_type_args.json)

## FFI_ReturnFieldArgs

`struct` · `datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs`

```rust
struct FFI_ReturnFieldArgs
```

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: ReturnFieldArgs<'_>) -> Result<Self, Self::Error>
```

A stable struct for sharing a [`ReturnFieldArgs`] across FFI boundaries.

---

## ForeignReturnFieldArgs

`struct` · `datafusion_ffi::udf::return_type_args::ForeignReturnFieldArgs`

```rust
struct ForeignReturnFieldArgs<'a>
```

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(value: &'a ForeignReturnFieldArgsOwned) -> Self
```

---

## ForeignReturnFieldArgsOwned

`struct` · `datafusion_ffi::udf::return_type_args::ForeignReturnFieldArgsOwned`

```rust
struct ForeignReturnFieldArgsOwned
```

**Implements**: `core::convert::TryFrom`

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &FFI_ReturnFieldArgs) -> Result<Self, Self::Error>
```

---
