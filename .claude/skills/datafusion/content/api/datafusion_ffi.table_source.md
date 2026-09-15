# `datafusion_ffi::table_source`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.table_source.json`](../model/datafusion_ffi.table_source.json)

## FFI_TableProviderFilterPushDown

`enum` · `datafusion_ffi::table_source::FFI_TableProviderFilterPushDown`

```rust
enum FFI_TableProviderFilterPushDown
```

**Variants**: `Unsupported`, `Inexact`, `Exact`

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(value: &TableProviderFilterPushDown) -> Self
```

FFI safe version of [`TableProviderFilterPushDown`].

---

## FFI_TableType

`enum` · `datafusion_ffi::table_source::FFI_TableType`

```rust
enum FFI_TableType
```

**Variants**: `Base`, `View`, `Temporary`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: TableType) -> Self
```

FFI safe version of [`TableType`].

---
