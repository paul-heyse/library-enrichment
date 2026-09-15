# `datafusion_ffi::ffi_option`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.ffi_option.json`](../model/datafusion_ffi.ffi_option.json)

## FFI_Option

`enum` · `datafusion_ffi::ffi_option::FFI_Option`

Also reachable as `datafusion_ffi::util::FFI_Option`

```rust
enum FFI_Option<T>
```

**Variants**: `Some`, `None`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn as_ref(&self) -> Option<&T>
fn into_option(self) -> Option<T>
fn map<U, F: FnOnce(T) -> U>(self, f: F) -> FFI_Option<U>
```

**via `core::convert::From`**

```rust
fn from(opt: Option<T>) -> Self
```

An FFI-safe option type.

---

## FFI_Result

`enum` · `datafusion_ffi::ffi_option::FFI_Result`

Also reachable as `datafusion_ffi::util::FFI_Result`

```rust
enum FFI_Result<T>
```

**Variants**: `Ok`, `Err`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, PartialEq

**Methods** (5)

```rust
fn into_result(self) -> Result<T, SString>
fn is_err(&self) -> bool
fn is_ok(&self) -> bool
fn map<U, F: FnOnce(T) -> U>(self, f: F) -> FFI_Result<U>
fn unwrap_err(self) -> SString
```

**via `core::convert::From`**

```rust
fn from(res: Result<T, E>) -> Self
```

An FFI-safe result type with SString as the error type.

---
