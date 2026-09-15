# `datafusion_ffi::expr::expr_properties`

Crate `datafusion-ffi` · 3 public items · structured records in [`model/datafusion_ffi.expr.expr_properties.json`](../model/datafusion_ffi.expr.expr_properties.json)

## FFI_SortProperties

`enum` · `datafusion_ffi::expr::expr_properties::FFI_SortProperties`

```rust
enum FFI_SortProperties
```

**Variants**: `Ordered`, `Unordered`, `Singleton`

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: &SortProperties) -> Self
```

---

## FFI_ExprProperties

`struct` · `datafusion_ffi::expr::expr_properties::FFI_ExprProperties`

```rust
struct FFI_ExprProperties
```

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &ExprProperties) -> Result<Self, Self::Error>
```

A stable struct for sharing [`ExprProperties`] across FFI boundaries.
See [`ExprProperties`] for the meaning of each field.

---

## FFI_SortOptions

`struct` · `datafusion_ffi::expr::expr_properties::FFI_SortOptions`

```rust
struct FFI_SortOptions
```

**Fields**: `descending`, `nulls_first`

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: &SortOptions) -> Self
```

---
